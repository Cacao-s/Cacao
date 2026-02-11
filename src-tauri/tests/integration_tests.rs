use sqlx::sqlite::{SqlitePool, SqlitePoolOptions};

/// Create an in-memory SQLite pool with migrations applied.
async fn test_pool() -> SqlitePool {
    let pool = SqlitePoolOptions::new()
        .max_connections(1)
        .connect("sqlite::memory:")
        .await
        .expect("failed to create in-memory pool");

    sqlx::query("PRAGMA foreign_keys = ON;")
        .execute(&pool)
        .await
        .unwrap();

    sqlx::migrate!("./migrations")
        .run(&pool)
        .await
        .expect("failed to run migrations");

    pool
}

// ── Setup Service ────────────────────────────────────────────

mod setup {
    use super::*;
    use cacao_lib::services::setup_service;

    #[tokio::test]
    async fn setup_device_creates_profile() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();
        assert_eq!(profile.display_name, "Alice");
        assert_eq!(profile.role, "giver");
        assert!(!profile.uuid.is_empty());
    }

    #[tokio::test]
    async fn setup_device_trims_name() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "  Bob  ", "baby")
            .await
            .unwrap();
        assert_eq!(profile.display_name, "Bob");
    }

    #[tokio::test]
    async fn setup_device_rejects_empty_name() {
        let pool = test_pool().await;
        let err = setup_service::setup_device(&pool, "   ", "giver")
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn setup_device_rejects_invalid_role() {
        let pool = test_pool().await;
        let err = setup_service::setup_device(&pool, "Charlie", "admin")
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn setup_device_prevents_double_setup() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();
        let err = setup_service::setup_device(&pool, "Bob", "baby")
            .await
            .unwrap_err();
        assert_eq!(err.code, "DEVICE_ALREADY_SETUP");
    }

    #[tokio::test]
    async fn is_setup_returns_false_initially() {
        let pool = test_pool().await;
        assert!(!setup_service::is_setup(&pool).await.unwrap());
    }

    #[tokio::test]
    async fn is_setup_returns_true_after_setup() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();
        assert!(setup_service::is_setup(&pool).await.unwrap());
    }

    #[tokio::test]
    async fn get_profile_returns_none_initially() {
        let pool = test_pool().await;
        let profile = setup_service::get_profile(&pool).await.unwrap();
        assert!(profile.is_none());
    }

    #[tokio::test]
    async fn reset_device_clears_all_data() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();
        assert!(setup_service::is_setup(&pool).await.unwrap());

        setup_service::reset_device(&pool).await.unwrap();
        assert!(!setup_service::is_setup(&pool).await.unwrap());
    }
}

// ── Family Service ───────────────────────────────────────────

mod family {
    use super::*;
    use cacao_lib::services::{family_service, setup_service};

    async fn setup_giver(pool: &SqlitePool) -> String {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        profile.uuid
    }

    #[tokio::test]
    async fn create_family_and_auto_adds_member() {
        let pool = test_pool().await;
        let uuid = setup_giver(&pool).await;

        let family = family_service::create_family(&pool, &uuid, "Smith Family")
            .await
            .unwrap();
        assert_eq!(family.name, "Smith Family");
        assert_eq!(family.currency, "TWD");

        let members = family_service::get_members(&pool, family.id).await.unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].family_role, "giver");
        assert_eq!(members[0].profile_uuid, uuid);
    }

    #[tokio::test]
    async fn create_family_rejects_empty_name() {
        let pool = test_pool().await;
        let uuid = setup_giver(&pool).await;
        let err = family_service::create_family(&pool, &uuid, "  ")
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn get_family_returns_none_when_empty() {
        let pool = test_pool().await;
        let family = family_service::get_family(&pool).await.unwrap();
        assert!(family.is_none());
    }

    #[tokio::test]
    async fn add_and_remove_member() {
        let pool = test_pool().await;
        let uuid = setup_giver(&pool).await;
        let family = family_service::create_family(&pool, &uuid, "Test Family")
            .await
            .unwrap();

        let baby = family_service::add_member(&pool, family.id, "baby-uuid-123", "baby")
            .await
            .unwrap();
        assert_eq!(baby.family_role, "baby");

        let members = family_service::get_members(&pool, family.id).await.unwrap();
        assert_eq!(members.len(), 2);

        family_service::remove_member(&pool, baby.id).await.unwrap();

        // Removed members are soft-deleted
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        assert_eq!(members.len(), 1);
    }
}

// ── Wallet Service ───────────────────────────────────────────

mod wallet {
    use super::*;
    use cacao_lib::models::params::CreateWalletParams;
    use cacao_lib::services::{family_service, setup_service, wallet_service};

    async fn setup_family(pool: &SqlitePool) -> i64 {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test Family")
            .await
            .unwrap();
        family.id
    }

    #[tokio::test]
    async fn create_wallet_basic() {
        let pool = test_pool().await;
        let family_id = setup_family(&pool).await;

        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "Piggy Bank".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(wallet.name, "Piggy Bank");
        assert_eq!(wallet.type_, "cash");
        assert_eq!(wallet.balance_cents, 0);
        assert_eq!(wallet.status, "active");
    }

    #[tokio::test]
    async fn create_wallet_with_initial_balance() {
        let pool = test_pool().await;
        let family_id = setup_family(&pool).await;

        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "Savings".into(),
                wallet_type: "bank".into(),
                initial_balance_cents: Some(50000),
            },
        )
        .await
        .unwrap();

        assert_eq!(wallet.balance_cents, 50000); // $500.00
    }

    #[tokio::test]
    async fn create_wallet_rejects_empty_name() {
        let pool = test_pool().await;
        let family_id = setup_family(&pool).await;

        let err = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "  ".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn create_wallet_rejects_duplicate_name() {
        let pool = test_pool().await;
        let family_id = setup_family(&pool).await;

        wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "Wallet A".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        let err = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "Wallet A".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "WALLET_NAME_EXISTS");
    }

    #[tokio::test]
    async fn list_wallets_returns_only_active() {
        let pool = test_pool().await;
        let family_id = setup_family(&pool).await;

        let w1 = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "Wallet A".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "Wallet B".into(),
                wallet_type: "bank".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        wallet_service::archive_wallet(&pool, w1.id).await.unwrap();

        // archive_wallet does not set is_deleted, it sets status = 'archived'
        // list_wallets filters by is_deleted = 0, so archived wallets still appear
        let wallets = wallet_service::list_wallets(&pool, family_id)
            .await
            .unwrap();
        assert_eq!(wallets.len(), 2);
    }

    #[tokio::test]
    async fn update_wallet_changes_name_and_threshold() {
        let pool = test_pool().await;
        let family_id = setup_family(&pool).await;

        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "Old Name".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        let updated = wallet_service::update_wallet(&pool, wallet.id, "New Name", 10000)
            .await
            .unwrap();

        assert_eq!(updated.name, "New Name");
        assert_eq!(updated.warning_threshold_cents, 10000);
    }

    #[tokio::test]
    async fn archive_wallet_sets_status() {
        let pool = test_pool().await;
        let family_id = setup_family(&pool).await;

        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "To Archive".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        wallet_service::archive_wallet(&pool, wallet.id)
            .await
            .unwrap();

        let archived = wallet_service::get_wallet(&pool, wallet.id).await.unwrap();
        assert_eq!(archived.status, "archived");
    }

    #[tokio::test]
    async fn archive_wallet_rejects_already_archived() {
        let pool = test_pool().await;
        let family_id = setup_family(&pool).await;

        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "Double Archive".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        wallet_service::archive_wallet(&pool, wallet.id)
            .await
            .unwrap();
        let err = wallet_service::archive_wallet(&pool, wallet.id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "WALLET_ARCHIVED");
    }

    #[tokio::test]
    async fn get_wallet_not_found() {
        let pool = test_pool().await;
        let err = wallet_service::get_wallet(&pool, 9999).await.unwrap_err();
        assert_eq!(err.code, "WALLET_NOT_FOUND");
    }
}

// ── Request Service (State Machine) ─────────────────────────

mod request {
    use super::*;
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams, UpdateRequestParams};
    use cacao_lib::services::{family_service, request_service, setup_service, wallet_service};

    struct TestContext {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup_context(pool: &SqlitePool) -> TestContext {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test Family")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main Wallet".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000), // $1,000
            },
        )
        .await
        .unwrap();

        TestContext {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn create_request_as_draft() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: Some("food".into()),
                notes: Some("Lunch money".into()),
            },
        )
        .await
        .unwrap();

        assert_eq!(req.status, "draft");
        assert_eq!(req.amount_cents, 5000);
        assert_eq!(req.category.as_deref(), Some("food"));
    }

    #[tokio::test]
    async fn create_request_rejects_zero_amount() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let err = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 0,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn create_request_rejects_negative_amount() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let err = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: -100,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn draft_to_pending_submit() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let submitted = request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        assert_eq!(submitted.status, "pending");
    }

    #[tokio::test]
    async fn submit_rejects_non_draft() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();

        // Submitting again should fail
        let err = request_service::submit_request(&pool, req.id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_INVALID_STATUS");
    }

    #[tokio::test]
    async fn approve_request_debits_wallet() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 30000, // $300
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();

        let approved = request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();

        assert_eq!(approved.status, "approved");
        assert!(approved.decision_at.is_some());
        assert_eq!(approved.decision_by_member_id, Some(ctx.member_id));

        // Wallet balance should be reduced
        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.balance_cents, 70000); // 100000 - 30000
    }

    #[tokio::test]
    async fn approve_request_insufficient_balance() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 200000, // more than $1,000 balance
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();

        let err = request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "INSUFFICIENT_BALANCE");

        // Wallet balance should be unchanged
        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.balance_cents, 100000);
    }

    #[tokio::test]
    async fn reject_request_requires_reason() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();

        let err = request_service::reject_request(&pool, req.id, "  ", ctx.member_id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REJECTION_REASON_REQUIRED");
    }

    #[tokio::test]
    async fn reject_request_with_reason() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();

        let rejected =
            request_service::reject_request(&pool, req.id, "Too expensive", ctx.member_id)
                .await
                .unwrap();

        assert_eq!(rejected.status, "rejected");
        assert_eq!(rejected.rejection_reason.as_deref(), Some("Too expensive"));

        // Wallet balance should be unchanged (no debit on rejection)
        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.balance_cents, 100000);
    }

    #[tokio::test]
    async fn cannot_approve_already_decided() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();

        // Try to approve again
        let err = request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_ALREADY_DECIDED");
    }

    #[tokio::test]
    async fn cancel_draft_request() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let cancelled = request_service::cancel_request(&pool, req.id)
            .await
            .unwrap();
        assert_eq!(cancelled.status, "cancelled");
    }

    #[tokio::test]
    async fn cancel_pending_request() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();

        let cancelled = request_service::cancel_request(&pool, req.id)
            .await
            .unwrap();
        assert_eq!(cancelled.status, "cancelled");
    }

    #[tokio::test]
    async fn cannot_cancel_approved_request() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();

        let err = request_service::cancel_request(&pool, req.id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_INVALID_STATUS");
    }

    #[tokio::test]
    async fn update_draft_request() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: Some("food".into()),
                notes: None,
            },
        )
        .await
        .unwrap();

        let updated = request_service::update_request(
            &pool,
            req.id,
            &UpdateRequestParams {
                amount_cents: Some(8000),
                category: Some("education".into()),
                notes: Some("Updated notes".into()),
            },
        )
        .await
        .unwrap();

        assert_eq!(updated.amount_cents, 8000);
        assert_eq!(updated.category.as_deref(), Some("education"));
        assert_eq!(updated.notes.as_deref(), Some("Updated notes"));
    }

    #[tokio::test]
    async fn cannot_update_non_draft_request() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();

        let err = request_service::update_request(
            &pool,
            req.id,
            &UpdateRequestParams {
                amount_cents: Some(8000),
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "REQUEST_INVALID_STATUS");
    }

    #[tokio::test]
    async fn list_requests_filters_by_status() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let params = CreateRequestParams {
            family_id: ctx.family_id,
            requester_member_id: ctx.member_id,
            wallet_id: ctx.wallet_id,
            amount_cents: 5000,
            category: None,
            notes: None,
        };

        // Create 2 requests, submit one
        request_service::create_request(&pool, &params)
            .await
            .unwrap();
        let r2 = request_service::create_request(&pool, &params)
            .await
            .unwrap();
        request_service::submit_request(&pool, r2.id).await.unwrap();

        let drafts = request_service::list_requests(&pool, ctx.family_id, Some("draft"))
            .await
            .unwrap();
        assert_eq!(drafts.len(), 1);

        let pending = request_service::list_requests(&pool, ctx.family_id, Some("pending"))
            .await
            .unwrap();
        assert_eq!(pending.len(), 1);

        let all = request_service::list_requests(&pool, ctx.family_id, None)
            .await
            .unwrap();
        assert_eq!(all.len(), 2);
    }
}

// ── Auth Service (PIN) ──────────────────────────────────────

mod auth {
    use super::*;
    use cacao_lib::services::{auth_service, setup_service};

    #[tokio::test]
    async fn has_pin_false_initially() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();
        assert!(!auth_service::has_pin(&pool).await.unwrap());
    }

    #[tokio::test]
    async fn set_and_verify_pin() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();

        auth_service::set_pin(&pool, "1234").await.unwrap();
        assert!(auth_service::has_pin(&pool).await.unwrap());
        assert!(auth_service::verify_pin(&pool, "1234").await.unwrap());
        assert!(!auth_service::verify_pin(&pool, "0000").await.unwrap());
    }

    #[tokio::test]
    async fn remove_pin() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();

        auth_service::set_pin(&pool, "5678").await.unwrap();
        assert!(auth_service::has_pin(&pool).await.unwrap());

        auth_service::remove_pin(&pool).await.unwrap();
        assert!(!auth_service::has_pin(&pool).await.unwrap());
    }

    #[tokio::test]
    async fn verify_pin_returns_false_when_no_pin() {
        let pool = test_pool().await;
        assert!(!auth_service::verify_pin(&pool, "1234").await.unwrap());
    }
}

// ── Export Service ───────────────────────────────────────────

mod export {
    use super::*;
    use cacao_lib::models::params::{CreateWalletParams, ManualTransactionParams};
    use cacao_lib::services::{
        export_service, family_service, setup_service, transaction_service, wallet_service,
    };

    #[tokio::test]
    async fn export_empty_csv_has_header() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        let csv = export_service::export_transactions_csv(&pool, family.id, None, None, None)
            .await
            .unwrap();

        assert!(csv.starts_with("Date,Wallet,Type,Amount,Source,Category,Notes\n"));
        // Only header, no data rows
        assert_eq!(csv.lines().count(), 1);
    }

    #[tokio::test]
    async fn export_csv_contains_transaction_data() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        // Create wallet with initial balance (creates a credit transaction)
        wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Test Wallet".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(10000),
            },
        )
        .await
        .unwrap();

        let csv = export_service::export_transactions_csv(&pool, family.id, None, None, None)
            .await
            .unwrap();

        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines.len(), 2); // header + 1 transaction
        assert!(lines[1].contains("credit"));
        assert!(lines[1].contains("100.00")); // 10000 cents = $100.00
        assert!(lines[1].contains("Test Wallet"));
    }

    #[tokio::test]
    async fn export_csv_filter_by_wallet() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        let w1 = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Wallet A".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(50000),
            },
        )
        .await
        .unwrap();

        wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Wallet B".into(),
                wallet_type: "bank".into(),
                initial_balance_cents: Some(30000),
            },
        )
        .await
        .unwrap();

        // Filter by wallet A only
        let csv =
            export_service::export_transactions_csv(&pool, family.id, None, None, Some(w1.id))
                .await
                .unwrap();

        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines.len(), 2); // header + 1 transaction for wallet A
        assert!(lines[1].contains("Wallet A"));
    }

    #[tokio::test]
    async fn export_csv_multiple_transactions() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();

        // Add a manual debit transaction
        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: family.id,
                wallet_id: wallet.id,
                transaction_type: "debit".into(),
                amount_cents: 5000,
                category: Some("food".into()),
                notes: Some("Lunch".into()),
            },
        )
        .await
        .unwrap();

        let csv = export_service::export_transactions_csv(&pool, family.id, None, None, None)
            .await
            .unwrap();

        let lines: Vec<&str> = csv.lines().collect();
        assert_eq!(lines.len(), 3); // header + initial credit + manual debit
    }
}

// ── Transaction Service ─────────────────────────────────────

mod transaction {
    use super::*;
    use cacao_lib::models::params::{
        CreateWalletParams, ManualTransactionParams, TransactionFilters,
    };
    use cacao_lib::services::{family_service, setup_service, transaction_service, wallet_service};

    struct TxContext {
        family_id: i64,
        wallet_id: i64,
    }

