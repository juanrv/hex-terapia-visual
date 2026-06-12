pub mod commands;
pub mod setup;
pub mod state;
pub mod tray;

use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(setup::init)
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(setup::global_shortcut_handler)
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            commands::cmd_get_therapy_config,
            commands::cmd_start_therapy,
            commands::cmd_stop_therapy,
            commands::cmd_update_therapy_config,
            commands::cmd_get_app_settings,
            commands::cmd_update_app_settings,
            commands::cmd_change_layout,
            commands::cmd_update_zone_color,
            commands::cmd_update_zone_opacity,
        ])
        .on_window_event(|window, event| {
            if let tauri::WindowEvent::CloseRequested { api, .. } = event {
                api.prevent_close();

                if let Err(e) = window.hide() {
                    eprintln!("Error hiding main window: {}", e);
                }

                // Guardar configuracion
                setup::save_configs(window.app_handle());
            }
        })
        .on_menu_event(|app, event| {
            if event.id.as_ref() == "quit" {
                setup::save_configs(app);
                std::process::exit(0);
            }
        })
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
