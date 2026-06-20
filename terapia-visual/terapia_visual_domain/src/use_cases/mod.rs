//! # Casos de Uso
//!
//! Este módulo contiene los **casos de uso** (interactores) de la aplicación,
//! que orquestan la lógica de negocio utilizando los puertos del dominio.
//!
//! Los casos de uso son el punto de entrada desde la capa de aplicación
//! (adaptadores primarios) hacia el dominio.
//!
//! # Casos de uso disponibles
//!
//! | Caso de uso | Propósito |
//! |-------------|-----------|
//! | [`start_overlay_therapy()`] | Iniciar la terapia visual |
//! | [`stop_therapy()`] | Detener la terapia visual |
//! | [`update_therapy_config()`] | Actualizar la configuración de la terapia |
//! | [`get_therapy_config()`] | Obtener la configuración de la terapia |
//! | [`update_app_settings()`] | Actualizar la configuración de la aplicación |
//! | [`get_app_settings()`] | Obtener la configuración de la aplicación |
//!
//! # Flujo típico
//!
//! 1. El **adaptador primario** (ej. comando Tauri) recibe una petición del usuario.
//! 2. El adaptador llama al caso de uso correspondiente.
//! 3. El caso de uso utiliza los **puertos** para interactuar con el exterior.
//! 4. El caso de uso devuelve el resultado al adaptador.
//!
//! # Ejemplo
//!
//! ```
//! use terapia_visual_domain::use_cases::start_overlay_therapy;
//! use terapia_visual_domain::ports::OverlayPort;
//! use terapia_visual_domain::domain::OverlayTherapyConfig;
//!
//! # async fn example(overlay: &mut dyn OverlayPort) -> Result<(), Box<dyn std::error::Error>> {
//! let config = OverlayTherapyConfig::default();
//! start_overlay_therapy::start_overlay_therapy(overlay, &config, 1920, 1080).await?;
//! # Ok(())
//! # }
//! ```

pub mod get_app_settings;
pub mod get_overlay_therapy;
pub mod start_overlay_therapy;
pub mod stop_overlay_therapy;
pub mod update_app_settings;
pub mod update_overlay_therapy;

#[cfg(test)]
pub mod mocks;

pub use get_app_settings::get_app_settings;
pub use get_overlay_therapy::get_overlay_therapy;
pub use start_overlay_therapy::start_overlay_therapy;
pub use stop_overlay_therapy::stop_overlay_therapy;
pub use update_app_settings::{UpdateAppSettingsError, update_app_settings};
pub use update_overlay_therapy::{UpdateConfigError, update_overlay_therapy};
