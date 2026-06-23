//! # Comandos Tauri para el Frontend
//!
//! Este módulo organiza todos los comandos que el frontend puede invocar,
//! agrupados por funcionalidad para mantener un código limpio y mantenible.
//!
//! ## Estructura
//!
//! | Módulo | Propósito |
//! |--------|-----------|
//! | [`overlay`] | Comandos para la terapia de overlay (pantalla completa) |
//! | [`settings`] | Comandos para configuración de la aplicación (idioma, etc.) |
//!
//! ## Flujo típico
//!
//! 1. El frontend llama a un comando usando `invoke()`.
//! 2. El comando obtiene el estado de la aplicación (`AppState`).
//! 3. El comando llama al caso de uso correspondiente.
//! 4. El comando devuelve el resultado al frontend.
//!
//! ## Ejemplo desde el frontend
//!
//! ```typescript
//! import { invoke } from '@tauri-apps/api/core';
//!
//! // Iniciar la terapia de overlay
//! await invoke('cmd_start_overlay', { screenWidth: 1920, screenHeight: 1080 });
//!
//! // Cambiar el layout
//! await invoke('cmd_change_overlay_layout', { newLayout: 'Horizontal', screenWidth: 1920, screenHeight: 1080 });
//!
//! // Actualizar el idioma
//! await invoke('cmd_update_app_settings', { newSettings: { language: 'en' } });
//! ```

pub mod overlay;
pub mod reading;
pub mod settings;

pub use overlay::*;
pub use reading::*;
pub use settings::*;

/// Devuelve todos los comandos de tauri empaquetados y listo para inyectar en el builder
pub fn get_handlers<R: tauri::Runtime>(
) -> impl Fn(tauri::ipc::Invoke<R>) -> bool + Send + Sync + 'static {
    tauri::generate_handler![
        // Comandos de Overlay
        overlay::cmd_get_overlay_config,
        overlay::cmd_start_overlay,
        overlay::cmd_stop_overlay,
        overlay::cmd_update_overlay_config,
        overlay::cmd_change_overlay_layout,
        overlay::cmd_update_overlay_zone_color,
        overlay::cmd_update_overlay_zone_opacity,
        overlay::cmd_reset_overlay_config,
        // Comandos de Lectura
        reading::cmd_get_reading_config,
        reading::cmd_start_reading_therapy,
        reading::cmd_stop_reading_therapy,
        reading::cmd_update_reading_config,
        reading::cmd_change_reading_layout,
        reading::cmd_update_reading_zone_color,
        reading::cmd_update_reading_zone_opacity,
        reading::cmd_update_reading_settings,
        reading::cmd_reset_reading_config,
        reading::cmd_reading_window_resized,
        // Comandos Globales / Settings
        settings::cmd_get_app_settings,
        settings::cmd_update_app_settings,
        settings::cmd_exit_app,
    ]
}
