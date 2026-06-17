//! # Estado Global de la Aplicación
//!
//! Este módulo define la estructura [`AppState`], que contiene todo el estado
//! compartido de la aplicación Tauri.
//!
//! El estado se inyecta en Tauri durante el [`crate::setup::init`] y se accede desde
//! los comandos mediante `tauri::State<AppState>`.
//!
//! # Componentes del estado
//!
//! | Campo | Tipo | Propósito |
//! |-------|------|-----------|
//! | `therapy_storage` | `TomlStorage` | Almacenamiento de configuración de terapia |
//! | `app_storage` | `TomlStorage` | Almacenamiento de configuración de la aplicación |
//! | `overlay` | `Mutex<TauriOverlay>` | Control de la ventana de superposición |
//! | `notifier` | `TauriSystemNotifier` | Notificaciones y bandeja del sistema |
//! | `current_config` | `RwLock<TherapyConfig>` | Configuración actual de la terapia (en memoria) |
//! | `is_toggling` | `AtomicBool` | Bandera para evitar múltiples pulsaciones del atajo |
//!
//! # Ejemplo de acceso al estado
//!
//! ```no_run
//! use tauri::State;
//! use terapia_visual_app_lib::state::AppState;
//!
//! #[tauri::command]
//! async fn example_command(state: State<'_, AppState>) -> Result<(), String> {
//!     // Leer la configuración actual
//!     let config = state.current_config.read().await;
//!     println!("Layout actual: {:?}", config.layout());
//!     Ok(())
//! }
//! ```

use std::sync::atomic::AtomicBool;

use terapia_visual_adapter::config_storage::TomlStorage;
use terapia_visual_adapter::notifier::TauriSystemNotifier;
use terapia_visual_adapter::overlay::TauriOverlay;
use terapia_visual_domain::domain::TherapyConfig;
use tokio::sync::{Mutex, RwLock};

/// Estado global de la aplicación Tauri.
///
/// Contiene todos los adaptadores y datos compartidos que necesitan
/// los comandos y el setup.
///
/// # Seguridad entre hilos
///
/// - `Mutex<T>`: Se usa para el overlay, que requiere acceso exclusivo
///   para operaciones como `show()` o `hide()`.
/// - `RwLock<T>`: Se usa para la configuración, permitiendo múltiples
///   lecturas concurrentes y escrituras exclusivas.
/// - `AtomicBool`: Se usa para la bandera de alternancia, que es
///   segura para operaciones atómicas sin bloqueos.
///
/// # Ejemplos
///
/// ```no_run
/// use terapia_visual_app_lib::state::AppState;
/// use tauri::State;
///
/// #[tauri::command]
/// async fn get_layout(state: State<'_, AppState>) -> Result<String, String> {
///     let config = state.current_config.read().await;
///     Ok(format!("{:?}", config.layout()))
/// }
/// ```
pub struct AppState {
    /// Almacenamiento de configuración de terapia (colores, layout, opacidades).
    pub therapy_storage: TomlStorage,

    /// Almacenamiento de configuración de la aplicación (idioma, etc.).
    pub app_storage: TomlStorage,

    /// Adaptador de overlay para controlar la ventana de superposición.
    /// Usa `Mutex` porque las operaciones requieren acceso exclusivo.
    pub overlay: Mutex<TauriOverlay>,

    /// Adaptador de notificaciones y bandeja del sistema.
    pub notifier: TauriSystemNotifier,

    /// Configuración actual de la terapia en memoria.
    /// Usa `RwLock` para permitir múltiples lecturas concurrentes.
    pub current_config: RwLock<TherapyConfig>,

    /// Bandera para evitar múltiples pulsaciones del atajo de teclado.
    /// Usa `AtomicBool` para operaciones atómicas sin bloqueos.
    pub is_toggling: AtomicBool,
}
