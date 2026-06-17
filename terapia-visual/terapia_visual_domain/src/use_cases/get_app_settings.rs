//! # Caso de Uso: Obtener Configuración de la Aplicación
//!
//! Este caso de uso obtiene la configuración global de la aplicación
//! (idioma, etc.) desde el almacenamiento.
//!
//! # Comportamiento
//!
//! - Si existe una configuración guardada, la devuelve.
//! - Si no existe o hay un error, devuelve la configuración por defecto.
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_domain::use_cases::get_app_settings;
//! use terapia_visual_domain::ports::ConfigStorage;
//! use terapia_visual_domain::domain::AppSettings;
//!
//! # async fn example(storage: &dyn ConfigStorage<AppSettings>) {
//! let settings = get_app_settings::get_app_settings(storage).await;
//! # }
//! ```

use crate::{domain::AppSettings, ports::ConfigStorage};

/// Obtiene la configuración de la aplicación desde el almacenamiento.
///
/// Si la carga falla (archivo no encontrado, error de parseo, etc.),
/// devuelve la configuración por defecto.
///
/// # Argumentos
///
/// * `storage` - Adaptador que implementa `ConfigStorage<AppSettings>`.
///
/// # Retorno
///
/// La configuración actual o la configuración por defecto en caso de error.
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::use_cases::get_app_settings;
/// use terapia_visual_domain::ports::ConfigStorage;
/// use terapia_visual_domain::domain::AppSettings;
///
/// # async fn example(storage: &dyn ConfigStorage<AppSettings>) {
/// let settings = get_app_settings::get_app_settings(storage).await;
/// assert_eq!(settings, AppSettings::default());
/// # }
/// ```
pub async fn get_app_settings(storage: &dyn ConfigStorage<AppSettings>) -> AppSettings {
    storage.load().await.unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::AppSettings;
    use crate::domain::app_settings::Language;
    use crate::use_cases::mocks::MockAppConfigStorage;

    #[tokio::test]
    async fn test_get_app_settings_returns_default_on_error() {
        let storage = MockAppConfigStorage {
            app_settings: None,
            should_fail: true,
        };
        let settings = get_app_settings(&storage).await;
        assert_eq!(settings, AppSettings::default());
    }

    #[tokio::test]
    async fn test_get_app_settings_returns_stored() {
        let expected = AppSettings {
            language: Language::English,
        };
        let storage = MockAppConfigStorage {
            app_settings: Some(expected.clone()),
            should_fail: false,
        };
        let settings = get_app_settings(&storage).await;
        assert_eq!(settings, expected);
    }
}