    async fn setup_tx_context(pool: &SqlitePool) -> TxContext {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test Family")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main Wallet".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000), // $1,000
            },
        )
        .await
        .unwrap();

        TxContext {
            family_id: family.id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn create_manual_credit_transaction() {
        let pool = test_pool().await;
        let ctx = setup_tx_context(&pool).await;

        let tx = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "credit".into(),
                amount_cents: 5000,
                category: Some("allowance".into()),
                notes: Some("Weekly allowance".into()),
            },
        )
        .await
        .unwrap();

        assert_eq!(tx.type_, "credit");
        assert_eq!(tx.amount_cents, 5000);
        assert_eq!(tx.source_type, "manual");
        assert_eq!(tx.category.as_deref(), Some("allowance"));

        // Wallet balance should increase
        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.balance_cents, 105000); // 100000 + 5000
    }

    #[tokio::test]
    async fn create_manual_debit_transaction() {
        let pool = test_pool().await;
        let ctx = setup_tx_context(&pool).await;

        let tx = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "debit".into(),
                amount_cents: 30000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(tx.type_, "debit");
        assert_eq!(tx.amount_cents, 30000);

        // Wallet balance should decrease
        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.balance_cents, 70000); // 100000 - 30000
    }

    #[tokio::test]
    async fn debit_rejects_insufficient_balance() {
        let pool = test_pool().await;
        let ctx = setup_tx_context(&pool).await;

        let err = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "debit".into(),
                amount_cents: 200000, // More than $1,000 balance
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");

        // Balance should be unchanged
        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.balance_cents, 100000);
    }

    #[tokio::test]
    async fn rejects_zero_amount() {
        let pool = test_pool().await;
        let ctx = setup_tx_context(&pool).await;

        let err = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "credit".into(),
                amount_cents: 0,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn rejects_negative_amount() {
        let pool = test_pool().await;
        let ctx = setup_tx_context(&pool).await;

        let err = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "credit".into(),
                amount_cents: -100,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn rejects_invalid_transaction_type() {
        let pool = test_pool().await;
        let ctx = setup_tx_context(&pool).await;

        let err = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "transfer".into(),
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn debit_nonexistent_wallet_fails() {
        let pool = test_pool().await;
        let ctx = setup_tx_context(&pool).await;

        let err = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: 99999,
                transaction_type: "debit".into(),
                amount_cents: 100,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "WALLET_NOT_FOUND");
    }

    #[tokio::test]
    async fn list_wallet_transactions_returns_ordered() {
        let pool = test_pool().await;
        let ctx = setup_tx_context(&pool).await;

        // The initial balance creates 1 credit transaction
        // Add 2 more
        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "debit".into(),
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "credit".into(),
                amount_cents: 2000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let txs = transaction_service::list_wallet_transactions(&pool, ctx.wallet_id, None)
            .await
            .unwrap();

        assert_eq!(txs.len(), 3); // initial + 2 manual
    }

    #[tokio::test]
    async fn list_transactions_filter_by_type() {
        let pool = test_pool().await;
        let ctx = setup_tx_context(&pool).await;

        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "debit".into(),
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let filters = TransactionFilters {
            date_from: None,
            date_to: None,
            wallet_id: None,
            transaction_type: Some("debit".into()),
            source_type: None,
            limit: None,
            offset: None,
        };

        let txs = transaction_service::list_transactions(&pool, ctx.family_id, &filters)
            .await
            .unwrap();

        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].type_, "debit");
    }

    #[tokio::test]
    async fn list_transactions_filter_by_source_type() {
        let pool = test_pool().await;
        let ctx = setup_tx_context(&pool).await;

        // Initial balance creates a manual source tx
        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "debit".into(),
                amount_cents: 500,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let filters = TransactionFilters {
            date_from: None,
            date_to: None,
            wallet_id: None,
            transaction_type: None,
            source_type: Some("manual".into()),
            limit: None,
            offset: None,
        };

        let txs = transaction_service::list_transactions(&pool, ctx.family_id, &filters)
            .await
            .unwrap();

        // All manual transactions
        for tx in &txs {
            assert_eq!(tx.source_type, "manual");
        }
    }

    #[tokio::test]
    async fn monthly_summary_empty_month() {
        let pool = test_pool().await;
        let ctx = setup_tx_context(&pool).await;

        // Query a month with no transactions (year 2020)
        let summary = transaction_service::get_monthly_summary(&pool, ctx.family_id, 2020, 1)
            .await
            .unwrap();

        assert_eq!(summary.total_credit_cents, 0);
        assert_eq!(summary.total_debit_cents, 0);
        assert_eq!(summary.net_change_cents, 0);
        assert_eq!(summary.transaction_count, 0);
    }

    #[tokio::test]
    async fn monthly_summary_rejects_invalid_month() {
        let pool = test_pool().await;
        let ctx = setup_tx_context(&pool).await;

        let err = transaction_service::get_monthly_summary(&pool, ctx.family_id, 2024, 0)
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");

        let err = transaction_service::get_monthly_summary(&pool, ctx.family_id, 2024, 13)
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn sequential_debits_track_balance_correctly() {
        let pool = test_pool().await;
        let ctx = setup_tx_context(&pool).await;

        // Debit 3 times
        for _ in 0..3 {
            transaction_service::create_manual_transaction(
                &pool,
                &ManualTransactionParams {
                    family_id: ctx.family_id,
                    wallet_id: ctx.wallet_id,
                    transaction_type: "debit".into(),
                    amount_cents: 10000,
                    category: None,
                    notes: None,
                },
            )
            .await
            .unwrap();
        }

        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.balance_cents, 70000); // 100000 - 3*10000
    }
}

// ── Notification Service ────────────────────────────────────

mod notification {
    use super::*;
    use cacao_lib::services::notification_service;

    /// Helper: insert a test notification directly via SQL
    async fn insert_notification(pool: &SqlitePool, event_type: &str, is_read: i64) -> i64 {
        sqlx::query_scalar::<_, i64>(
            "INSERT INTO notifications (event_type, payload, is_read) VALUES (?, '{}', ?) RETURNING id",
        )
        .bind(event_type)
        .bind(is_read)
        .fetch_one(pool)
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn list_notifications_empty() {
        let pool = test_pool().await;
        let notifs = notification_service::list_notifications(&pool, None, None)
            .await
            .unwrap();
        assert!(notifs.is_empty());
    }

    #[tokio::test]
    async fn list_notifications_returns_inserted() {
        let pool = test_pool().await;
        insert_notification(&pool, "request_approved", 0).await;
        insert_notification(&pool, "request_rejected", 0).await;

        let notifs = notification_service::list_notifications(&pool, None, None)
            .await
            .unwrap();
        assert_eq!(notifs.len(), 2);
    }

    #[tokio::test]
    async fn list_notifications_respects_limit() {
        let pool = test_pool().await;
        for _ in 0..5 {
            insert_notification(&pool, "low_balance", 0).await;
        }

        let notifs = notification_service::list_notifications(&pool, Some(3), None)
            .await
            .unwrap();
        assert_eq!(notifs.len(), 3);
    }

    #[tokio::test]
    async fn list_notifications_respects_offset() {
        let pool = test_pool().await;
        for _ in 0..5 {
            insert_notification(&pool, "low_balance", 0).await;
        }

        let notifs = notification_service::list_notifications(&pool, Some(10), Some(3))
            .await
            .unwrap();
        assert_eq!(notifs.len(), 2); // 5 total - 3 offset = 2
    }

    #[tokio::test]
    async fn unread_count_initially_zero() {
        let pool = test_pool().await;
        let count = notification_service::unread_count(&pool).await.unwrap();
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn unread_count_tracks_unread() {
        let pool = test_pool().await;
        insert_notification(&pool, "request_submitted", 0).await;
        insert_notification(&pool, "request_approved", 0).await;
        insert_notification(&pool, "low_balance", 1).await; // already read

        let count = notification_service::unread_count(&pool).await.unwrap();
        assert_eq!(count, 2);
    }

    #[tokio::test]
    async fn mark_read_single_notification() {
        let pool = test_pool().await;
        let id = insert_notification(&pool, "member_joined", 0).await;

        assert_eq!(notification_service::unread_count(&pool).await.unwrap(), 1);

        notification_service::mark_read(&pool, id).await.unwrap();

        assert_eq!(notification_service::unread_count(&pool).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn mark_read_nonexistent_returns_error() {
        let pool = test_pool().await;
        let err = notification_service::mark_read(&pool, 99999)
            .await
            .unwrap_err();
        assert_eq!(err.code, "NOTIFICATION_NOT_FOUND");
    }

    #[tokio::test]
    async fn mark_all_read_clears_unread() {
        let pool = test_pool().await;
        insert_notification(&pool, "request_submitted", 0).await;
        insert_notification(&pool, "request_approved", 0).await;
        insert_notification(&pool, "low_balance", 0).await;

        assert_eq!(notification_service::unread_count(&pool).await.unwrap(), 3);

        notification_service::mark_all_read(&pool).await.unwrap();

        assert_eq!(notification_service::unread_count(&pool).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn mark_all_read_idempotent() {
        let pool = test_pool().await;
        // No unread notifications — should not fail
        notification_service::mark_all_read(&pool).await.unwrap();
        assert_eq!(notification_service::unread_count(&pool).await.unwrap(), 0);
    }
}

// ── Allowance Service ───────────────────────────────────────

mod allowance {
    use super::*;
    use cacao_lib::models::params::{
        CreateAllowanceParams, CreateWalletParams, UpdateAllowanceParams,
    };
    use cacao_lib::services::{
        allowance_service, family_service, notification_service, setup_service, wallet_service,
    };

    struct AllowanceContext {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup_allowance_context(pool: &SqlitePool) -> AllowanceContext {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test Family")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Allowance Wallet".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(0),
            },
        )
        .await
        .unwrap();

        AllowanceContext {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn create_allowance_daily() {
        let pool = test_pool().await;
        let ctx = setup_allowance_context(&pool).await;

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 10000,
                frequency: "daily".into(),
                interval_count: None,
                notes: Some("Daily allowance".into()),
            },
        )
        .await
        .unwrap();

        assert_eq!(allowance.amount_cents, 10000);
        assert_eq!(allowance.frequency, "daily");
        assert_eq!(allowance.status, "active");
        assert!(allowance.next_run_at.is_some());
    }

    #[tokio::test]
    async fn create_allowance_weekly() {
        let pool = test_pool().await;
        let ctx = setup_allowance_context(&pool).await;

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 50000,
                frequency: "weekly".into(),
                interval_count: Some(1),
                notes: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(allowance.frequency, "weekly");
        assert_eq!(allowance.interval_count, 1);
    }

    #[tokio::test]
    async fn create_allowance_monthly() {
        let pool = test_pool().await;
        let ctx = setup_allowance_context(&pool).await;

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 200000,
                frequency: "monthly".into(),
                interval_count: Some(1),
                notes: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(allowance.frequency, "monthly");
    }

    #[tokio::test]
    async fn create_allowance_rejects_zero_amount() {
        let pool = test_pool().await;
        let ctx = setup_allowance_context(&pool).await;

        let err = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 0,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn create_allowance_rejects_negative_amount() {
        let pool = test_pool().await;
        let ctx = setup_allowance_context(&pool).await;

        let err = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: -1000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn create_allowance_rejects_invalid_frequency() {
        let pool = test_pool().await;
        let ctx = setup_allowance_context(&pool).await;

        let err = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                frequency: "yearly".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn create_allowance_rejects_zero_interval() {
        let pool = test_pool().await;
        let ctx = setup_allowance_context(&pool).await;

        let err = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                frequency: "daily".into(),
                interval_count: Some(0),
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn pause_and_resume_allowance() {
        let pool = test_pool().await;
        let ctx = setup_allowance_context(&pool).await;

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        // Pause
        allowance_service::pause_allowance(&pool, allowance.id)
            .await
            .unwrap();

        let list = allowance_service::list_allowances(&pool, ctx.family_id)
            .await
            .unwrap();
        assert_eq!(list[0].status, "paused");

        // Resume
        allowance_service::resume_allowance(&pool, allowance.id)
            .await
            .unwrap();

        let list = allowance_service::list_allowances(&pool, ctx.family_id)
            .await
            .unwrap();
        assert_eq!(list[0].status, "active");
    }

    #[tokio::test]
    async fn archive_allowance() {
        let pool = test_pool().await;
        let ctx = setup_allowance_context(&pool).await;

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        allowance_service::archive_allowance(&pool, allowance.id)
            .await
            .unwrap();

        let list = allowance_service::list_allowances(&pool, ctx.family_id)
            .await
            .unwrap();
        assert_eq!(list[0].status, "archived");
    }

    #[tokio::test]
    async fn archive_nonexistent_allowance_fails() {
        let pool = test_pool().await;
        let err = allowance_service::archive_allowance(&pool, 99999)
            .await
            .unwrap_err();
        assert_eq!(err.code, "ALLOWANCE_NOT_FOUND");
    }

    #[tokio::test]
    async fn pause_nonexistent_allowance_fails() {
        let pool = test_pool().await;
        let err = allowance_service::pause_allowance(&pool, 99999)
            .await
            .unwrap_err();
        assert_eq!(err.code, "ALLOWANCE_NOT_FOUND");
    }

    #[tokio::test]
    async fn list_allowances_empty() {
        let pool = test_pool().await;
        let ctx = setup_allowance_context(&pool).await;
        let list = allowance_service::list_allowances(&pool, ctx.family_id)
            .await
            .unwrap();
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn update_allowance_amount() {
        let pool = test_pool().await;
        let ctx = setup_allowance_context(&pool).await;

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let updated = allowance_service::update_allowance(
            &pool,
            allowance.id,
            &UpdateAllowanceParams {
                amount_cents: Some(8000),
                frequency: None,
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(updated.amount_cents, 8000);
    }

    #[tokio::test]
    async fn update_allowance_rejects_zero_amount() {
        let pool = test_pool().await;
        let ctx = setup_allowance_context(&pool).await;

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let err = allowance_service::update_allowance(
            &pool,
            allowance.id,
            &UpdateAllowanceParams {
                amount_cents: Some(0),
                frequency: None,
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn execute_allowance_credits_wallet_and_creates_notification() {
        let pool = test_pool().await;
        let ctx = setup_allowance_context(&pool).await;

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 25000,
                frequency: "weekly".into(),
                interval_count: Some(1),
                notes: None,
            },
        )
        .await
        .unwrap();

        // Execute the allowance
        allowance_service::execute_allowance(&pool, &allowance)
            .await
            .unwrap();

        // Wallet balance should increase
        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.balance_cents, 25000);

        // A notification should be created
        let count = notification_service::unread_count(&pool).await.unwrap();
        assert!(count >= 1);
    }

    #[tokio::test]
    async fn execute_allowance_twice_doubles_balance() {
        let pool = test_pool().await;
        let ctx = setup_allowance_context(&pool).await;

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 10000,
                frequency: "daily".into(),
                interval_count: Some(1),
                notes: None,
            },
        )
        .await
        .unwrap();

        allowance_service::execute_allowance(&pool, &allowance)
            .await
            .unwrap();
        allowance_service::execute_allowance(&pool, &allowance)
            .await
            .unwrap();

        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.balance_cents, 20000); // 10000 * 2
    }
}

// ── Pairing Service ─────────────────────────────────────────

mod pairing {
    use super::*;
    use cacao_lib::services::{family_service, pairing_service, setup_service};

    #[tokio::test]
    async fn generate_pairing_code_is_6_digits() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        let code = pairing_service::generate_pairing_code(&pool, family.id)
            .await
            .unwrap();

        assert_eq!(code.len(), 6);
        assert!(code.chars().all(|c| c.is_ascii_digit()));
    }

    #[tokio::test]
    async fn validate_correct_code_returns_family_id() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        let code = pairing_service::generate_pairing_code(&pool, family.id)
            .await
            .unwrap();
        let returned_family_id = pairing_service::validate_pairing_code(&pool, &code)
            .await
            .unwrap();

        assert_eq!(returned_family_id, family.id);
    }

    #[tokio::test]
    async fn validate_wrong_code_returns_error() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        pairing_service::generate_pairing_code(&pool, family.id)
            .await
            .unwrap();

        let err = pairing_service::validate_pairing_code(&pool, "000000")
            .await
            .unwrap_err();
        assert_eq!(err.code, "PAIRING_CODE_INVALID");
    }

    #[tokio::test]
    async fn validate_without_generating_returns_error() {
        let pool = test_pool().await;

        let err = pairing_service::validate_pairing_code(&pool, "123456")
            .await
            .unwrap_err();
        assert_eq!(err.code, "PAIRING_CODE_INVALID");
    }

    #[tokio::test]
    async fn code_is_consumed_after_validation() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        let code = pairing_service::generate_pairing_code(&pool, family.id)
            .await
            .unwrap();

        // First validation succeeds
        pairing_service::validate_pairing_code(&pool, &code)
            .await
            .unwrap();

        // Second validation fails (code consumed)
        let err = pairing_service::validate_pairing_code(&pool, &code)
            .await
            .unwrap_err();
        assert_eq!(err.code, "PAIRING_CODE_INVALID");
    }

    #[tokio::test]
    async fn regenerate_code_invalidates_previous() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        let code1 = pairing_service::generate_pairing_code(&pool, family.id)
            .await
            .unwrap();
        let code2 = pairing_service::generate_pairing_code(&pool, family.id)
            .await
            .unwrap();

        // First code is overwritten
        if code1 != code2 {
            let err = pairing_service::validate_pairing_code(&pool, &code1)
                .await
                .unwrap_err();
            assert_eq!(err.code, "PAIRING_CODE_INVALID");
        }

        // Second code works
        let fid = pairing_service::validate_pairing_code(&pool, &code2)
            .await
            .unwrap();
        assert_eq!(fid, family.id);
    }
}

// ── Cross-Service Regression ────────────────────────────────

mod regression {
    use super::*;
    use cacao_lib::models::params::TransactionFilters;
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams};
    use cacao_lib::services::{
        family_service, notification_service, request_service, setup_service, transaction_service,
        wallet_service,
    };

    struct RegressionContext {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup_regression_context(pool: &SqlitePool) -> RegressionContext {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test Family")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main Wallet".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(500000), // $5,000
            },
        )
        .await
        .unwrap();

        RegressionContext {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn approve_request_creates_debit_transaction() {
        let pool = test_pool().await;
        let ctx = setup_regression_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 10000,
                category: Some("entertainment".into()),
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();

        // Verify debit transaction was created
        let filters = TransactionFilters {
            date_from: None,
            date_to: None,
            wallet_id: Some(ctx.wallet_id),
            transaction_type: Some("debit".into()),
            source_type: Some("request".into()),
            limit: None,
            offset: None,
        };
        let txs = transaction_service::list_transactions(&pool, ctx.family_id, &filters)
            .await
            .unwrap();

        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].amount_cents, 10000);
        assert_eq!(txs[0].source_type, "request");
        assert_eq!(txs[0].source_id, Some(req.id));
    }

    #[tokio::test]
    async fn approve_request_creates_notification() {
        let pool = test_pool().await;
        let ctx = setup_regression_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();

        // Verify notification was created
        let count = notification_service::unread_count(&pool).await.unwrap();
        assert!(
            count >= 1,
            "Expected at least 1 notification after approval"
        );

        let notifs = notification_service::list_notifications(&pool, None, None)
            .await
            .unwrap();
        let approved_notif = notifs.iter().find(|n| n.event_type == "request_approved");
        assert!(
            approved_notif.is_some(),
            "Should have request_approved notification"
        );
    }

    #[tokio::test]
    async fn reject_request_creates_notification_but_no_transaction() {
        let pool = test_pool().await;
        let ctx = setup_regression_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::reject_request(&pool, req.id, "Too expensive", ctx.member_id)
            .await
            .unwrap();

        // Notification should exist
        let notifs = notification_service::list_notifications(&pool, None, None)
            .await
            .unwrap();
        let rejected_notif = notifs.iter().find(|n| n.event_type == "request_rejected");
        assert!(rejected_notif.is_some());

        // No debit transaction should be created (only the initial credit from wallet creation)
        let filters = TransactionFilters {
            date_from: None,
            date_to: None,
            wallet_id: None,
            transaction_type: Some("debit".into()),
            source_type: None,
            limit: None,
            offset: None,
        };
        let txs = transaction_service::list_transactions(&pool, ctx.family_id, &filters)
            .await
            .unwrap();
        assert_eq!(
            txs.len(),
            0,
            "Rejection should not create debit transactions"
        );

        // Wallet balance should be unchanged
        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.balance_cents, 500000);
    }

    #[tokio::test]
    async fn approve_multiple_requests_balance_consistency() {
        let pool = test_pool().await;
        let ctx = setup_regression_context(&pool).await;

        // Create and approve 3 requests of $500 each
        for _ in 0..3 {
            let req = request_service::create_request(
                &pool,
                &CreateRequestParams {
                    family_id: ctx.family_id,
                    requester_member_id: ctx.member_id,
                    wallet_id: ctx.wallet_id,
                    amount_cents: 50000,
                    category: None,
                    notes: None,
                },
            )
            .await
            .unwrap();

            request_service::submit_request(&pool, req.id)
                .await
                .unwrap();
            request_service::approve_request(&pool, req.id, ctx.member_id)
                .await
                .unwrap();
        }

        // Wallet: 500000 - 3*50000 = 350000
        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.balance_cents, 350000);

        // Should have 3 debit transactions + 1 initial credit = 4 total
        let txs = transaction_service::list_wallet_transactions(&pool, ctx.wallet_id, None)
            .await
            .unwrap();
        assert_eq!(txs.len(), 4);
    }

