//! # Caso de Uso: Actualizar Configuración de la Aplicación
//!
//! Este caso de uso actualiza la configuración global de la aplicación
//! (idioma, etc.) en el almacenamiento.
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_domain::use_cases::update_app_settings;
//! use terapia_visual_domain::ports::ConfigStorage;
//! use terapia_visual_domain::domain::AppSettings;
//!
//! # async fn example(storage: &dyn ConfigStorage<AppSettings>) -> Result<(), Box<dyn std::error::Error>> {
//! let settings = AppSettings::default();
//! update_app_settings::update_app_settings(storage, &settings).await?;
//! # Ok(())
//! # }
//! ```

use crate::{
    domain::AppSettings,
    ports::{ConfigStorage, StorageError},
};

/// Error que puede ocurrir al actualizar la configuración de la aplicación.
#[derive(Debug, thiserror::Error)]
pub enum UpdateAppSettingsError {
    /// Error al guardar en el almacenamiento.
    #[error(transparent)]
    Storage(#[from] StorageError),
}

/// Actualiza la configuración de la aplicación en el almacenamiento.
///
/// Esta función guarda la nueva configuración global de la aplicación.
/// No afecta al overlay ni a la terapia activa.
///
/// # Argumentos
///
/// * `storage` - Adaptador que implementa `ConfigStorage<AppSettings>`.
/// * `new_settings` - La nueva configuración de la aplicación.
///
/// # Errores
///
/// - [`UpdateAppSettingsError::Storage`] si falla al guardar en el almacenamiento.
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::use_cases::update_app_settings;
/// use terapia_visual_domain::ports::ConfigStorage;
/// use terapia_visual_domain::domain::{AppSettings, Language};
///
/// # async fn example(storage: &dyn ConfigStorage<AppSettings>) -> Result<(), Box<dyn std::error::Error>> {
/// let mut settings = AppSettings::default();
/// settings.set_language(Language::English);
/// update_app_settings::update_app_settings(storage, &settings).await?;
/// # Ok(())
/// # }
/// ```
pub async fn update_app_settings(
    storage: &dyn ConfigStorage<AppSettings>,
    new_settings: &AppSettings,
) -> Result<(), UpdateAppSettingsError> {
    storage.save(new_settings).await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::{domain::app_settings::Language, use_cases::mocks::MockAppConfigStorage};

    use super::*;

    #[tokio::test]
    async fn test_update_app_settings_ok() {
        let storage = MockAppConfigStorage {
            app_settings: None,
            should_fail: false,
        };
        let new = AppSettings {
            language: Language::English,
        };
        let result = update_app_settings(&storage, &new).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_app_settings_fails() {
        let storage = MockAppConfigStorage {
            app_settings: None,
            should_fail: true,
        };
        let new = AppSettings::default();

        let result = update_app_settings(&storage, &new).await;
        assert!(result.is_err())
    }
}
