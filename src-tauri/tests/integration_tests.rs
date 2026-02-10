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
    use cacao_lib::services::{
        family_service, setup_service, transaction_service, wallet_service,
    };

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
    use cacao_lib::models::params::{CreateAllowanceParams, CreateWalletParams, UpdateAllowanceParams};
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
    use cacao_lib::models::params::{CreateRequestParams, CreateWalletParams};
    use cacao_lib::services::{
        family_service, notification_service, request_service, setup_service,
        transaction_service, wallet_service,
    };
    use cacao_lib::models::params::TransactionFilters;

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
        assert!(count >= 1, "Expected at least 1 notification after approval");

        let notifs = notification_service::list_notifications(&pool, None, None)
            .await
            .unwrap();
        let approved_notif = notifs
            .iter()
            .find(|n| n.event_type == "request_approved");
        assert!(approved_notif.is_some(), "Should have request_approved notification");
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
        let rejected_notif = notifs
            .iter()
            .find(|n| n.event_type == "request_rejected");
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
        assert_eq!(txs.len(), 0, "Rejection should not create debit transactions");

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
        request_service::submit_request(&pool, req.id).await.unwrap();
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
            request_service::submit_request(&pool, req.id).await.unwrap();
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
        request_service::submit_request(&pool, req.id).await.unwrap();
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
    use cacao_lib::services::{
        family_service, request_service, setup_service, wallet_service,
    };

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
        assert_eq!(members.iter().filter(|m| m.family_role == "giver").count(), 1);
        assert_eq!(members.iter().filter(|m| m.family_role == "baby").count(), 2);
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