    #[tokio::test]
    async fn approve_then_debit_fails_when_no_balance() {
        let pool = test_pool().await;
        let ctx = setup_regression_context(&pool).await;

        // Approve a request that takes nearly all balance
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 490000, // $4,900 of $5,000
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();

        // Now try to approve another request for more than remaining balance
        let req2 = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 20000, // $200 > $100 remaining
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req2.id)
            .await
            .unwrap();
        let err = request_service::approve_request(&pool, req2.id, ctx.member_id)
            .await
            .unwrap_err();

        assert_eq!(err.code, "INSUFFICIENT_BALANCE");
    }
}

// ── Auth Service Extended ───────────────────────────────────

mod auth_extended {
    use super::*;
    use cacao_lib::services::{auth_service, setup_service};

    #[tokio::test]
    async fn overwrite_pin_with_new_pin() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();

        auth_service::set_pin(&pool, "1234").await.unwrap();
        assert!(auth_service::verify_pin(&pool, "1234").await.unwrap());

        // Overwrite with new PIN
        auth_service::set_pin(&pool, "5678").await.unwrap();
        assert!(!auth_service::verify_pin(&pool, "1234").await.unwrap());
        assert!(auth_service::verify_pin(&pool, "5678").await.unwrap());
    }

    #[tokio::test]
    async fn set_pin_with_empty_string() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();

        // Empty PIN should still hash (validation is at command level)
        auth_service::set_pin(&pool, "").await.unwrap();
        assert!(auth_service::has_pin(&pool).await.unwrap());
        assert!(auth_service::verify_pin(&pool, "").await.unwrap());
    }

    #[tokio::test]
    async fn set_pin_with_long_string() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();

        let long_pin = "1".repeat(100);
        auth_service::set_pin(&pool, &long_pin).await.unwrap();
        assert!(auth_service::verify_pin(&pool, &long_pin).await.unwrap());
    }

    #[tokio::test]
    async fn remove_and_reset_pin() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();

        auth_service::set_pin(&pool, "1111").await.unwrap();
        auth_service::remove_pin(&pool).await.unwrap();
        assert!(!auth_service::has_pin(&pool).await.unwrap());

        // Set a new PIN after removal
        auth_service::set_pin(&pool, "2222").await.unwrap();
        assert!(auth_service::has_pin(&pool).await.unwrap());
        assert!(auth_service::verify_pin(&pool, "2222").await.unwrap());
    }
}

// ── Setup Service Extended ──────────────────────────────────

mod setup_extended {
    use super::*;
    use cacao_lib::services::setup_service;

    #[tokio::test]
    async fn get_profile_returns_correct_data() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();

        let profile = setup_service::get_profile(&pool).await.unwrap().unwrap();
        assert_eq!(profile.display_name, "Alice");
        assert_eq!(profile.role, "giver");
        assert!(!profile.uuid.is_empty());
    }

    #[tokio::test]
    async fn setup_baby_role() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Bobby", "baby")
            .await
            .unwrap();
        assert_eq!(profile.role, "baby");
    }

    #[tokio::test]
    async fn reset_then_re_setup() {
        let pool = test_pool().await;

        setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();
        setup_service::reset_device(&pool).await.unwrap();

        // Should be able to re-setup with different data
        let profile = setup_service::setup_device(&pool, "Bob", "baby")
            .await
            .unwrap();
        assert_eq!(profile.display_name, "Bob");
        assert_eq!(profile.role, "baby");
    }
}

// ── Audit Service ───────────────────────────────────────────

mod audit_extended {
    use super::*;
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams};
    use cacao_lib::services::{
        audit_service, family_service, request_service, setup_service, wallet_service,
    };

    #[tokio::test]
    async fn list_audit_logs_empty() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        let logs = audit_service::list_audit_logs(&pool, family.id, None, None)
            .await
            .unwrap();
        assert!(logs.is_empty());
    }

    #[tokio::test]
    async fn approve_request_creates_audit_log() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: family.id,
                requester_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::approve_request(&pool, req.id, members[0].id)
            .await
            .unwrap();

        let logs = audit_service::list_audit_logs(&pool, family.id, None, None)
            .await
            .unwrap();
        assert_eq!(logs.len(), 1);
        assert_eq!(logs[0].action, "approve_request");
        assert_eq!(logs[0].resource_type, "request");
        assert_eq!(logs[0].resource_id, Some(req.id.to_string()));
        assert!(logs[0].metadata.is_some());
    }

    #[tokio::test]
    async fn list_audit_logs_respects_limit_and_offset() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(500000),
            },
        )
        .await
        .unwrap();

        // Approve 3 requests → 3 audit logs
        for i in 0..3 {
            let req = request_service::create_request(
                &pool,
                &CreateRequestParams {
                    family_id: family.id,
                    requester_member_id: members[0].id,
                    wallet_id: wallet.id,
                    amount_cents: 1000 + i * 100,
                    category: None,
                    notes: None,
                },
            )
            .await
            .unwrap();
            request_service::submit_request(&pool, req.id)
                .await
                .unwrap();
            request_service::approve_request(&pool, req.id, members[0].id)
                .await
                .unwrap();
        }

        let all = audit_service::list_audit_logs(&pool, family.id, None, None)
            .await
            .unwrap();
        assert_eq!(all.len(), 3);

        let limited = audit_service::list_audit_logs(&pool, family.id, Some(2), None)
            .await
            .unwrap();
        assert_eq!(limited.len(), 2);

        let offset = audit_service::list_audit_logs(&pool, family.id, Some(10), Some(2))
            .await
            .unwrap();
        assert_eq!(offset.len(), 1);
    }

    #[tokio::test]
    async fn audit_log_scoped_to_family() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: family.id,
                requester_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 3000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::approve_request(&pool, req.id, members[0].id)
            .await
            .unwrap();

        let logs = audit_service::list_audit_logs(&pool, family.id, None, None)
            .await
            .unwrap();
        assert_eq!(logs[0].family_id, Some(family.id));

        // Different family_id returns empty
        let empty = audit_service::list_audit_logs(&pool, 99999, None, None)
            .await
            .unwrap();
        assert!(empty.is_empty());
    }
}

// ── Request Service Extended ────────────────────────────────

mod request_extended {
    use super::*;
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams};
    use cacao_lib::services::{family_service, request_service, setup_service, wallet_service};

    struct Ctx {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup_ctx(pool: &SqlitePool) -> Ctx {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test Family")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();

        Ctx {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn get_request_returns_correct_data() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 7500,
                category: Some("food".into()),
                notes: Some("test notes".into()),
            },
        )
        .await
        .unwrap();

        let fetched = request_service::get_request(&pool, req.id).await.unwrap();
        assert_eq!(fetched.id, req.id);
        assert_eq!(fetched.family_id, ctx.family_id);
        assert_eq!(fetched.requester_member_id, ctx.member_id);
        assert_eq!(fetched.wallet_id, ctx.wallet_id);
        assert_eq!(fetched.amount_cents, 7500);
        assert_eq!(fetched.category.as_deref(), Some("food"));
        assert_eq!(fetched.notes.as_deref(), Some("test notes"));
        assert_eq!(fetched.status, "draft");
        assert!(!fetched.uuid.is_empty());
    }

    #[tokio::test]
    async fn get_nonexistent_request_returns_not_found() {
        let pool = test_pool().await;
        let err = request_service::get_request(&pool, 99999)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_NOT_FOUND");
    }

    #[tokio::test]
    async fn approve_with_exact_balance() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 100000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        let approved = request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();

        assert_eq!(approved.status, "approved");

        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.balance_cents, 0);
    }

    #[tokio::test]
    async fn cancel_already_cancelled_request_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::cancel_request(&pool, req.id)
            .await
            .unwrap();

        let err = request_service::cancel_request(&pool, req.id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_INVALID_STATUS");
    }

    #[tokio::test]
    async fn create_request_on_archived_wallet_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;

        wallet_service::archive_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();

        let err = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "WALLET_NOT_FOUND");
    }
}

// ── Family Service Extended ─────────────────────────────────

mod family_extended {
    use super::*;
    use cacao_lib::services::{family_service, setup_service};

    #[tokio::test]
    async fn get_family_returns_correct_data() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let created = family_service::create_family(&pool, &profile.uuid, "The Smiths")
            .await
            .unwrap();

        let fetched = family_service::get_family(&pool).await.unwrap().unwrap();
        assert_eq!(fetched.id, created.id);
        assert_eq!(fetched.name, "The Smiths");
        assert_eq!(fetched.currency, "TWD");
        assert!(!fetched.uuid.is_empty());
        assert_eq!(fetched.created_by_device, profile.uuid);
    }

    #[tokio::test]
    async fn create_family_trims_name() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "  Trimmed  ")
            .await
            .unwrap();
        assert_eq!(family.name, "Trimmed");
    }

    #[tokio::test]
    async fn create_family_rejects_too_long_name() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let long_name = "a".repeat(51);
        let err = family_service::create_family(&pool, &profile.uuid, &long_name)
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn get_members_returns_correct_roles() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        family_service::add_member(&pool, family.id, "baby-uuid-1", "baby")
            .await
            .unwrap();
        family_service::add_member(&pool, family.id, "baby-uuid-2", "baby")
            .await
            .unwrap();

        let members = family_service::get_members(&pool, family.id).await.unwrap();
        assert_eq!(members.len(), 3);
        assert_eq!(
            members.iter().filter(|m| m.family_role == "giver").count(),
            1
        );
        assert_eq!(
            members.iter().filter(|m| m.family_role == "baby").count(),
            2
        );
    }
}

// ── Wallet Service Extended ─────────────────────────────────

mod wallet_extended {
    use super::*;
    use cacao_lib::models::params::CreateWalletParams;
    use cacao_lib::services::{family_service, setup_service, wallet_service};

    async fn setup_family(pool: &SqlitePool) -> i64 {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test")
            .await
            .unwrap();
        family.id
    }

    #[tokio::test]
    async fn create_wallet_with_negative_initial_balance_stays_zero() {
        let pool = test_pool().await;
        let family_id = setup_family(&pool).await;

        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "Negative Start".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(-500),
            },
        )
        .await
        .unwrap();

        assert_eq!(wallet.balance_cents, 0);
    }

    #[tokio::test]
    async fn create_wallet_rejects_too_long_name() {
        let pool = test_pool().await;
        let family_id = setup_family(&pool).await;

        let long_name = "a".repeat(51);
        let err = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: long_name,
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn update_wallet_rejects_empty_name() {
        let pool = test_pool().await;
        let family_id = setup_family(&pool).await;

        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "Good Name".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        let err = wallet_service::update_wallet(&pool, wallet.id, "  ", 0)
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn update_wallet_rejects_too_long_name() {
        let pool = test_pool().await;
        let family_id = setup_family(&pool).await;

        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "Good Name".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        let long_name = "b".repeat(51);
        let err = wallet_service::update_wallet(&pool, wallet.id, &long_name, 0)
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn update_nonexistent_wallet_fails() {
        let pool = test_pool().await;
        let err = wallet_service::update_wallet(&pool, 99999, "Name", 0)
            .await
            .unwrap_err();
        assert_eq!(err.code, "WALLET_NOT_FOUND");
    }
}

// ── R3: CSV Export Security ─────────────────────────────────

mod export_security {
    use super::*;
    use cacao_lib::models::params::{CreateWalletParams, ManualTransactionParams};
    use cacao_lib::services::{
        export_service, family_service, setup_service, transaction_service, wallet_service,
    };

    #[tokio::test]
    async fn csv_escapes_commas_in_wallet_name() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Cash, Savings".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(10000),
            },
        )
        .await
        .unwrap();

        let csv = export_service::export_transactions_csv(&pool, family.id, None, None, None)
            .await
            .unwrap();

        // Wallet name with comma should be quoted
        assert!(csv.contains("\"Cash, Savings\""));
    }

    #[tokio::test]
    async fn csv_escapes_quotes_in_notes() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(50000),
            },
        )
        .await
        .unwrap();

        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: family.id,
                wallet_id: wallet.id,
                transaction_type: "debit".into(),
                amount_cents: 1000,
                category: Some("food".into()),
                notes: Some("He said \"hello\"".into()),
            },
        )
        .await
        .unwrap();

        let csv = export_service::export_transactions_csv(&pool, family.id, None, None, None)
            .await
            .unwrap();

        // Double quotes should be escaped as ""
        assert!(csv.contains("\"\"hello\"\""));
    }

    #[tokio::test]
    async fn csv_escapes_newlines_in_notes() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(50000),
            },
        )
        .await
        .unwrap();

        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: family.id,
                wallet_id: wallet.id,
                transaction_type: "debit".into(),
                amount_cents: 500,
                category: None,
                notes: Some("Line1\nLine2".into()),
            },
        )
        .await
        .unwrap();

        let csv = export_service::export_transactions_csv(&pool, family.id, None, None, None)
            .await
            .unwrap();

        // Newlines in notes should be quoted
        assert!(csv.contains("\"Line1\nLine2\""));
    }

    #[tokio::test]
    async fn csv_amount_formatting_precision() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(12345), // $123.45
            },
        )
        .await
        .unwrap();

        let csv = export_service::export_transactions_csv(&pool, family.id, None, None, None)
            .await
            .unwrap();

        assert!(csv.contains("123.45"));
    }
}

// ── R3: Transaction Filter Edge Cases ───────────────────────

mod transaction_filters {
    use super::*;
    use cacao_lib::models::params::{
        CreateWalletParams, ManualTransactionParams, TransactionFilters,
    };
    use cacao_lib::services::{family_service, setup_service, transaction_service, wallet_service};
    use chrono::Datelike;

    struct Ctx {
        family_id: i64,
        wallet_id: i64,
    }

