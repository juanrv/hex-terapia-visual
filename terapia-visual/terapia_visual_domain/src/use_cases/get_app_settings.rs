use crate::{domain::AppSettings, ports::ConfigStorage};

/// Obtiene la configuracion de la aplicacion desde el almacenamiento
/// Si no existe o hay error, devuelve la configiracion por defecto
pub async fn get_app_settings(storage: &dyn ConfigStorage<AppSettings>) -> AppSettings {
    match storage.load().await {
        Ok(settings) => settings,
        Err(_) => AppSettings::default(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::AppSettings;
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
            language: "en".to_string(),
        };
        let storage = MockAppConfigStorage {
            app_settings: Some(expected.clone()),
            should_fail: false,
        };
        let settings = get_app_settings(&storage).await;
        assert_eq!(settings, expected);
    }
}
