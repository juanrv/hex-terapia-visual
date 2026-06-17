//! # Aplicación Tauri - Terapia Visual
//!
//! Este es el punto de entrada de la aplicación Tauri para la terapia visual.
//! Orquesta la inicialización de los adaptadores, la configuración de la ventana
//! principal, la bandeja del sistema y los comandos expuestos al frontend.
//!
//! # Estructura de la aplicación
//!
//! La aplicación está organizada en los siguientes módulos:
//!
//! | Módulo | Propósito |
//! |--------|-----------|
//! | [`commands`] | Comandos Tauri que el frontend puede invocar |
//! | [`setup`] | Inicialización de la aplicación (adaptadores, estado, etc.) |
//! | [`state`] | Estado global de la aplicación |
//! | [`tray`] | Configuración y eventos de la bandeja del sistema |
//!
//! # Flujo de inicio
//!
//! 1. `tauri::Builder::default()` inicia la construcción de la aplicación.
//! 2. [`setup::init`] configura los adaptadores, carga la configuración
//!    y crea el estado global.
//! 3. Se registran los plugins (opener, notification, log, etc.).
//! 4. Se registran los comandos que el frontend puede usar.
//! 5. Se configuran los manejadores de eventos (cierre de ventana, menú, etc.).
//! 6. La aplicación se inicia con `run()`.
//!
//! # Ejemplo de uso del frontend
//!
//! ```typescript
//! import { invoke } from '@tauri-apps/api/core';
//!
//! // Iniciar la terapia
//! await invoke('cmd_start_therapy', { screenWidth: 1920, screenHeight: 1080 });
//!
//! // Detener la terapia
//! await invoke('cmd_stop_therapy');
//!
//! // Obtener la configuración actual
//! const config = await invoke('cmd_get_therapy_config');
//! ```

pub mod commands;
pub mod setup;
pub mod state;
pub mod tray;

use tauri::Manager;

/// Punto de entrada de la aplicación Tauri.
///
/// Esta función construye, configura y ejecuta la aplicación Tauri.
/// Se encarga de:
/// - Inicializar el estado global y los adaptadores.
/// - Registrar los plugins necesarios.
/// - Exponer los comandos al frontend.
/// - Manejar eventos de ventana y menú.
///
/// # Ejecución
///
/// ```no_run
/// // Desde main.rs
/// fn main() {
///     terapia_visual_app_lib::run();
/// }
/// ```
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        // Configuración inicial (adaptadores, estado, bandeja)
        .setup(setup::init)
        // Plugins de Tauri
        .plugin(tauri_plugin_log::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .plugin(
            tauri_plugin_global_shortcut::Builder::new()
                .with_handler(setup::global_shortcut_handler)
                .build(),
        )
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            // Si la aplicación ya está en ejecución, traer al frente la ventana principal
            if let Some(window) = app.get_webview_window("main") {
                let _ = window.show();
                let _ = window.unminimize();
                let _ = window.set_focus();
            }
        }))
        // Comandos expuestos al frontend
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
            commands::cmd_reset_therapy_config,
        ])
        // Evento de cierre de ventana
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
        // Evento de menú de la bandeja
        .on_menu_event(|app, event| {
            if event.id.as_ref() == "quit" {
                setup::save_configs(app);
                std::process::exit(0);
            }
        })
        // Iniciar la aplicación
        .run(tauri::generate_context!())
        .expect("Error while running tauri application");
}