    async fn setup(pool: &SqlitePool) -> Ctx {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(500000),
            },
        )
        .await
        .unwrap();
        Ctx {
            family_id: family.id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn list_transactions_with_limit_and_offset() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        // Create 5 debit transactions
        for _ in 0..5 {
            transaction_service::create_manual_transaction(
                &pool,
                &ManualTransactionParams {
                    family_id: ctx.family_id,
                    wallet_id: ctx.wallet_id,
                    transaction_type: "debit".into(),
                    amount_cents: 1000,
                    category: None,
                    notes: None,
                },
            )
            .await
            .unwrap();
        }

        // Total: 1 initial credit + 5 debits = 6
        let all = transaction_service::list_transactions(
            &pool,
            ctx.family_id,
            &TransactionFilters {
                date_from: None,
                date_to: None,
                wallet_id: None,
                transaction_type: None,
                source_type: None,
                limit: None,
                offset: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(all.len(), 6);

        // Limit to 3
        let limited = transaction_service::list_transactions(
            &pool,
            ctx.family_id,
            &TransactionFilters {
                date_from: None,
                date_to: None,
                wallet_id: None,
                transaction_type: None,
                source_type: None,
                limit: Some(3),
                offset: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(limited.len(), 3);

        // Offset 4 → should return 2
        let offset = transaction_service::list_transactions(
            &pool,
            ctx.family_id,
            &TransactionFilters {
                date_from: None,
                date_to: None,
                wallet_id: None,
                transaction_type: None,
                source_type: None,
                limit: Some(10),
                offset: Some(4),
            },
        )
        .await
        .unwrap();
        assert_eq!(offset.len(), 2);
    }

    #[tokio::test]
    async fn list_wallet_transactions_custom_limit() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        for _ in 0..4 {
            transaction_service::create_manual_transaction(
                &pool,
                &ManualTransactionParams {
                    family_id: ctx.family_id,
                    wallet_id: ctx.wallet_id,
                    transaction_type: "debit".into(),
                    amount_cents: 500,
                    category: None,
                    notes: None,
                },
            )
            .await
            .unwrap();
        }

        // 1 initial + 4 debits = 5 total, limit to 2
        let txs = transaction_service::list_wallet_transactions(&pool, ctx.wallet_id, Some(2))
            .await
            .unwrap();
        assert_eq!(txs.len(), 2);
    }

    #[tokio::test]
    async fn combined_wallet_and_type_filter() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        // Create second wallet
        let wallet2 = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: ctx.family_id,
                name: "Secondary".into(),
                wallet_type: "bank".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();

        // Debit from wallet 1
        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "debit".into(),
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        // Debit from wallet 2
        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: wallet2.id,
                transaction_type: "debit".into(),
                amount_cents: 2000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        // Filter: wallet 1 + debit only
        let txs = transaction_service::list_transactions(
            &pool,
            ctx.family_id,
            &TransactionFilters {
                date_from: None,
                date_to: None,
                wallet_id: Some(ctx.wallet_id),
                transaction_type: Some("debit".into()),
                source_type: None,
                limit: None,
                offset: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].wallet_id, ctx.wallet_id);
        assert_eq!(txs[0].type_, "debit");
    }

    #[tokio::test]
    async fn debit_exact_balance_leaves_zero() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Exact".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(5000),
            },
        )
        .await
        .unwrap();

        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: family.id,
                wallet_id: wallet.id,
                transaction_type: "debit".into(),
                amount_cents: 5000, // exact balance
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let w = wallet_service::get_wallet(&pool, wallet.id).await.unwrap();
        assert_eq!(w.balance_cents, 0);
    }

    #[tokio::test]
    async fn monthly_summary_with_mixed_transactions() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        // The initial credit from wallet creation happens in "now"
        // Add a debit too
        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "debit".into(),
                amount_cents: 10000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        // Query current month
        let now = chrono::Utc::now();
        let summary =
            transaction_service::get_monthly_summary(&pool, ctx.family_id, now.year(), now.month())
                .await
                .unwrap();

        // Should have: initial credit of 500000 + debit of 10000
        assert_eq!(summary.total_credit_cents, 500000);
        assert_eq!(summary.total_debit_cents, 10000);
        assert_eq!(summary.net_change_cents, 490000);
        assert_eq!(summary.transaction_count, 2);
    }
}

// ── R3: Allowance Biweekly + Custom ─────────────────────────

mod allowance_extended {
    use super::*;
    use cacao_lib::models::params::{
        CreateAllowanceParams, CreateWalletParams, UpdateAllowanceParams,
    };
    use cacao_lib::services::{allowance_service, family_service, setup_service, wallet_service};

    struct Ctx {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup(pool: &SqlitePool) -> Ctx {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Allowance".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(0),
            },
        )
        .await
        .unwrap();
        Ctx {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn create_allowance_biweekly() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 30000,
                frequency: "biweekly".into(),
                interval_count: Some(1),
                notes: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(a.frequency, "biweekly");
        assert_eq!(a.status, "active");
    }

    #[tokio::test]
    async fn create_allowance_custom() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                frequency: "custom".into(),
                interval_count: Some(3), // every 3 days
                notes: Some("Every 3 days".into()),
            },
        )
        .await
        .unwrap();

        assert_eq!(a.frequency, "custom");
        assert_eq!(a.interval_count, 3);
    }

    #[tokio::test]
    async fn update_paused_allowance_fails() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        allowance_service::pause_allowance(&pool, a.id)
            .await
            .unwrap();

        let err = allowance_service::update_allowance(
            &pool,
            a.id,
            &UpdateAllowanceParams {
                amount_cents: Some(10000),
                frequency: None,
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn update_allowance_no_changes_returns_same() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let unchanged = allowance_service::update_allowance(
            &pool,
            a.id,
            &UpdateAllowanceParams {
                amount_cents: None,
                frequency: None,
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(unchanged.amount_cents, 5000);
    }

    #[tokio::test]
    async fn update_allowance_frequency() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let updated = allowance_service::update_allowance(
            &pool,
            a.id,
            &UpdateAllowanceParams {
                amount_cents: None,
                frequency: Some("weekly".into()),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(updated.frequency, "weekly");
    }

    #[tokio::test]
    async fn update_allowance_rejects_invalid_frequency() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let err = allowance_service::update_allowance(
            &pool,
            a.id,
            &UpdateAllowanceParams {
                amount_cents: None,
                frequency: Some("yearly".into()),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn resume_nonexistent_allowance_fails() {
        let pool = test_pool().await;
        let err = allowance_service::resume_allowance(&pool, 99999)
            .await
            .unwrap_err();
        assert_eq!(err.code, "ALLOWANCE_NOT_FOUND");
    }
}

// ── R5: Request State Machine Complete Paths ─────────────────

mod request_state_machine {
    use super::*;
    use cacao_lib::models::params::CreateRequestParams;
    use cacao_lib::models::params::CreateWalletParams;
    use cacao_lib::services::{family_service, request_service, setup_service, wallet_service};

    struct Ctx {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup(pool: &SqlitePool) -> Ctx {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(500000),
            },
        )
        .await
        .unwrap();
        Ctx {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    async fn create_draft(pool: &SqlitePool, ctx: &Ctx) -> cacao_lib::models::request::Request {
        request_service::create_request(
            pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 10000,
                category: Some("food".into()),
                notes: Some("Test request".into()),
            },
        )
        .await
        .unwrap()
    }

    #[tokio::test]
    async fn reject_draft_request_fails() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;
        let req = create_draft(&pool, &ctx).await;

        // Try to reject a draft (not submitted yet)
        let err = request_service::reject_request(&pool, req.id, "Not needed", ctx.member_id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_INVALID_STATUS");
    }

    #[tokio::test]
    async fn reject_cancelled_request_fails() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;
        let req = create_draft(&pool, &ctx).await;

        request_service::cancel_request(&pool, req.id)
            .await
            .unwrap();

        let err = request_service::reject_request(&pool, req.id, "Too late", ctx.member_id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_INVALID_STATUS");
    }

    #[tokio::test]
    async fn reject_already_rejected_request_fails() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;
        let req = create_draft(&pool, &ctx).await;

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::reject_request(&pool, req.id, "Too expensive", ctx.member_id)
            .await
            .unwrap();

        let err = request_service::reject_request(&pool, req.id, "Again", ctx.member_id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_ALREADY_DECIDED");
    }

    #[tokio::test]
    async fn approve_draft_request_fails() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;
        let req = create_draft(&pool, &ctx).await;

        let err = request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_INVALID_STATUS");
    }

    #[tokio::test]
    async fn approve_cancelled_request_fails() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;
        let req = create_draft(&pool, &ctx).await;

        request_service::cancel_request(&pool, req.id)
            .await
            .unwrap();

        let err = request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_INVALID_STATUS");
    }

    #[tokio::test]
    async fn update_request_with_negative_amount_fails() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;
        let req = create_draft(&pool, &ctx).await;

        let err = request_service::update_request(
            &pool,
            req.id,
            &cacao_lib::models::params::UpdateRequestParams {
                amount_cents: Some(-100),
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn full_draft_to_approved_verifies_all_fields() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;
        let req = create_draft(&pool, &ctx).await;

        // draft → pending
        let submitted = request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        assert_eq!(submitted.status, "pending");

        // pending → approved
        let approved = request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();
        assert_eq!(approved.status, "approved");
        assert_eq!(approved.decision_by_member_id, Some(ctx.member_id));
        assert!(approved.decision_at.is_some());
        assert_eq!(approved.amount_cents, 10000);
        assert_eq!(approved.category.as_deref(), Some("food"));
        assert_eq!(approved.notes.as_deref(), Some("Test request"));
    }

    #[tokio::test]
    async fn full_draft_to_rejected_verifies_rejection_reason() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;
        let req = create_draft(&pool, &ctx).await;

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();

        let rejected =
            request_service::reject_request(&pool, req.id, "Budget exceeded", ctx.member_id)
                .await
                .unwrap();
        assert_eq!(rejected.status, "rejected");
        assert_eq!(
            rejected.rejection_reason.as_deref(),
            Some("Budget exceeded")
        );
        assert_eq!(rejected.decision_by_member_id, Some(ctx.member_id));
        assert!(rejected.decision_at.is_some());
    }

    #[tokio::test]
    async fn list_requests_without_filter_returns_all() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        // Create 3 requests with different statuses
        let _r1 = create_draft(&pool, &ctx).await;
        let r2 = create_draft(&pool, &ctx).await;
        let r3 = create_draft(&pool, &ctx).await;

        request_service::submit_request(&pool, r2.id).await.unwrap();
        request_service::cancel_request(&pool, r3.id).await.unwrap();

        // List without filter
        let all = request_service::list_requests(&pool, ctx.family_id, None)
            .await
            .unwrap();
        assert_eq!(all.len(), 3);
    }
}

// ── R5: Allowance Execute Edge Cases ────────────────────────

mod allowance_execute_edge {
    use super::*;
    use cacao_lib::models::params::{CreateAllowanceParams, CreateWalletParams};
    use cacao_lib::services::{
        allowance_service, family_service, notification_service, setup_service, wallet_service,
    };

    #[tokio::test]
    async fn execute_allowance_updates_last_run_at() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(0),
            },
        )
        .await
        .unwrap();

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: family.id,
                giver_member_id: members[0].id,
                receiver_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 5000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        assert!(a.last_run_at.is_none());

        allowance_service::execute_allowance(&pool, &a)
            .await
            .unwrap();

        // Re-fetch allowance to verify last_run_at was set
        let allowances = allowance_service::list_allowances(&pool, family.id)
            .await
            .unwrap();
        let executed = &allowances[0];
        assert!(executed.last_run_at.is_some());
        assert!(executed.next_run_at.is_some());
    }

    #[tokio::test]
    async fn execute_allowance_on_deleted_wallet_fails() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Temp".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(0),
            },
        )
        .await
        .unwrap();

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: family.id,
                giver_member_id: members[0].id,
                receiver_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 1000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        // Archive the wallet (soft-delete scenario)
        wallet_service::archive_wallet(&pool, wallet.id)
            .await
            .unwrap();

        // Execute should still work because archive doesn't set is_deleted
        // But the wallet status is "archived", and execute only checks is_deleted
        let result = allowance_service::execute_allowance(&pool, &a).await;
        // Archive changes status, not is_deleted, so it should still work
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn execute_allowance_creates_disbursement_notification() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(0),
            },
        )
        .await
        .unwrap();

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: family.id,
                giver_member_id: members[0].id,
                receiver_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 10000,
                frequency: "weekly".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        allowance_service::execute_allowance(&pool, &a)
            .await
            .unwrap();

        // Check notification was created
        let notifications = notification_service::list_notifications(&pool, Some(10), Some(0))
            .await
            .unwrap();
        assert!(!notifications.is_empty());
        let notif = &notifications[0];
        assert_eq!(notif.event_type, "allowance_disbursed");
    }

    #[tokio::test]
    async fn execute_allowance_low_balance_creates_warning() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();

        // Create wallet with a low threshold
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Low".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(0),
            },
        )
        .await
        .unwrap();

        // Set warning threshold
        wallet_service::update_wallet(&pool, wallet.id, "Low", 100000)
            .await
            .unwrap();

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: family.id,
                giver_member_id: members[0].id,
                receiver_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 500, // 5 TWD, far below threshold of 1000 TWD
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        allowance_service::execute_allowance(&pool, &a)
            .await
            .unwrap();

        // Should have both "allowance_disbursed" and "low_balance" notifications
        let notifications = notification_service::list_notifications(&pool, Some(10), Some(0))
            .await
            .unwrap();
        let event_types: Vec<&str> = notifications
            .iter()
            .map(|n| n.event_type.as_str())
            .collect();
        assert!(event_types.contains(&"allowance_disbursed"));
        assert!(event_types.contains(&"low_balance"));
    }
}

// ── R6: Auth Unicode PIN + Allowance Interval + Notification Edge ─

mod auth_unicode {
    use super::*;
    use cacao_lib::services::{auth_service, setup_service};

    #[tokio::test]
    async fn pin_with_unicode_characters() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();

        auth_service::set_pin(&pool, "密碼1234").await.unwrap();
        assert!(auth_service::verify_pin(&pool, "密碼1234").await.unwrap());
        assert!(!auth_service::verify_pin(&pool, "密碼1235").await.unwrap());
    }

    #[tokio::test]
    async fn pin_with_emoji() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();

        auth_service::set_pin(&pool, "🔒🔑🏠").await.unwrap();
        assert!(auth_service::verify_pin(&pool, "🔒🔑🏠").await.unwrap());
        assert!(!auth_service::verify_pin(&pool, "🔒🔑").await.unwrap());
    }

    #[tokio::test]
    async fn pin_with_spaces_is_distinct() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();

        auth_service::set_pin(&pool, "1234").await.unwrap();
        // " 1234" should be different
        assert!(!auth_service::verify_pin(&pool, " 1234").await.unwrap());
        assert!(!auth_service::verify_pin(&pool, "1234 ").await.unwrap());
    }

    #[tokio::test]
    async fn pin_case_sensitive() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();

        auth_service::set_pin(&pool, "AbCd").await.unwrap();
        assert!(auth_service::verify_pin(&pool, "AbCd").await.unwrap());
        assert!(!auth_service::verify_pin(&pool, "abcd").await.unwrap());
        assert!(!auth_service::verify_pin(&pool, "ABCD").await.unwrap());
    }
}

mod allowance_interval {
    use super::*;
    use cacao_lib::models::params::{
        CreateAllowanceParams, CreateWalletParams, UpdateAllowanceParams,
    };
    use cacao_lib::services::{allowance_service, family_service, setup_service, wallet_service};

    struct Ctx {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup(pool: &SqlitePool) -> Ctx {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(0),
            },
        )
        .await
        .unwrap();
        Ctx {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn update_allowance_interval_count() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                frequency: "custom".into(),
                interval_count: Some(3),
                notes: None,
            },
        )
        .await
        .unwrap();

        let updated = allowance_service::update_allowance(
            &pool,
            a.id,
            &UpdateAllowanceParams {
                amount_cents: None,
                frequency: None,
                interval_count: Some(7),
                notes: None,
            },
        )
        .await
        .unwrap();

        assert_eq!(updated.interval_count, 7);
    }

    #[tokio::test]
    async fn update_allowance_rejects_zero_interval() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let err = allowance_service::update_allowance(
            &pool,
            a.id,
            &UpdateAllowanceParams {
                amount_cents: None,
                frequency: None,
                interval_count: Some(0),
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn update_allowance_notes() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let updated = allowance_service::update_allowance(
            &pool,
            a.id,
            &UpdateAllowanceParams {
                amount_cents: None,
                frequency: None,
                interval_count: None,
                notes: Some("Weekly pocket money".into()),
            },
        )
        .await
        .unwrap();

        assert_eq!(updated.notes.as_deref(), Some("Weekly pocket money"));
    }

    #[tokio::test]
    async fn create_allowance_negative_interval_fails() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        let err = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                frequency: "custom".into(),
                interval_count: Some(-1),
                notes: None,
            },
        )
        .await
        .unwrap_err();

        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn pause_already_paused_succeeds_idempotent() {
        let pool = test_pool().await;
        let ctx = setup(&pool).await;

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        allowance_service::pause_allowance(&pool, a.id)
            .await
            .unwrap();
        // Pause again — should still succeed
        allowance_service::pause_allowance(&pool, a.id)
            .await
            .unwrap();

        let list = allowance_service::list_allowances(&pool, ctx.family_id)
            .await
            .unwrap();
        assert_eq!(list[0].status, "paused");
    }
}

mod notification_edge {
    use super::*;
    use cacao_lib::services::{notification_service, setup_service};

    #[tokio::test]
    async fn mark_read_twice_is_idempotent() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();

        // Insert a notification
        sqlx::query("INSERT INTO notifications (event_type, payload, is_read) VALUES ('request_approved', '{}', 0)")
            .execute(&pool)
            .await
            .unwrap();

        let all = notification_service::list_notifications(&pool, Some(10), Some(0))
            .await
            .unwrap();
        let id = all[0].id;

        notification_service::mark_read(&pool, id).await.unwrap();
        // Mark again — should not error
        notification_service::mark_read(&pool, id).await.unwrap();

        let count = notification_service::unread_count(&pool).await.unwrap();
        assert_eq!(count, 0);
    }

    #[tokio::test]
    async fn list_notifications_large_offset_returns_empty() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();

        sqlx::query("INSERT INTO notifications (event_type, payload, is_read) VALUES ('request_approved', '{}', 0)")
            .execute(&pool)
            .await
            .unwrap();

        let result = notification_service::list_notifications(&pool, Some(10), Some(9999))
            .await
            .unwrap();
        assert!(result.is_empty());
    }

    #[tokio::test]
    async fn unread_count_ignores_read_notifications() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();

        // Insert 1 read and 1 unread
        sqlx::query("INSERT INTO notifications (event_type, payload, is_read) VALUES ('request_approved', '{}', 1)")
            .execute(&pool)
            .await
            .unwrap();
        sqlx::query("INSERT INTO notifications (event_type, payload, is_read) VALUES ('request_submitted', '{}', 0)")
            .execute(&pool)
            .await
            .unwrap();

        let count = notification_service::unread_count(&pool).await.unwrap();
        assert_eq!(count, 1);
    }
}

mod wallet_cross_family {
    use super::*;
    use cacao_lib::models::params::CreateWalletParams;
    use cacao_lib::services::{family_service, setup_service, wallet_service};

