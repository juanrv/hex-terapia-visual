//! # Adaptadores para Terapia Visual
//!
//! Este crate contiene las implementaciones concretas de los puertos definidos
//! en el dominio (`terapia_visual_domain`).
//!
//! Los adaptadores traducen las operaciones del dominio en llamadas a tecnologías
//! específicas:
//!
//! - **Persistencia**: Almacenamiento en archivos TOML.
//! - **Overlay**: Ventana de superposición con Tauri.
//! - **Notificaciones**: Sistema de notificaciones y bandeja con Tauri.
//! - **Mensajes**: Internacionalización (español/inglés).
//!
//! # Estructura del crate
//!
//! | Módulo | Propósito |
//! |--------|-----------|
//! | [`config_storage`] | Adaptadores de persistencia (TOML) |
//! | [`overlay`] | Adaptadores de ventana de superposición |
//! | [`notifier`] | Adaptadores de notificaciones y bandeja |
//! | [`messages`] | Sistema de mensajes internacionalizados |
//!
//! # Ejemplo de uso
//!
//! ```no_run
//! use terapia_visual_adapter::config_storage::TomlStorage;
//! use terapia_visual_adapter::overlay::TauriOverlay;
//! use terapia_visual_adapter::notifier::TauriSystemNotifier;
//! use terapia_visual_domain::ports::{ConfigStorage, OverlayPort, SystemNotifier};
//! use tauri::AppHandle;
//!
//! # fn example(app_handle: AppHandle) {
//! # let icon_inactive = &[];
//! # let icon_active = &[];
//! // Crear adaptadores
//! let storage = TomlStorage::new("./config", "therapy_config.toml");
//! let overlay = TauriOverlay::new(app_handle.clone());
//! let notifier = TauriSystemNotifier::new(
//!     app_handle,
//!     icon_inactive,
//!     icon_active,
//! );
//! # }
//! ```

pub mod config_storage;
pub mod messages;
pub mod notifier;
pub mod overlay;

pub use config_storage::TomlStorage;
pub use notifier::TauriSystemNotifier;
pub use overlay::TauriOverlay;
