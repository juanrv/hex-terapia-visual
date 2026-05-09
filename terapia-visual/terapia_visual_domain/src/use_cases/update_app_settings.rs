use crate::{
    domain::AppSettings,
    ports::{ConfigStorage, StorageError},
};

#[derive(Debug, thiserror::Error)]
pub enum UpdateAppSettingsError {
    #[error(transparent)]
    Storage(#[from] StorageError),
}

/// Actualiza la configuracion de la aplicacion en el almacenamiento.
/// No afecta al overlay ni a la terapia activa (solo se usa para el idiota, etc.).
pub async fn update_app_settings(
    storage: &dyn ConfigStorage<AppSettings>,
    new_settings: &AppSettings,
) -> Result<(), UpdateAppSettingsError> {
    storage
        .save(new_settings)
        .await
        .map_err(UpdateAppSettingsError::Storage)?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use crate::use_cases::mocks::MockAppConfigStorage;

    use super::*;

    #[tokio::test]
    async fn test_update_app_settings_ok() {
        let storage = MockAppConfigStorage {
            app_settings: None,
            should_fail: false,
        };
        let new = AppSettings {
            language: "en".to_string(),
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