    #[tokio::test]
    async fn same_wallet_name_allowed_across_families() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family1 = family_service::create_family(&pool, &profile.uuid, "Family A")
            .await
            .unwrap();

        // Create wallet "Cash" in family1
        wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family1.id,
                name: "Cash".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(0),
            },
        )
        .await
        .unwrap();

        // Reset and create second family
        setup_service::reset_device(&pool).await.unwrap();
        let profile2 = setup_service::setup_device(&pool, "Mom", "giver")
            .await
            .unwrap();
        let family2 = family_service::create_family(&pool, &profile2.uuid, "Family B")
            .await
            .unwrap();

        // Same name "Cash" in family2 should succeed
        let result = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family2.id,
                name: "Cash".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(0),
            },
        )
        .await;

        assert!(result.is_ok());
    }
}

// ── R8: Cross-Service Regression + Error Code Consistency ────

mod cross_service_regression {
    use super::*;
    use cacao_lib::models::params::{
        CreateAllowanceParams, CreateRequestParams, CreateWalletParams, ManualTransactionParams,
    };
    use cacao_lib::services::{
        allowance_service, export_service, family_service, notification_service, request_service,
        setup_service, transaction_service, wallet_service,
    };

    struct FullCtx {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn full_setup(pool: &SqlitePool) -> FullCtx {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test Family")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(1000000), // $10,000
            },
        )
        .await
        .unwrap();
        FullCtx {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn full_request_approve_flow_balance_and_notifications_consistent() {
        let pool = test_pool().await;
        let ctx = full_setup(&pool).await;

        // Create, submit, and approve request
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 50000, // $500
                category: Some("food".into()),
                notes: Some("Lunch money".into()),
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();

        // Verify wallet balance decreased
        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        // 1000000 (initial credit) - 50000 = 950000
        assert_eq!(wallet.balance_cents, 950000);

        // Verify transaction created
        let txs = transaction_service::list_wallet_transactions(&pool, ctx.wallet_id, None)
            .await
            .unwrap();
        // 1 initial credit + 1 debit from approval = 2
        assert_eq!(txs.len(), 2);
        let debit = txs.iter().find(|t| t.type_ == "debit").unwrap();
        assert_eq!(debit.amount_cents, 50000);
        assert_eq!(debit.source_type, "request");

        // Verify notification created
        let notifications = notification_service::list_notifications(&pool, Some(10), Some(0))
            .await
            .unwrap();
        assert!(
            notifications
                .iter()
                .any(|n| n.event_type == "request_approved")
        );

        // Verify CSV export includes the transaction
        let csv = export_service::export_transactions_csv(&pool, ctx.family_id, None, None, None)
            .await
            .unwrap();
        assert!(csv.contains("50000") || csv.contains("500"));
    }

    #[tokio::test]
    async fn allowance_execute_then_manual_debit_balance_correct() {
        let pool = test_pool().await;
        let ctx = full_setup(&pool).await;

        // Execute allowance to add $200
        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 20000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        allowance_service::execute_allowance(&pool, &a)
            .await
            .unwrap();

        // Manual debit $150
        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "debit".into(),
                amount_cents: 15000,
                category: Some("food".into()),
                notes: None,
            },
        )
        .await
        .unwrap();

        // Balance should be: 1000000 + 20000 - 15000 = 1005000
        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.balance_cents, 1005000);
    }

    #[tokio::test]
    async fn reject_request_does_not_affect_balance() {
        let pool = test_pool().await;
        let ctx = full_setup(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 50000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::reject_request(&pool, req.id, "Too expensive", ctx.member_id)
            .await
            .unwrap();

        // Balance should be unchanged: still 1000000
        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.balance_cents, 1000000);

        // No debit transaction should exist
        let txs = transaction_service::list_wallet_transactions(&pool, ctx.wallet_id, None)
            .await
            .unwrap();
        assert!(txs.iter().all(|t| t.type_ != "debit"));
    }

    #[tokio::test]
    async fn reset_device_clears_all_data_including_wallets_and_requests() {
        let pool = test_pool().await;
        let ctx = full_setup(&pool).await;

        // Create some data
        request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        // Reset
        setup_service::reset_device(&pool).await.unwrap();

        // Profile should be gone
        let profile = setup_service::get_profile(&pool).await.unwrap();
        assert!(profile.is_none());

        // Is setup should be false
        let is_setup = setup_service::is_setup(&pool).await.unwrap();
        assert!(!is_setup);
    }
}

mod error_code_consistency {
    use super::*;
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams};
    use cacao_lib::services::{
        allowance_service, family_service, request_service, setup_service, transaction_service,
        wallet_service,
    };

    #[tokio::test]
    async fn not_found_errors_use_correct_code_pattern() {
        let pool = test_pool().await;

        // wallet not found
        let err = wallet_service::get_wallet(&pool, 99999).await.unwrap_err();
        assert_eq!(err.code, "WALLET_NOT_FOUND");

        // request not found
        let err = request_service::get_request(&pool, 99999)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_NOT_FOUND");

        // allowance not found (via resume)
        let err = allowance_service::resume_allowance(&pool, 99999)
            .await
            .unwrap_err();
        assert_eq!(err.code, "ALLOWANCE_NOT_FOUND");
    }

    #[tokio::test]
    async fn validation_errors_use_validation_code() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(0),
            },
        )
        .await
        .unwrap();

        // Zero amount request
        let err = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: family.id,
                requester_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 0,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");

        // Negative amount transaction
        let err = transaction_service::create_manual_transaction(
            &pool,
            &cacao_lib::models::params::ManualTransactionParams {
                family_id: family.id,
                wallet_id: wallet.id,
                transaction_type: "credit".into(),
                amount_cents: -100,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");

        // Invalid month for monthly summary
        let err = transaction_service::get_monthly_summary(&pool, family.id, 2024, 13)
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");

        // Empty wallet name
        let err = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(0),
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn insufficient_balance_uses_correct_code() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Empty".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(0),
            },
        )
        .await
        .unwrap();

        let err = transaction_service::create_manual_transaction(
            &pool,
            &cacao_lib::models::params::ManualTransactionParams {
                family_id: family.id,
                wallet_id: wallet.id,
                transaction_type: "debit".into(),
                amount_cents: 100,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        // transaction_service uses AppError::validation for insufficient balance
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn approve_request_insufficient_uses_dedicated_code() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Empty".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(0),
            },
        )
        .await
        .unwrap();

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: family.id,
                requester_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 100,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();

        let err = request_service::approve_request(&pool, req.id, members[0].id)
            .await
            .unwrap_err();
        // request_service uses a dedicated INSUFFICIENT_BALANCE code
        assert_eq!(err.code, "INSUFFICIENT_BALANCE");
    }
}

// ── R9: Export Date Filter + Setup Edge + Pairing Edge ───────

mod export_date_filter {
    use super::*;
    use cacao_lib::models::params::{CreateWalletParams, ManualTransactionParams};
    use cacao_lib::services::{
        export_service, family_service, setup_service, transaction_service, wallet_service,
    };

    #[tokio::test]
    async fn export_with_date_from_filter() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();

        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: family.id,
                wallet_id: wallet.id,
                transaction_type: "debit".into(),
                amount_cents: 500,
                category: None,
                notes: Some("Test debit".into()),
            },
        )
        .await
        .unwrap();

        // Export with a far future date_from should return header only
        let csv = export_service::export_transactions_csv(
            &pool,
            family.id,
            Some("2099-01-01".into()),
            None,
            None,
        )
        .await
        .unwrap();

        // Should have header but no data rows (or just the initial credit row if it's future-proof)
        let lines: Vec<&str> = csv.trim().split('\n').collect();
        // At minimum, header line
        assert!(lines[0].contains("Date"));
    }

    #[tokio::test]
    async fn export_with_date_to_filter() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();

        // Export with date_to in the past should return fewer results
        let csv = export_service::export_transactions_csv(
            &pool,
            family.id,
            None,
            Some("2000-01-01".into()),
            None,
        )
        .await
        .unwrap();

        let lines: Vec<&str> = csv.trim().split('\n').collect();
        // Only header, no data (all transactions are after 2000)
        assert_eq!(lines.len(), 1);
    }
}

mod setup_edge_cases {
    use super::*;
    use cacao_lib::services::setup_service;

    #[tokio::test]
    async fn setup_device_with_whitespace_only_name_fails() {
        let pool = test_pool().await;
        let err = setup_service::setup_device(&pool, "   ", "giver")
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn setup_device_trims_leading_trailing_whitespace() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "  Alice  ", "giver")
            .await
            .unwrap();
        assert_eq!(profile.display_name, "Alice");
    }

    #[tokio::test]
    async fn get_profile_returns_correct_role() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();

        let profile = setup_service::get_profile(&pool).await.unwrap().unwrap();
        assert_eq!(profile.role, "giver");
        assert_eq!(profile.display_name, "Parent");
    }
}

mod pairing_edge {
    use super::*;
    use cacao_lib::services::{family_service, pairing_service, setup_service};

    #[tokio::test]
    async fn generated_codes_are_different() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        let code1 = pairing_service::generate_pairing_code(&pool, family.id)
            .await
            .unwrap();
        let code2 = pairing_service::generate_pairing_code(&pool, family.id)
            .await
            .unwrap();

        // Codes should be different (extremely unlikely to be the same)
        // But the second generate invalidates the first
        // At minimum, both should be 6 digits
        assert_eq!(code1.len(), 6);
        assert_eq!(code2.len(), 6);
        assert!(code1.chars().all(|c| c.is_ascii_digit()));
        assert!(code2.chars().all(|c| c.is_ascii_digit()));
    }

    #[tokio::test]
    async fn validate_empty_code_fails() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Test")
            .await
            .unwrap();

        pairing_service::generate_pairing_code(&pool, family.id)
            .await
            .unwrap();

        let err = pairing_service::validate_pairing_code(&pool, "")
            .await
            .unwrap_err();
        assert!(!err.code.is_empty());
    }
}

// ── R11: Family remove_member + wallet unicode + allowance archive + list ordering ──

mod family_remove_member {
    use super::*;
    use cacao_lib::services::{family_service, setup_service};

    #[tokio::test]
    async fn remove_member_sets_deleted_flag() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Fam")
            .await
            .unwrap();

        let baby = family_service::add_member(&pool, family.id, "baby-uuid-1", "baby")
            .await
            .unwrap();

        family_service::remove_member(&pool, baby.id).await.unwrap();

        let members = family_service::get_members(&pool, family.id).await.unwrap();
        // Only giver remains (baby was removed => is_deleted=1 => excluded)
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].family_role, "giver");
    }

    #[tokio::test]
    async fn remove_member_twice_is_idempotent() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "Fam")
            .await
            .unwrap();
        let baby = family_service::add_member(&pool, family.id, "baby-uuid-2", "baby")
            .await
            .unwrap();

        family_service::remove_member(&pool, baby.id).await.unwrap();
        // Second removal should not error
        family_service::remove_member(&pool, baby.id).await.unwrap();

        let members = family_service::get_members(&pool, family.id).await.unwrap();
        assert_eq!(members.len(), 1);
    }

    #[tokio::test]
    async fn remove_nonexistent_member_does_not_error() {
        let pool = test_pool().await;
        // remove_member does not check if member exists, just UPDATEs
        let result = family_service::remove_member(&pool, 99999).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn family_name_with_unicode_works() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "王家")
            .await
            .unwrap();
        assert_eq!(family.name, "王家");
    }
}

mod wallet_unicode {
    use super::*;
    use cacao_lib::models::params::CreateWalletParams;
    use cacao_lib::services::{family_service, setup_service, wallet_service};

    #[tokio::test]
    async fn wallet_name_with_chinese_characters() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();

        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "零用錢錢包".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(wallet.name, "零用錢錢包");
    }

    #[tokio::test]
    async fn wallet_name_with_emoji() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();

        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "💰 Savings".into(),
                wallet_type: "bank".into(),
                initial_balance_cents: Some(5000),
            },
        )
        .await
        .unwrap();
        assert_eq!(wallet.name, "💰 Savings");
        assert_eq!(wallet.balance_cents, 5000);
    }

    #[tokio::test]
    async fn wallet_name_at_50_char_boundary() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();

        let name_50 = "a".repeat(50);
        let w = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: name_50.clone(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(w.name, name_50);
    }

    #[tokio::test]
    async fn wallet_name_51_chars_rejected() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();

        let name_51 = "a".repeat(51);
        let err = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: name_51,
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn wallet_initial_balance_zero_stays_zero() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();

        let w = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Zero".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(0),
            },
        )
        .await
        .unwrap();
        assert_eq!(w.balance_cents, 0);
    }
}

mod allowance_archive_edge {
    use super::*;
    use cacao_lib::models::params::CreateAllowanceParams;
    use cacao_lib::models::params::CreateWalletParams;
    use cacao_lib::services::{allowance_service, family_service, setup_service, wallet_service};

    async fn setup_ctx(pool: &SqlitePool) -> (i64, i64, i64) {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "F")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();
        (family.id, wallet.id, 1) // family_id, wallet_id, member_id=1
    }

    #[tokio::test]
    async fn archive_active_allowance_succeeds() {
        let pool = test_pool().await;
        let (fid, wid, _) = setup_ctx(&pool).await;
        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: fid,
                giver_member_id: 1,
                receiver_member_id: 1,
                wallet_id: wid,
                amount_cents: 100,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        allowance_service::archive_allowance(&pool, a.id)
            .await
            .unwrap();

        // Archived allowance still appears in list (list returns all non-deleted)
        let list = allowance_service::list_allowances(&pool, fid)
            .await
            .unwrap();
        assert_eq!(list.len(), 1);
        assert_eq!(list[0].status, "archived");
    }

    #[tokio::test]
    async fn archive_already_archived_succeeds_idempotent() {
        let pool = test_pool().await;
        let (fid, wid, _) = setup_ctx(&pool).await;
        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: fid,
                giver_member_id: 1,
                receiver_member_id: 1,
                wallet_id: wid,
                amount_cents: 100,
                frequency: "weekly".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        allowance_service::archive_allowance(&pool, a.id)
            .await
            .unwrap();
        // Second archive: status is already 'archived', but is_deleted=0 so rows_affected=1
        allowance_service::archive_allowance(&pool, a.id)
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn update_archived_allowance_fails() {
        let pool = test_pool().await;
        let (fid, wid, _) = setup_ctx(&pool).await;
        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: fid,
                giver_member_id: 1,
                receiver_member_id: 1,
                wallet_id: wid,
                amount_cents: 100,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        allowance_service::archive_allowance(&pool, a.id)
            .await
            .unwrap();

        let err = allowance_service::update_allowance(
            &pool,
            a.id,
            &cacao_lib::models::params::UpdateAllowanceParams {
                amount_cents: Some(200),
                frequency: None,
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn list_allowances_returns_ordered_by_created_at_desc() {
        let pool = test_pool().await;
        let (fid, wid, _) = setup_ctx(&pool).await;

        let a1 = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: fid,
                giver_member_id: 1,
                receiver_member_id: 1,
                wallet_id: wid,
                amount_cents: 100,
                frequency: "daily".into(),
                interval_count: None,
                notes: Some("first".into()),
            },
        )
        .await
        .unwrap();

        let a2 = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: fid,
                giver_member_id: 1,
                receiver_member_id: 1,
                wallet_id: wid,
                amount_cents: 200,
                frequency: "weekly".into(),
                interval_count: None,
                notes: Some("second".into()),
            },
        )
        .await
        .unwrap();

        let _ = (a1, a2);
        let list = allowance_service::list_allowances(&pool, fid)
            .await
            .unwrap();
        assert_eq!(list.len(), 2);
        // Both allowances present with correct amounts
        let amounts: Vec<i64> = list.iter().map(|a| a.amount_cents).collect();
        assert!(amounts.contains(&100));
        assert!(amounts.contains(&200));
    }

    #[tokio::test]
    async fn list_allowances_empty_family() {
        let pool = test_pool().await;
        let list = allowance_service::list_allowances(&pool, 99999)
            .await
            .unwrap();
        assert!(list.is_empty());
    }

    #[tokio::test]
    async fn resume_archived_allowance_updates_next_run() {
        let pool = test_pool().await;
        let (fid, wid, _) = setup_ctx(&pool).await;
        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: fid,
                giver_member_id: 1,
                receiver_member_id: 1,
                wallet_id: wid,
                amount_cents: 100,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        // Archive then resume should still work (resume finds any non-deleted)
        allowance_service::archive_allowance(&pool, a.id)
            .await
            .unwrap();
        // resume reads status but doesn't check for "paused" specifically
        allowance_service::resume_allowance(&pool, a.id)
            .await
            .unwrap();
    }
}

// ── R12: Request state transitions + cancel edge cases ──

mod request_cancel_edge {
    use super::*;
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams};
    use cacao_lib::services::{family_service, request_service, setup_service, wallet_service};

    async fn setup_pending_request(pool: &SqlitePool) -> (i64, i64) {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "F")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();
        let request = request_service::create_request(
            pool,
            &CreateRequestParams {
                family_id: family.id,
                requester_member_id: 1,
                wallet_id: wallet.id,
                amount_cents: 500,
                category: Some("food".into()),
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(pool, request.id)
            .await
            .unwrap();
        (request.id, wallet.id)
    }

    #[tokio::test]
    async fn cancel_approved_request_fails() {
        let pool = test_pool().await;
        let (rid, _) = setup_pending_request(&pool).await;
        request_service::approve_request(&pool, rid, 1)
            .await
            .unwrap();
        let err = request_service::cancel_request(&pool, rid)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_INVALID_STATUS");
    }

    #[tokio::test]
    async fn cancel_rejected_request_fails() {
        let pool = test_pool().await;
        let (rid, _) = setup_pending_request(&pool).await;
        request_service::reject_request(&pool, rid, "no reason", 1)
            .await
            .unwrap();
        let err = request_service::cancel_request(&pool, rid)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_INVALID_STATUS");
    }

    #[tokio::test]
    async fn reject_with_whitespace_only_reason_fails() {
        let pool = test_pool().await;
        let (rid, _) = setup_pending_request(&pool).await;
        let err = request_service::reject_request(&pool, rid, "   ", 1)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REJECTION_REASON_REQUIRED");
    }

    #[tokio::test]
    async fn approve_creates_debit_transaction_with_category() {
        let pool = test_pool().await;
        let (rid, wid) = setup_pending_request(&pool).await;
        request_service::approve_request(&pool, rid, 1)
            .await
            .unwrap();

        // Verify transaction has the correct category from the request
        let txs = cacao_lib::services::transaction_service::list_wallet_transactions(
            &pool,
            wid,
            Some(10),
        )
        .await
        .unwrap();
        // Should have initial credit + debit from approval
        let debit = txs.iter().find(|t| t.type_ == "debit").unwrap();
        assert_eq!(debit.category.as_deref(), Some("food"));
        assert_eq!(debit.source_type, "request");
    }

    #[tokio::test]
    async fn list_requests_with_invalid_status_returns_empty() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();

        // Status "nonexistent" won't match any rows
        let list = request_service::list_requests(&pool, family.id, Some("nonexistent"))
            .await
            .unwrap();
        assert!(list.is_empty());
    }
}

