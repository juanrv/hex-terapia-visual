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
pub mod settings;

pub use overlay::*;
pub use settings::*;
