//! # Puertos del Dominio
//!
//! Este módulo define los **puertos** (interfaces) que el dominio utiliza
//! para comunicarse con el mundo exterior.
//!
//! Los puertos son contratos que especifican *qué* necesita el dominio,
//! sin especificar *cómo* se implementa. Las implementaciones concretas
//! (adaptadores) viven en el crate `terapia_visual_adapter`.
//!
//! # Puertos disponibles
//!
//! | Puerto | Propósito | Implementado por |
//! |--------|-----------|------------------|
//! | [`ConfigStorage<T>`] | Guardar y cargar configuración | `TomlStorage` |
//! | [`OverlayPort`] | Controlar la ventana de superposición | `TauriOverlay` |
//! | [`SystemNotifier`] | Notificaciones y bandeja del sistema | `TauriSystemNotifier` |
//!
//! # Flujo típico
//!
//! 1. El **caso de uso** recibe una referencia a un puerto (trait).
//! 2. El **adaptador concreto** implementa el trait.
//! 3. En la aplicación Tauri, se inyecta el adaptador en el caso de uso.
//!
//! # Ejemplo de uso
//!
//! ```
//! use terapia_visual_domain::ports::{ConfigStorage, StorageError};
//! use terapia_visual_domain::domain::TherapyConfig;
//!
//! async fn load_config(storage: &dyn ConfigStorage<TherapyConfig>) -> Result<TherapyConfig, StorageError> {
//!     storage.load().await
//! }
//! ```

pub mod config_storage;
pub mod overlay;
pub mod system_notifier;

pub use config_storage::{ConfigStorage, StorageError};
pub use overlay::{OverlayError, OverlayPort};
pub use system_notifier::{NotifierError, SystemNotifier};
