use std::sync::Arc;
use tauri::Manager;
use tokio::sync::Mutex;

mod commands;
mod db;
pub mod error;
pub mod models;
mod scheduler;
pub mod services;
mod sync;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_log::Builder::default().build())
        .setup(|app| {
            let app_data_dir = app.path().app_data_dir()?;
            std::fs::create_dir_all(&app_data_dir)?;

            let pool = tauri::async_runtime::block_on(db::pool::init_pool(&app_data_dir))
                .expect("failed to initialize database");
            app.manage(pool.clone());

            // Start sync coordinator if device is set up
            tauri::async_runtime::block_on(async {
                if let Ok(Some(profile)) = services::setup_service::get_profile(&pool).await
                    && let Ok(Some(family)) = services::family_service::get_family(&pool).await
                {
                    let coordinator = match profile.role.as_str() {
                        "giver" => {
                            sync::coordinator::SyncCoordinator::start_as_giver(
                                pool.clone(),
                                &family.uuid,
                                &profile.display_name,
                            )
                            .await
                        }
                        _ => {
                            sync::coordinator::SyncCoordinator::start_as_baby(
                                pool.clone(),
                                &family.uuid,
                                &profile.uuid,
                            )
                            .await
                        }
                    };

                    match coordinator {
                        Ok(coord) => {
                            app.manage(Arc::new(Mutex::new(coord)));
                            log::info!("Sync coordinator started as {}", profile.role);
                        }
                        Err(e) => {
                            log::warn!("Failed to start sync coordinator: {}", e);
                        }
                    }

                    // Start allowance runner on Giver device
                    if profile.role == "giver" {
                        let cancel = tokio_util::sync::CancellationToken::new();
                        let pool_clone = pool.clone();
                        let cancel_clone = cancel.clone();
                        tokio::spawn(async move {
                            scheduler::allowance_runner::start(pool_clone, cancel_clone).await;
                        });
                        app.manage(cancel);
                        log::info!("Allowance runner started");
                    }
                }
            });

            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::setup::setup_device,
            commands::setup::get_profile,
            commands::setup::is_device_setup,
            commands::setup::create_family,
            commands::setup::get_family,
            commands::setup::get_family_members,
            commands::setup::generate_pairing_code,
            commands::setup::join_family_with_code,
            commands::setup::remove_family_member,
            commands::setup::reset_device,
            commands::wallet::create_wallet,
            commands::wallet::list_wallets,
            commands::wallet::get_wallet,
            commands::wallet::update_wallet,
            commands::wallet::archive_wallet,
            commands::request::create_request,
            commands::request::update_request,
            commands::request::submit_request,
            commands::request::approve_request,
            commands::request::reject_request,
            commands::request::cancel_request,
            commands::request::list_requests,
            commands::request::get_request,
            commands::request::list_wallet_transactions,
            commands::sync::get_sync_status,
            commands::sync::trigger_sync,
            commands::sync::stop_sync,
            commands::allowance::create_allowance,
            commands::allowance::update_allowance,
            commands::allowance::pause_allowance,
            commands::allowance::resume_allowance,
            commands::allowance::list_allowances,
            commands::transaction::list_transactions,
            commands::transaction::create_manual_transaction,
            commands::transaction::get_monthly_summary,
            commands::notification::list_notifications,
            commands::notification::mark_notification_read,
            commands::notification::mark_all_notifications_read,
            commands::notification::unread_notification_count,
            commands::dashboard::get_dashboard_data,
            commands::auth::set_pin,
            commands::auth::verify_pin,
            commands::auth::remove_pin,
            commands::auth::has_pin,
            commands::audit::list_audit_logs,
            commands::export::export_csv,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