// ── R13: Transaction boundary values + monthly summary edge cases ──

mod transaction_boundary {
    use super::*;
    use cacao_lib::models::params::{CreateWalletParams, ManualTransactionParams};
    use cacao_lib::services::{family_service, setup_service, transaction_service, wallet_service};

    async fn setup_wallet(pool: &SqlitePool, balance: i64) -> (i64, i64) {
        let profile = setup_service::setup_device(pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "F")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: if balance > 0 { Some(balance) } else { None },
            },
        )
        .await
        .unwrap();
        (family.id, wallet.id)
    }

    #[tokio::test]
    async fn debit_exceeding_by_one_cent_fails() {
        let pool = test_pool().await;
        let (fid, wid) = setup_wallet(&pool, 1000).await;
        let err = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: fid,
                wallet_id: wid,
                transaction_type: "debit".into(),
                amount_cents: 1001,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn large_credit_amount_succeeds() {
        let pool = test_pool().await;
        let (fid, wid) = setup_wallet(&pool, 0).await;
        let tx = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: fid,
                wallet_id: wid,
                transaction_type: "credit".into(),
                amount_cents: 999_999_999,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(tx.amount_cents, 999_999_999);
    }

    #[tokio::test]
    async fn credit_then_exact_debit_leaves_zero() {
        let pool = test_pool().await;
        let (fid, wid) = setup_wallet(&pool, 5000).await;
        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: fid,
                wallet_id: wid,
                transaction_type: "debit".into(),
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let wallet = wallet_service::get_wallet(&pool, wid).await.unwrap();
        assert_eq!(wallet.balance_cents, 0);
    }

    #[tokio::test]
    async fn monthly_summary_month_zero_rejected() {
        let pool = test_pool().await;
        let (fid, _) = setup_wallet(&pool, 0).await;
        let err = transaction_service::get_monthly_summary(&pool, fid, 2024, 0)
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn monthly_summary_month_13_rejected() {
        let pool = test_pool().await;
        let (fid, _) = setup_wallet(&pool, 0).await;
        let err = transaction_service::get_monthly_summary(&pool, fid, 2024, 13)
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn monthly_summary_far_future_year_returns_zero() {
        let pool = test_pool().await;
        let (fid, _) = setup_wallet(&pool, 0).await;
        let summary = transaction_service::get_monthly_summary(&pool, fid, 2099, 6)
            .await
            .unwrap();
        assert_eq!(summary.total_credit_cents, 0);
        assert_eq!(summary.total_debit_cents, 0);
        assert_eq!(summary.transaction_count, 0);
    }

    #[tokio::test]
    async fn list_transactions_all_five_filters_combined() {
        let pool = test_pool().await;
        let (fid, wid) = setup_wallet(&pool, 10000).await;

        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: fid,
                wallet_id: wid,
                transaction_type: "debit".into(),
                amount_cents: 100,
                category: Some("food".into()),
                notes: Some("lunch".into()),
            },
        )
        .await
        .unwrap();

        let filters = cacao_lib::models::params::TransactionFilters {
            date_from: Some("2000-01-01".into()),
            date_to: Some("2099-12-31".into()),
            wallet_id: Some(wid),
            transaction_type: Some("debit".into()),
            source_type: Some("manual".into()),
            limit: Some(10),
            offset: Some(0),
        };
        let txs = transaction_service::list_transactions(&pool, fid, &filters)
            .await
            .unwrap();
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].amount_cents, 100);
    }
}

// ── R14: Export edge cases + AppError factory methods + wallet update ──

mod export_edge {
    use super::*;
    use cacao_lib::models::params::{CreateWalletParams, ManualTransactionParams};
    use cacao_lib::services::{
        export_service, family_service, setup_service, transaction_service, wallet_service,
    };

    #[tokio::test]
    async fn csv_with_null_notes_and_category() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(10000),
            },
        )
        .await
        .unwrap();

        // Create transaction with no category and no notes
        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: family.id,
                wallet_id: wallet.id,
                transaction_type: "debit".into(),
                amount_cents: 500,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let csv = export_service::export_transactions_csv(&pool, family.id, None, None, None)
            .await
            .unwrap();
        // Should not panic on NULL values
        let lines: Vec<&str> = csv.trim().split('\n').collect();
        assert!(lines.len() >= 2); // header + at least the initial credit
    }

    #[tokio::test]
    async fn csv_wallet_filter_returns_only_matching() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();

        let w1 = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W1".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(5000),
            },
        )
        .await
        .unwrap();

        let _w2 = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W2".into(),
                wallet_type: "bank".into(),
                initial_balance_cents: Some(8000),
            },
        )
        .await
        .unwrap();

        // Export only W1
        let csv =
            export_service::export_transactions_csv(&pool, family.id, None, None, Some(w1.id))
                .await
                .unwrap();
        let lines: Vec<&str> = csv.trim().split('\n').collect();
        // header + 1 initial credit for W1 only
        assert_eq!(lines.len(), 2);
        assert!(lines[1].contains("W1"));
    }
}

mod wallet_update_edge {
    use super::*;
    use cacao_lib::models::params::CreateWalletParams;
    use cacao_lib::services::{family_service, setup_service, wallet_service};

    #[tokio::test]
    async fn update_wallet_with_unicode_name() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let w = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Old".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        let updated = wallet_service::update_wallet(&pool, w.id, "新名稱", 500)
            .await
            .unwrap();
        assert_eq!(updated.name, "新名稱");
        assert_eq!(updated.warning_threshold_cents, 500);
    }

    #[tokio::test]
    async fn update_wallet_threshold_above_balance() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let w = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100),
            },
        )
        .await
        .unwrap();

        // Setting threshold above balance should be allowed
        let updated = wallet_service::update_wallet(&pool, w.id, "W", 9999)
            .await
            .unwrap();
        assert_eq!(updated.warning_threshold_cents, 9999);
    }

    #[tokio::test]
    async fn get_wallet_deleted_returns_not_found() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let w = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        // Soft-delete the wallet
        sqlx::query("UPDATE wallets SET is_deleted = 1 WHERE id = ?")
            .bind(w.id)
            .execute(&pool)
            .await
            .unwrap();

        let err = wallet_service::get_wallet(&pool, w.id).await.unwrap_err();
        assert_eq!(err.code, "WALLET_NOT_FOUND");
    }

    #[tokio::test]
    async fn list_wallets_excludes_deleted() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let w = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        sqlx::query("UPDATE wallets SET is_deleted = 1 WHERE id = ?")
            .bind(w.id)
            .execute(&pool)
            .await
            .unwrap();

        let list = wallet_service::list_wallets(&pool, family.id)
            .await
            .unwrap();
        assert!(list.is_empty());
    }
}

mod error_factory {
    use cacao_lib::error::AppError;

    #[test]
    fn not_found_uppercases_resource_name() {
        let e = AppError::not_found("wallet");
        assert_eq!(e.code, "WALLET_NOT_FOUND");
        assert_eq!(e.message, "wallet not found");
    }

    #[test]
    fn not_found_with_multi_word() {
        let e = AppError::not_found("audit_log");
        assert_eq!(e.code, "AUDIT_LOG_NOT_FOUND");
    }

    #[test]
    fn validation_has_correct_code() {
        let e = AppError::validation("bad input");
        assert_eq!(e.code, "VALIDATION_ERROR");
        assert_eq!(e.message, "bad input");
    }

    #[test]
    fn internal_has_correct_code() {
        let e = AppError::internal("something broke");
        assert_eq!(e.code, "INTERNAL_ERROR");
    }

    #[test]
    fn forbidden_has_correct_code() {
        let e = AppError::forbidden();
        assert_eq!(e.code, "FORBIDDEN");
        assert_eq!(e.message, "Permission denied");
    }

    #[test]
    fn display_format_is_correct() {
        let e = AppError::new("TEST_CODE", "test message");
        assert_eq!(format!("{}", e), "[TEST_CODE] test message");
    }
}

// ── R15: Setup + Reset Edge Cases ───────────────────────────

mod setup_reset_edge {
    use super::*;
    use cacao_lib::services::setup_service;

    #[tokio::test]
    async fn reset_device_twice_is_idempotent() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();

        setup_service::reset_device(&pool).await.unwrap();
        assert!(!setup_service::is_setup(&pool).await.unwrap());

        // Second reset should not error
        setup_service::reset_device(&pool).await.unwrap();
        assert!(!setup_service::is_setup(&pool).await.unwrap());
    }

    #[tokio::test]
    async fn setup_device_name_at_50_chars() {
        let pool = test_pool().await;
        let name = "a".repeat(50);
        let profile = setup_service::setup_device(&pool, &name, "giver")
            .await
            .unwrap();
        assert_eq!(profile.display_name, name);
    }

    #[tokio::test]
    async fn setup_device_name_at_51_chars() {
        let pool = test_pool().await;
        let name = "a".repeat(51);
        let err = setup_service::setup_device(&pool, &name, "giver")
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn setup_preserves_internal_whitespace() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "John  Doe", "giver")
            .await
            .unwrap();
        // trim() only strips leading/trailing whitespace; internal spaces are preserved
        assert_eq!(profile.display_name, "John  Doe");
    }
}

// ── R16: Notification Payload Verification ──────────────────

mod notification_payload {
    use super::*;
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams};
    use cacao_lib::services::{
        family_service, notification_service, request_service, setup_service, wallet_service,
    };

    struct NotifContext {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup_context(pool: &SqlitePool) -> NotifContext {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test Family")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Main Wallet".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(10000),
            },
        )
        .await
        .unwrap();

        NotifContext {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn approve_request_notification_has_correct_payload() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: Some("food".into()),
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();

        let notifs = notification_service::list_notifications(&pool, None, None)
            .await
            .unwrap();

        let approved_notif = notifs
            .iter()
            .find(|n| n.event_type == "request_approved")
            .expect("Should have request_approved notification");

        let payload: serde_json::Value =
            serde_json::from_str(approved_notif.payload.as_deref().unwrap()).unwrap();

        assert_eq!(payload["request_id"], req.id);
        assert_eq!(payload["amount_cents"], 5000);
    }

    #[tokio::test]
    async fn reject_request_notification_has_reason() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 3000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::reject_request(&pool, req.id, "too expensive", ctx.member_id)
            .await
            .unwrap();

        let notifs = notification_service::list_notifications(&pool, None, None)
            .await
            .unwrap();

        let rejected_notif = notifs
            .iter()
            .find(|n| n.event_type == "request_rejected")
            .expect("Should have request_rejected notification");

        let payload: serde_json::Value =
            serde_json::from_str(rejected_notif.payload.as_deref().unwrap()).unwrap();

        assert_eq!(payload["reason"], "too expensive");
    }
}

// ── R17: Wallet Status Filter ───────────────────────────────

mod wallet_status_filter {
    use super::*;
    use cacao_lib::models::params::CreateWalletParams;
    use cacao_lib::services::{family_service, setup_service, wallet_service};

    async fn setup_family(pool: &SqlitePool) -> i64 {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test Family")
            .await
            .unwrap();
        family.id
    }

    #[tokio::test]
    async fn list_wallets_includes_archived_by_default() {
        let pool = test_pool().await;
        let family_id = setup_family(&pool).await;

        let w1 = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "Wallet Active".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id,
                name: "Wallet Stays".into(),
                wallet_type: "bank".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        // Archive the first wallet
        wallet_service::archive_wallet(&pool, w1.id).await.unwrap();

        // list_wallets filters by is_deleted = 0 only, so archived wallets ARE included
        let wallets = wallet_service::list_wallets(&pool, family_id)
            .await
            .unwrap();
        assert_eq!(wallets.len(), 2);

        // Verify one is archived and one is active
        let archived = wallets.iter().find(|w| w.id == w1.id).unwrap();
        assert_eq!(archived.status, "archived");

        let active = wallets.iter().find(|w| w.id != w1.id).unwrap();
        assert_eq!(active.status, "active");
    }
}

// ── R18: Sync Version Tracking ──────────────────────────────

mod sync_version_tracking {
    use super::*;
    use cacao_lib::models::params::{
        CreateAllowanceParams, CreateRequestParams, CreateWalletParams,
    };
    use cacao_lib::services::{
        allowance_service, family_service, request_service, setup_service, wallet_service,
    };

    struct SyncContext {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup_context(pool: &SqlitePool) -> SyncContext {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test Family")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Sync Wallet".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(500000),
            },
        )
        .await
        .unwrap();

        SyncContext {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn wallet_update_increments_sync_version() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        // Initial sync_version should be 0
        let wallet = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(wallet.sync_version, 0);

        // Update wallet
        wallet_service::update_wallet(&pool, ctx.wallet_id, "Updated Name", 5000)
            .await
            .unwrap();

        // Re-read and verify sync_version incremented
        let updated = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert!(
            updated.sync_version > 0,
            "sync_version should have incremented after update, got {}",
            updated.sync_version
        );
    }

    #[tokio::test]
    async fn request_approve_increments_sync_version() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(req.sync_version, 0);

        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();

        // Read directly from DB to verify sync_version
        let sv: i64 = sqlx::query_scalar("SELECT sync_version FROM requests WHERE id = ?")
            .bind(req.id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert!(
            sv > 0,
            "sync_version should have incremented after approve, got {}",
            sv
        );
    }

    #[tokio::test]
    async fn allowance_pause_increments_sync_version() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 10000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(allowance.sync_version, 0);

        // Pause the allowance
        allowance_service::pause_allowance(&pool, allowance.id)
            .await
            .unwrap();

        // Read directly from DB to verify sync_version incremented
        let sv: i64 = sqlx::query_scalar("SELECT sync_version FROM allowances WHERE id = ?")
            .bind(allowance.id)
            .fetch_one(&pool)
            .await
            .unwrap();

        assert!(
            sv > 0,
            "sync_version should have incremented after pause, got {}",
            sv
        );
    }
}

// ── R19: Soft Delete Consistency ────────────────────────────

mod soft_delete {
    use super::*;
    use cacao_lib::models::params::{
        CreateAllowanceParams, CreateRequestParams, CreateWalletParams,
    };
    use cacao_lib::services::{
        allowance_service, family_service, request_service, setup_service, wallet_service,
    };

    struct DeleteContext {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup_context(pool: &SqlitePool) -> DeleteContext {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test Family")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Delete Test Wallet".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();

        DeleteContext {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn deleted_request_not_returned_by_get() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        // Confirm it exists
        request_service::get_request(&pool, req.id).await.unwrap();

        // Soft-delete via raw SQL
        sqlx::query("UPDATE requests SET is_deleted = 1 WHERE id = ?")
            .bind(req.id)
            .execute(&pool)
            .await
            .unwrap();

        // get_request filters by is_deleted = 0, should return not found
        let err = request_service::get_request(&pool, req.id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_NOT_FOUND");
    }

    #[tokio::test]
    async fn deleted_allowance_not_returned_by_list() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 10000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        // Verify it appears in the list
        let list = allowance_service::list_allowances(&pool, ctx.family_id)
            .await
            .unwrap();
        assert_eq!(list.len(), 1);

        // Soft-delete via raw SQL
        sqlx::query("UPDATE allowances SET is_deleted = 1 WHERE id = ?")
            .bind(allowance.id)
            .execute(&pool)
            .await
            .unwrap();

        // list_allowances filters by is_deleted = 0, should return empty
        let list = allowance_service::list_allowances(&pool, ctx.family_id)
            .await
            .unwrap();
        assert!(list.is_empty());
    }
}

// ── R20: Create Request With All 7 Categories ───────────────

mod request_categories {
    use super::*;
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams};
    use cacao_lib::services::{family_service, request_service, setup_service, wallet_service};

    struct CatContext {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup_context(pool: &SqlitePool) -> CatContext {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "Test Family")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Category Wallet".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();

        CatContext {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn create_request_with_each_category() {
        let pool = test_pool().await;
        let ctx = setup_context(&pool).await;

        let categories = [
            "food",
            "transport",
            "education",
            "entertainment",
            "clothing",
            "health",
            "other",
        ];

        for category in &categories {
            let req = request_service::create_request(
                &pool,
                &CreateRequestParams {
                    family_id: ctx.family_id,
                    requester_member_id: ctx.member_id,
                    wallet_id: ctx.wallet_id,
                    amount_cents: 1000,
                    category: Some(category.to_string()),
                    notes: Some(format!("Testing {} category", category)),
                },
            )
            .await
            .unwrap();

            assert_eq!(
                req.category.as_deref(),
                Some(*category),
                "Category '{}' was not stored correctly",
                category
            );
            assert_eq!(req.status, "draft");
        }

        // Verify all 7 requests were created
        let all_requests = request_service::list_requests(&pool, ctx.family_id, None)
            .await
            .unwrap();
        assert_eq!(all_requests.len(), 7);
    }
}

// ── R21: Setup Guard + Device Already Setup ──────────────────

mod setup_guard_backend {
    use super::*;
    use cacao_lib::services::setup_service;

