//! # Adaptadores de Overlay
//!
//! Este módulo contiene los adaptadores que implementan el puerto [`terapia_visual_domain::ports::OverlayPort`]
//! para controlar la ventana de superposición (overlay) de la terapia visual.
//!
//! # Adaptadores disponibles
//!
//! | Adaptador | Tecnología | Estado |
//! |-----------|------------|--------|
//! | [`TauriOverlay`] | Tauri (Webview) | ✅ Activo |
//! | `WindowsOverlay` | WinAPI (legado) | ⚠️ Deshabilitado (no se compila) |
//!
//! # Uso
//!
//! ```no_run
//! use terapia_visual_adapter::overlay::TauriOverlay;
//! use tauri::AppHandle;
//!
//! # fn example(app_handle: AppHandle) {
//! let overlay = TauriOverlay::new(app_handle);
//! # }
//! ```

pub mod tauri_overlay;
pub use tauri_overlay::TauriOverlay;
