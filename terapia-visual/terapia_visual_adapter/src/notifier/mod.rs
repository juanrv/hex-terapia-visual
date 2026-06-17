//! # Adaptadores de Notificaciones
//!
//! Este módulo contiene los adaptadores que implementan el puerto [`SystemNotifier`]
//! para diferentes plataformas.
//!
//! Actualmente solo se proporciona la implementación para Tauri, que utiliza
//! las capacidades nativas del sistema operativo a través de Tauri.
//!
//! # Adaptadores disponibles
//!
//! | Adaptador | Plataforma | Tecnología |
//! |-----------|------------|------------|
//! | [`TauriSystemNotifier`] | Windows, Linux, macOS | Tauri + `tauri-plugin-notification` |
//!
//! # Uso
//!
//! ```no_run
//! use terapia_visual_adapter::notifier::TauriSystemNotifier;
//! use terapia_visual_domain::ports::SystemNotifier;
//! use tauri::AppHandle;
//!
//! # fn example(app_handle: AppHandle) {
//! # let icon_inactive = &[];
//! # let icon_active = &[];
//! let notifier = TauriSystemNotifier::new(app_handle, icon_inactive, icon_active);
//! # }
//! ```

pub mod tauri_notifier;
pub use tauri_notifier::TauriSystemNotifier;