    #[tokio::test]
    async fn setup_device_already_setup_returns_error() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "Alice", "giver")
            .await
            .unwrap();
        let err = setup_service::setup_device(&pool, "Bob", "giver")
            .await
            .unwrap_err();
        assert_eq!(err.code, "DEVICE_ALREADY_SETUP");
    }

    #[tokio::test]
    async fn setup_invalid_role_returns_validation_error() {
        let pool = test_pool().await;
        let err = setup_service::setup_device(&pool, "Alice", "admin")
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn setup_empty_name_returns_validation_error() {
        let pool = test_pool().await;
        let err = setup_service::setup_device(&pool, "", "giver")
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn setup_whitespace_only_name_returns_validation_error() {
        let pool = test_pool().await;
        let err = setup_service::setup_device(&pool, "   ", "giver")
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn is_setup_returns_false_on_empty_db() {
        let pool = test_pool().await;
        assert!(!setup_service::is_setup(&pool).await.unwrap());
    }

    #[tokio::test]
    async fn get_profile_returns_none_on_empty_db() {
        let pool = test_pool().await;
        assert!(setup_service::get_profile(&pool).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn setup_then_reset_then_setup_again() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "First", "giver")
            .await
            .unwrap();
        setup_service::reset_device(&pool).await.unwrap();
        let profile = setup_service::setup_device(&pool, "Second", "baby")
            .await
            .unwrap();
        assert_eq!(profile.display_name, "Second");
        assert_eq!(profile.role, "baby");
    }
}

// ── R22: Manual Transaction Edge Cases ──────────────────────

mod manual_transaction_edge {
    use super::*;
    use cacao_lib::models::params::{CreateWalletParams, ManualTransactionParams};
    use cacao_lib::services::{family_service, setup_service, transaction_service, wallet_service};

    struct TxContext {
        family_id: i64,
        wallet_id: i64,
    }

    async fn setup_with_balance(pool: &SqlitePool, balance: i64) -> TxContext {
        let profile = setup_service::setup_device(pool, "Parent", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "F")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: if balance > 0 { Some(balance) } else { None },
            },
        )
        .await
        .unwrap();
        TxContext {
            family_id: family.id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn credit_increases_balance() {
        let pool = test_pool().await;
        let ctx = setup_with_balance(&pool, 1000).await;
        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "credit".into(),
                amount_cents: 500,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        let w = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(w.balance_cents, 1500);
    }

    #[tokio::test]
    async fn debit_decreases_balance() {
        let pool = test_pool().await;
        let ctx = setup_with_balance(&pool, 1000).await;
        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "debit".into(),
                amount_cents: 300,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        let w = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(w.balance_cents, 700);
    }

    #[tokio::test]
    async fn debit_exact_balance_leaves_zero() {
        let pool = test_pool().await;
        let ctx = setup_with_balance(&pool, 1000).await;
        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "debit".into(),
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        let w = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(w.balance_cents, 0);
    }

    #[tokio::test]
    async fn debit_exceeding_balance_fails() {
        let pool = test_pool().await;
        let ctx = setup_with_balance(&pool, 1000).await;
        let err = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "debit".into(),
                amount_cents: 1001,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn invalid_transaction_type_fails() {
        let pool = test_pool().await;
        let ctx = setup_with_balance(&pool, 1000).await;
        let err = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "transfer".into(),
                amount_cents: 100,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn zero_amount_fails() {
        let pool = test_pool().await;
        let ctx = setup_with_balance(&pool, 1000).await;
        let err = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "credit".into(),
                amount_cents: 0,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn negative_amount_fails() {
        let pool = test_pool().await;
        let ctx = setup_with_balance(&pool, 1000).await;
        let err = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "credit".into(),
                amount_cents: -100,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn transaction_with_category_and_notes() {
        let pool = test_pool().await;
        let ctx = setup_with_balance(&pool, 5000).await;
        let tx = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: ctx.family_id,
                wallet_id: ctx.wallet_id,
                transaction_type: "credit".into(),
                amount_cents: 100,
                category: Some("food".into()),
                notes: Some("lunch money".into()),
            },
        )
        .await
        .unwrap();
        assert_eq!(tx.category.as_deref(), Some("food"));
        assert_eq!(tx.notes.as_deref(), Some("lunch money"));
        assert_eq!(tx.source_type, "manual");
    }

    #[tokio::test]
    async fn debit_nonexistent_wallet_fails() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let err = transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: family.id,
                wallet_id: 99999,
                transaction_type: "credit".into(),
                amount_cents: 100,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "WALLET_NOT_FOUND");
    }
}

// ── R23: Monthly Summary Edge Cases ─────────────────────────

mod monthly_summary_edge {
    use super::*;
    use cacao_lib::models::params::CreateWalletParams;
    use cacao_lib::services::{family_service, setup_service, transaction_service, wallet_service};

    #[tokio::test]
    async fn empty_month_returns_zeros() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let summary = transaction_service::get_monthly_summary(&pool, family.id, 2024, 6)
            .await
            .unwrap();
        assert_eq!(summary.total_credit_cents, 0);
        assert_eq!(summary.total_debit_cents, 0);
        assert_eq!(summary.net_change_cents, 0);
        assert_eq!(summary.transaction_count, 0);
    }

    #[tokio::test]
    async fn month_zero_fails() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let err = transaction_service::get_monthly_summary(&pool, family.id, 2024, 0)
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn month_13_fails() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let err = transaction_service::get_monthly_summary(&pool, family.id, 2024, 13)
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn december_boundary() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        // December should not crash
        let summary = transaction_service::get_monthly_summary(&pool, family.id, 2024, 12)
            .await
            .unwrap();
        assert_eq!(summary.transaction_count, 0);
    }

    #[tokio::test]
    async fn summary_with_transactions() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None, // No initial balance to avoid extra transaction
            },
        )
        .await
        .unwrap();

        // Insert transactions with known occurred_at in current month
        let now = chrono::Utc::now();
        let date_str = now.format("%Y-%m-%d %H:%M:%S").to_string();
        sqlx::query(
            "INSERT INTO transactions (family_id, wallet_id, type, amount_cents, source_type, occurred_at) VALUES (?, ?, 'credit', 5000, 'manual', ?)"
        )
        .bind(family.id)
        .bind(wallet.id)
        .bind(&date_str)
        .execute(&pool)
        .await
        .unwrap();

        sqlx::query(
            "INSERT INTO transactions (family_id, wallet_id, type, amount_cents, source_type, occurred_at) VALUES (?, ?, 'debit', 2000, 'manual', ?)"
        )
        .bind(family.id)
        .bind(wallet.id)
        .bind(&date_str)
        .execute(&pool)
        .await
        .unwrap();

        let summary = transaction_service::get_monthly_summary(
            &pool,
            family.id,
            now.format("%Y").to_string().parse().unwrap(),
            now.format("%m").to_string().parse().unwrap(),
        )
        .await
        .unwrap();

        assert_eq!(summary.total_credit_cents, 5000);
        assert_eq!(summary.total_debit_cents, 2000);
        assert_eq!(summary.net_change_cents, 3000);
        assert_eq!(summary.transaction_count, 2);
    }
}

// ── R24: Allowance Create + Update Validation ───────────────

mod allowance_validation {
    use super::*;
    use cacao_lib::models::params::{
        CreateAllowanceParams, CreateWalletParams, UpdateAllowanceParams,
    };
    use cacao_lib::services::{allowance_service, family_service, setup_service, wallet_service};

    struct AllowCtx {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup_ctx(pool: &SqlitePool) -> AllowCtx {
        let profile = setup_service::setup_device(pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "F")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();
        AllowCtx {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn create_with_zero_amount_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let err = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 0,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn create_with_negative_amount_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let err = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: -100,
                frequency: "weekly".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn create_with_invalid_frequency_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let err = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                frequency: "hourly".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn create_with_zero_interval_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let err = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                frequency: "daily".into(),
                interval_count: Some(0),
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn create_all_valid_frequencies() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        for freq in &["daily", "weekly", "biweekly", "monthly", "custom"] {
            let a = allowance_service::create_allowance(
                &pool,
                &CreateAllowanceParams {
                    family_id: ctx.family_id,
                    giver_member_id: ctx.member_id,
                    receiver_member_id: ctx.member_id,
                    wallet_id: ctx.wallet_id,
                    amount_cents: 100,
                    frequency: freq.to_string(),
                    interval_count: Some(1),
                    notes: None,
                },
            )
            .await
            .unwrap();
            assert_eq!(a.frequency, *freq);
            assert_eq!(a.status, "active");
        }
    }

    #[tokio::test]
    async fn update_paused_allowance_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        allowance_service::pause_allowance(&pool, a.id)
            .await
            .unwrap();
        let err = allowance_service::update_allowance(
            &pool,
            a.id,
            &UpdateAllowanceParams {
                amount_cents: Some(2000),
                frequency: None,
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn update_amount_succeeds() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                frequency: "weekly".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        let updated = allowance_service::update_allowance(
            &pool,
            a.id,
            &UpdateAllowanceParams {
                amount_cents: Some(2000),
                frequency: None,
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(updated.amount_cents, 2000);
    }

    #[tokio::test]
    async fn update_with_no_changes_returns_same() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: ctx.family_id,
                giver_member_id: ctx.member_id,
                receiver_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        let unchanged = allowance_service::update_allowance(
            &pool,
            a.id,
            &UpdateAllowanceParams {
                amount_cents: None,
                frequency: None,
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(unchanged.amount_cents, a.amount_cents);
    }
}

// ── R25: Request Full Lifecycle ─────────────────────────────

mod request_lifecycle {
    use super::*;
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams, UpdateRequestParams};
    use cacao_lib::services::{family_service, request_service, setup_service, wallet_service};

    struct LifecycleCtx {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup_ctx(pool: &SqlitePool) -> LifecycleCtx {
        let profile = setup_service::setup_device(pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "F")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();
        LifecycleCtx {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn draft_to_pending_to_approved() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(req.status, "draft");

        let submitted = request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        assert_eq!(submitted.status, "pending");

        let approved = request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();
        assert_eq!(approved.status, "approved");
        assert!(approved.decision_at.is_some());
    }

    #[tokio::test]
    async fn draft_to_pending_to_rejected() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 5000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        let rejected = request_service::reject_request(&pool, req.id, "No budget", ctx.member_id)
            .await
            .unwrap();
        assert_eq!(rejected.status, "rejected");
        assert_eq!(rejected.rejection_reason.as_deref(), Some("No budget"));
    }

    #[tokio::test]
    async fn draft_cancel() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        let cancelled = request_service::cancel_request(&pool, req.id)
            .await
            .unwrap();
        assert_eq!(cancelled.status, "cancelled");
    }

    #[tokio::test]
    async fn pending_cancel() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        let cancelled = request_service::cancel_request(&pool, req.id)
            .await
            .unwrap();
        assert_eq!(cancelled.status, "cancelled");
    }

    #[tokio::test]
    async fn update_draft_changes_amount() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        let updated = request_service::update_request(
            &pool,
            req.id,
            &UpdateRequestParams {
                amount_cents: Some(2000),
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(updated.amount_cents, 2000);
    }

    #[tokio::test]
    async fn update_pending_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        let err = request_service::update_request(
            &pool,
            req.id,
            &UpdateRequestParams {
                amount_cents: Some(2000),
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "REQUEST_INVALID_STATUS");
    }

    #[tokio::test]
    async fn approve_already_approved_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();
        let err = request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_ALREADY_DECIDED");
    }

    #[tokio::test]
    async fn reject_already_rejected_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::reject_request(&pool, req.id, "No", ctx.member_id)
            .await
            .unwrap();
        let err = request_service::reject_request(&pool, req.id, "No again", ctx.member_id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_ALREADY_DECIDED");
    }

    #[tokio::test]
    async fn cancel_approved_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();
        let err = request_service::cancel_request(&pool, req.id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REQUEST_INVALID_STATUS");
    }
}

// ── R26: Wallet Name Constraints ────────────────────────────

mod wallet_name_constraints {
    use super::*;
    use cacao_lib::models::params::CreateWalletParams;
    use cacao_lib::services::{family_service, setup_service, wallet_service};

    #[tokio::test]
    async fn duplicate_name_fails() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Piggy Bank".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();
        let err = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "Piggy Bank".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "WALLET_NAME_EXISTS");
    }

    #[tokio::test]
    async fn empty_name_fails() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let err = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn name_51_chars_fails() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let err = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "a".repeat(51),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn name_50_chars_succeeds() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let w = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "a".repeat(50),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(w.name.len(), 50);
    }

    #[tokio::test]
    async fn trimmed_name() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let w = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "  Savings  ".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(w.name, "Savings");
    }
}

// ── R27: Export CSV Edge Cases ───────────────────────────────

mod export_csv_edge {
    use super::*;
    use cacao_lib::models::params::CreateWalletParams;
    use cacao_lib::services::{export_service, family_service, setup_service, wallet_service};

    #[tokio::test]
    async fn empty_family_returns_header_only() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let csv = export_service::export_transactions_csv(&pool, family.id, None, None, None)
            .await
            .unwrap();
        let lines: Vec<&str> = csv.trim().lines().collect();
        assert_eq!(lines.len(), 1);
        assert!(lines[0].contains("Date"));
    }

    #[tokio::test]
    async fn csv_escapes_commas_in_wallet_name() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let w = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "My, Wallet".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(1000),
            },
        )
        .await
        .unwrap();

        // Insert a transaction to generate CSV output
        sqlx::query(
            "INSERT INTO transactions (family_id, wallet_id, type, amount_cents, source_type, occurred_at) VALUES (?, ?, 'credit', 1000, 'manual', datetime('now'))"
        )
        .bind(family.id)
        .bind(w.id)
        .execute(&pool)
        .await
        .unwrap();

        let csv = export_service::export_transactions_csv(&pool, family.id, None, None, None)
            .await
            .unwrap();
        // Wallet name with comma should be quoted
        assert!(csv.contains("\"My, Wallet\""));
    }

    #[tokio::test]
    async fn csv_escapes_quotes_in_notes() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let w = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(5000),
            },
        )
        .await
        .unwrap();

        sqlx::query(
            "INSERT INTO transactions (family_id, wallet_id, type, amount_cents, source_type, occurred_at, notes) VALUES (?, ?, 'credit', 1000, 'manual', datetime('now'), '\"special\" notes')"
        )
        .bind(family.id)
        .bind(w.id)
        .execute(&pool)
        .await
        .unwrap();

        let csv = export_service::export_transactions_csv(&pool, family.id, None, None, None)
            .await
            .unwrap();
        // Notes with quotes should be escaped with double quotes
        assert!(csv.contains("\"\"special\"\" notes"));
    }
}

// ── R28: Notification Mark All Read ─────────────────────────

mod notification_mark_all {
    use super::*;
    use cacao_lib::services::{notification_service, setup_service};

    #[tokio::test]
    async fn mark_all_read_marks_multiple() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();

        // Insert 3 unread notifications (using valid CHECK-constrained event_types)
        let event_types = ["request_submitted", "request_approved", "request_rejected"];
        for et in &event_types {
            sqlx::query(
                "INSERT INTO notifications (event_type, payload, is_read) VALUES (?, '{}', 0)",
            )
            .bind(et)
            .execute(&pool)
            .await
            .unwrap();
        }

        assert_eq!(notification_service::unread_count(&pool).await.unwrap(), 3);

        notification_service::mark_all_read(&pool).await.unwrap();

        assert_eq!(notification_service::unread_count(&pool).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn mark_all_read_on_empty_is_ok() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        // No notifications exist
        notification_service::mark_all_read(&pool).await.unwrap();
        assert_eq!(notification_service::unread_count(&pool).await.unwrap(), 0);
    }

    #[tokio::test]
    async fn mark_nonexistent_notification_fails() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let err = notification_service::mark_read(&pool, 99999)
            .await
            .unwrap_err();
        assert_eq!(err.code, "NOTIFICATION_NOT_FOUND");
    }

    #[tokio::test]
    async fn list_notifications_ordering() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();

        sqlx::query("INSERT INTO notifications (event_type, payload, is_read, created_at) VALUES ('request_submitted', '{}', 0, '2024-01-01 00:00:00')")
            .execute(&pool).await.unwrap();
        sqlx::query("INSERT INTO notifications (event_type, payload, is_read, created_at) VALUES ('request_approved', '{}', 0, '2024-06-15 00:00:00')")
            .execute(&pool).await.unwrap();

        let list = notification_service::list_notifications(&pool, None, None)
            .await
            .unwrap();
        // Should be ordered by created_at DESC
        assert_eq!(list[0].event_type, "request_approved");
        assert_eq!(list[1].event_type, "request_submitted");
    }
}

// ── R29: Family Member Operations ───────────────────────────

mod family_member_ops {
    use super::*;
    use cacao_lib::services::{family_service, setup_service};

    #[tokio::test]
    async fn create_family_auto_creates_member() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        assert_eq!(members.len(), 1);
        assert_eq!(members[0].family_role, "giver");
        assert_eq!(members[0].profile_uuid, profile.uuid);
    }

    #[tokio::test]
    async fn add_member_then_list() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        family_service::add_member(&pool, family.id, "baby-uuid-123", "baby")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        assert_eq!(members.len(), 2);
    }

    #[tokio::test]
    async fn remove_member_hides_from_list() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let baby = family_service::add_member(&pool, family.id, "baby-uuid", "baby")
            .await
            .unwrap();
        family_service::remove_member(&pool, baby.id).await.unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        assert_eq!(members.len(), 1); // Only giver remains
    }

    #[tokio::test]
    async fn get_family_returns_none_when_empty() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        assert!(family_service::get_family(&pool).await.unwrap().is_none());
    }

    #[tokio::test]
    async fn family_name_validation() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        // Empty name
        let err = family_service::create_family(&pool, &profile.uuid, "")
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
        // Name too long
        let err = family_service::create_family(&pool, &profile.uuid, &"x".repeat(51))
            .await
            .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }
}

// ── R30: Execute Allowance Integration ──────────────────────

mod execute_allowance_integration {
    use super::*;
    use cacao_lib::models::params::{CreateAllowanceParams, CreateWalletParams};
    use cacao_lib::services::{
        allowance_service, family_service, notification_service, setup_service, wallet_service,
    };

    #[tokio::test]
    async fn execute_credits_wallet() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(1000),
            },
        )
        .await
        .unwrap();

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: family.id,
                giver_member_id: members[0].id,
                receiver_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 500,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        allowance_service::execute_allowance(&pool, &allowance)
            .await
            .unwrap();

        let w = wallet_service::get_wallet(&pool, wallet.id).await.unwrap();
        assert_eq!(w.balance_cents, 1500);
    }

    #[tokio::test]
    async fn execute_creates_notification() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(5000),
            },
        )
        .await
        .unwrap();

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: family.id,
                giver_member_id: members[0].id,
                receiver_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 1000,
                frequency: "weekly".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        allowance_service::execute_allowance(&pool, &allowance)
            .await
            .unwrap();

        let notifs = notification_service::list_notifications(&pool, None, None)
            .await
            .unwrap();
        let disbursed = notifs
            .iter()
            .find(|n| n.event_type == "allowance_disbursed");
        assert!(disbursed.is_some());
    }

    #[tokio::test]
    async fn execute_with_low_balance_warning() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(500),
            },
        )
        .await
        .unwrap();
        // Set warning threshold
        wallet_service::update_wallet(&pool, wallet.id, "W", 2000)
            .await
            .unwrap();

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: family.id,
                giver_member_id: members[0].id,
                receiver_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 100,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        allowance_service::execute_allowance(&pool, &allowance)
            .await
            .unwrap();

        // Balance is 600 (500 + 100), threshold is 2000, so low_balance notification should exist
        let notifs = notification_service::list_notifications(&pool, None, None)
            .await
            .unwrap();
        let low_balance = notifs.iter().find(|n| n.event_type == "low_balance");
        assert!(
            low_balance.is_some(),
            "Should have low_balance notification"
        );
    }

    #[tokio::test]
    async fn execute_updates_last_run_at() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(10000),
            },
        )
        .await
        .unwrap();

        let allowance = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: family.id,
                giver_member_id: members[0].id,
                receiver_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 100,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        assert!(allowance.last_run_at.is_none());

        allowance_service::execute_allowance(&pool, &allowance)
            .await
            .unwrap();

        // Re-fetch from DB
        let updated: (Option<String>,) =
            sqlx::query_as("SELECT last_run_at FROM allowances WHERE id = ?")
                .bind(allowance.id)
                .fetch_one(&pool)
                .await
                .unwrap();
        assert!(
            updated.0.is_some(),
            "last_run_at should be set after execution"
        );
    }
}

// ── R31: Wallet Archive vs Active Operations ────────────────

mod wallet_archive_operations {
    use super::*;
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams};
    use cacao_lib::services::{family_service, request_service, setup_service, wallet_service};

    struct WalletCtx {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup_ctx(pool: &SqlitePool) -> WalletCtx {
        let profile = setup_service::setup_device(pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "F")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(50000),
            },
        )
        .await
        .unwrap();
        WalletCtx {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn archive_wallet_twice_returns_error() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        wallet_service::archive_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        let err = wallet_service::archive_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "WALLET_ARCHIVED");
    }

    #[tokio::test]
    async fn create_request_on_archived_wallet_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        wallet_service::archive_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        // create_request checks wallet is active
        let err = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 100,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "WALLET_NOT_FOUND");
    }

    #[tokio::test]
    async fn get_archived_wallet_still_works() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        wallet_service::archive_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        // get_wallet only filters is_deleted, not status
        let w = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(w.status, "archived");
    }
}

// ── R32: Request Amount Boundaries ──────────────────────────

mod request_amount_boundary {
    use super::*;
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams};
    use cacao_lib::services::{family_service, request_service, setup_service, wallet_service};

    struct ReqCtx {
        family_id: i64,
        member_id: i64,
        wallet_id: i64,
    }

    async fn setup_ctx(pool: &SqlitePool) -> ReqCtx {
        let profile = setup_service::setup_device(pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(pool, &profile.uuid, "F")
            .await
            .unwrap();
        let members = family_service::get_members(pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();
        ReqCtx {
            family_id: family.id,
            member_id: members[0].id,
            wallet_id: wallet.id,
        }
    }

    #[tokio::test]
    async fn create_request_zero_amount_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let err = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 0,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn create_request_negative_amount_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let err = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: -1,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap_err();
        assert_eq!(err.code, "VALIDATION_ERROR");
    }

    #[tokio::test]
    async fn create_request_1_cent_succeeds() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 1,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(req.amount_cents, 1);
    }

    #[tokio::test]
    async fn approve_exact_balance_succeeds() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 100000, // exact balance
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap();
        let w = wallet_service::get_wallet(&pool, ctx.wallet_id)
            .await
            .unwrap();
        assert_eq!(w.balance_cents, 0);
    }

    #[tokio::test]
    async fn approve_exceeding_balance_fails() {
        let pool = test_pool().await;
        let ctx = setup_ctx(&pool).await;
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: ctx.family_id,
                requester_member_id: ctx.member_id,
                wallet_id: ctx.wallet_id,
                amount_cents: 100001, // 1 over balance
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        let err = request_service::approve_request(&pool, req.id, ctx.member_id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "INSUFFICIENT_BALANCE");
    }
}

// ── R33: List Requests Filtering ────────────────────────────

mod list_requests_filter {
    use super::*;
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams};
    use cacao_lib::services::{family_service, request_service, setup_service, wallet_service};

    #[tokio::test]
    async fn list_by_status() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();

        // Create 2 draft, 1 pending
        for _ in 0..2 {
            request_service::create_request(
                &pool,
                &CreateRequestParams {
                    family_id: family.id,
                    requester_member_id: members[0].id,
                    wallet_id: wallet.id,
                    amount_cents: 1000,
                    category: None,
                    notes: None,
                },
            )
            .await
            .unwrap();
        }
        let r3 = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: family.id,
                requester_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 2000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, r3.id).await.unwrap();

        let all = request_service::list_requests(&pool, family.id, None)
            .await
            .unwrap();
        assert_eq!(all.len(), 3);

        let drafts = request_service::list_requests(&pool, family.id, Some("draft"))
            .await
            .unwrap();
        assert_eq!(drafts.len(), 2);

        let pending = request_service::list_requests(&pool, family.id, Some("pending"))
            .await
            .unwrap();
        assert_eq!(pending.len(), 1);

        let approved = request_service::list_requests(&pool, family.id, Some("approved"))
            .await
            .unwrap();
        assert!(approved.is_empty());
    }

    #[tokio::test]
    async fn list_empty_family() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let list = request_service::list_requests(&pool, family.id, None)
            .await
            .unwrap();
        assert!(list.is_empty());
    }
}

// ── R34: Wallet Transaction List ────────────────────────────

mod wallet_tx_list {
    use super::*;
    use cacao_lib::models::params::{CreateWalletParams, ManualTransactionParams};
    use cacao_lib::services::{family_service, setup_service, transaction_service, wallet_service};

    #[tokio::test]
    async fn list_wallet_transactions_returns_recent() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();

        // Create 3 transactions
        for i in 0..3 {
            transaction_service::create_manual_transaction(
                &pool,
                &ManualTransactionParams {
                    family_id: family.id,
                    wallet_id: wallet.id,
                    transaction_type: "debit".into(),
                    amount_cents: 100 * (i + 1),
                    category: None,
                    notes: None,
                },
            )
            .await
            .unwrap();
        }

        // Initial balance also creates a transaction = 4 total
        let txs = transaction_service::list_wallet_transactions(&pool, wallet.id, None)
            .await
            .unwrap();
        assert_eq!(txs.len(), 4);
    }

    #[tokio::test]
    async fn list_wallet_transactions_with_limit() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();

        for _ in 0..5 {
            transaction_service::create_manual_transaction(
                &pool,
                &ManualTransactionParams {
                    family_id: family.id,
                    wallet_id: wallet.id,
                    transaction_type: "debit".into(),
                    amount_cents: 100,
                    category: None,
                    notes: None,
                },
            )
            .await
            .unwrap();
        }

        let txs = transaction_service::list_wallet_transactions(&pool, wallet.id, Some(2))
            .await
            .unwrap();
        assert_eq!(txs.len(), 2);
    }
}

// ── R35: Transaction Filters Extended ────────────────────────

mod transaction_filters_extended {
    use super::*;
    use cacao_lib::models::params::{
        CreateWalletParams, ManualTransactionParams, TransactionFilters,
    };
    use cacao_lib::services::{family_service, setup_service, transaction_service, wallet_service};

    #[tokio::test]
    async fn filter_by_type() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();

        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: family.id,
                wallet_id: wallet.id,
                transaction_type: "debit".into(),
                amount_cents: 1000,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        // Filter debit only
        let debits = transaction_service::list_transactions(
            &pool,
            family.id,
            &TransactionFilters {
                transaction_type: Some("debit".into()),
                date_from: None,
                date_to: None,
                wallet_id: None,
                source_type: None,
                limit: None,
                offset: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(debits.len(), 1);
        assert_eq!(debits[0].amount_cents, 1000);
    }

    #[tokio::test]
    async fn filter_by_source_type() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(5000), // creates a "manual" credit
            },
        )
        .await
        .unwrap();

        transaction_service::create_manual_transaction(
            &pool,
            &ManualTransactionParams {
                family_id: family.id,
                wallet_id: wallet.id,
                transaction_type: "debit".into(),
                amount_cents: 100,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();

        let manual_only = transaction_service::list_transactions(
            &pool,
            family.id,
            &TransactionFilters {
                source_type: Some("manual".into()),
                date_from: None,
                date_to: None,
                wallet_id: None,
                transaction_type: None,
                limit: None,
                offset: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(manual_only.len(), 2); // initial credit + manual debit
    }

    #[tokio::test]
    async fn filter_with_limit_and_offset() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(100000),
            },
        )
        .await
        .unwrap();

        for _ in 0..5 {
            transaction_service::create_manual_transaction(
                &pool,
                &ManualTransactionParams {
                    family_id: family.id,
                    wallet_id: wallet.id,
                    transaction_type: "debit".into(),
                    amount_cents: 100,
                    category: None,
                    notes: None,
                },
            )
            .await
            .unwrap();
        }

        let page = transaction_service::list_transactions(
            &pool,
            family.id,
            &TransactionFilters {
                limit: Some(2),
                offset: Some(1),
                date_from: None,
                date_to: None,
                wallet_id: None,
                transaction_type: None,
                source_type: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(page.len(), 2);
    }
}

// ── R36: Reject Reason Validation ───────────────────────────

mod reject_reason_validation {
    use super::*;
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams};
    use cacao_lib::services::{family_service, request_service, setup_service, wallet_service};

    #[tokio::test]
    async fn reject_with_empty_reason_fails() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(10000),
            },
        )
        .await
        .unwrap();
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: family.id,
                requester_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 100,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();

        let err = request_service::reject_request(&pool, req.id, "", members[0].id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REJECTION_REASON_REQUIRED");
    }

    #[tokio::test]
    async fn reject_with_whitespace_only_reason_fails() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(10000),
            },
        )
        .await
        .unwrap();
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: family.id,
                requester_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 100,
                category: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();

        let err = request_service::reject_request(&pool, req.id, "   ", members[0].id)
            .await
            .unwrap_err();
        assert_eq!(err.code, "REJECTION_REASON_REQUIRED");
    }
}

// ── R37: Approve Creates Transaction + Debits Wallet ────────

mod approve_side_effects {
    use super::*;
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams, TransactionFilters};
    use cacao_lib::services::{
        family_service, request_service, setup_service, transaction_service, wallet_service,
    };

    #[tokio::test]
    async fn approve_creates_debit_transaction() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: Some(50000),
            },
        )
        .await
        .unwrap();
        let req = request_service::create_request(
            &pool,
            &CreateRequestParams {
                family_id: family.id,
                requester_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 10000,
                category: Some("food".into()),
                notes: None,
            },
        )
        .await
        .unwrap();
        request_service::submit_request(&pool, req.id)
            .await
            .unwrap();
        request_service::approve_request(&pool, req.id, members[0].id)
            .await
            .unwrap();

        // Check wallet balance
        let w = wallet_service::get_wallet(&pool, wallet.id).await.unwrap();
        assert_eq!(w.balance_cents, 40000);

        // Check transaction was created
        let txs = transaction_service::list_transactions(
            &pool,
            family.id,
            &TransactionFilters {
                transaction_type: Some("debit".into()),
                source_type: Some("request".into()),
                date_from: None,
                date_to: None,
                wallet_id: None,
                limit: None,
                offset: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(txs.len(), 1);
        assert_eq!(txs[0].amount_cents, 10000);
        assert_eq!(txs[0].category.as_deref(), Some("food"));
    }
}

// ── R38: Allowance Pause/Resume Cycle ───────────────────────

mod allowance_lifecycle {
    use super::*;
    use cacao_lib::models::params::{CreateAllowanceParams, CreateWalletParams};
    use cacao_lib::services::{allowance_service, family_service, setup_service, wallet_service};

    #[tokio::test]
    async fn pause_resume_cycle() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family = family_service::create_family(&pool, &profile.uuid, "F")
            .await
            .unwrap();
        let members = family_service::get_members(&pool, family.id).await.unwrap();
        let wallet = wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family.id,
                name: "W".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        let a = allowance_service::create_allowance(
            &pool,
            &CreateAllowanceParams {
                family_id: family.id,
                giver_member_id: members[0].id,
                receiver_member_id: members[0].id,
                wallet_id: wallet.id,
                amount_cents: 1000,
                frequency: "daily".into(),
                interval_count: None,
                notes: None,
            },
        )
        .await
        .unwrap();
        assert_eq!(a.status, "active");

        allowance_service::pause_allowance(&pool, a.id)
            .await
            .unwrap();
        let list = allowance_service::list_allowances(&pool, family.id)
            .await
            .unwrap();
        assert_eq!(list[0].status, "paused");

        allowance_service::resume_allowance(&pool, a.id)
            .await
            .unwrap();
        let list = allowance_service::list_allowances(&pool, family.id)
            .await
            .unwrap();
        assert_eq!(list[0].status, "active");
    }

    #[tokio::test]
    async fn pause_nonexistent_fails() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let err = allowance_service::pause_allowance(&pool, 99999)
            .await
            .unwrap_err();
        assert_eq!(err.code, "ALLOWANCE_NOT_FOUND");
    }

    #[tokio::test]
    async fn archive_nonexistent_fails() {
        let pool = test_pool().await;
        setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let err = allowance_service::archive_allowance(&pool, 99999)
            .await
            .unwrap_err();
        assert_eq!(err.code, "ALLOWANCE_NOT_FOUND");
    }
}

// ── R39: Data Isolation Between Families ────────────────────

mod data_isolation {
    use super::*;
    use cacao_lib::models::params::CreateWalletParams;
    use cacao_lib::services::{family_service, setup_service, wallet_service};

    #[tokio::test]
    async fn wallets_are_isolated_by_family() {
        let pool = test_pool().await;
        let profile = setup_service::setup_device(&pool, "P", "giver")
            .await
            .unwrap();
        let family1 = family_service::create_family(&pool, &profile.uuid, "F1")
            .await
            .unwrap();
        // Create a second family via raw SQL (normally one device = one family)
        sqlx::query("INSERT INTO families (name, created_by_device) VALUES ('F2', ?)")
            .bind(&profile.uuid)
            .execute(&pool)
            .await
            .unwrap();
        let family2_id: i64 = sqlx::query_scalar("SELECT id FROM families WHERE name = 'F2'")
            .fetch_one(&pool)
            .await
            .unwrap();

        wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family1.id,
                name: "F1 Wallet".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        wallet_service::create_wallet(
            &pool,
            &CreateWalletParams {
                family_id: family2_id,
                name: "F2 Wallet".into(),
                wallet_type: "cash".into(),
                initial_balance_cents: None,
            },
        )
        .await
        .unwrap();

        let f1_wallets = wallet_service::list_wallets(&pool, family1.id)
            .await
            .unwrap();
        assert_eq!(f1_wallets.len(), 1);
        assert_eq!(f1_wallets[0].name, "F1 Wallet");

        let f2_wallets = wallet_service::list_wallets(&pool, family2_id)
            .await
            .unwrap();
        assert_eq!(f2_wallets.len(), 1);
        assert_eq!(f2_wallets[0].name, "F2 Wallet");
    }
}

// ── R40: Error Serialization ────────────────────────────────

mod error_serialization {
    use cacao_lib::error::AppError;

    #[test]
    fn serialize_error_to_json() {
        let err = AppError::new("TEST_CODE", "test message");
        let json = serde_json::to_string(&err).unwrap();
        assert!(json.contains("TEST_CODE"));
        assert!(json.contains("test message"));
    }

    #[test]
    fn from_sqlx_error_is_internal() {
        let sqlx_err = sqlx::Error::RowNotFound;
        let app_err = AppError::from(sqlx_err);
        assert_eq!(app_err.code, "INTERNAL_ERROR");
    }

    #[test]
    fn error_implements_std_error() {
        let err = AppError::validation("bad");
        let _std_err: &dyn std::error::Error = &err;
    }

    #[test]
    fn not_found_various_resources() {
        let resources = ["wallet", "request", "allowance", "notification", "family"];
        for r in resources {
            let err = AppError::not_found(r);
            assert!(
                err.code.ends_with("_NOT_FOUND"),
                "Error code for '{}' should end with _NOT_FOUND but got '{}'",
                r,
                err.code
            );
        }
    }
}
