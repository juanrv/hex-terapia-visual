use crate::{domain::TherapyConfig, ports::ConfigStorage};

/// Obtiene la configuración de terapia actual desde el almacenamiento.
/// Si no se encuentra el archivo de configuración, devuelve una configuración predeterminada.
pub async fn get_therapy_config(storage: &dyn ConfigStorage<TherapyConfig>) -> TherapyConfig {
    storage.load().await.unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Layout, TherapyConfig};
    use crate::use_cases::mocks::MockTherapyConfigStorage;

    #[tokio::test]
    async fn test_get_therapy_config_returns_default_on_error() {
        let storage = MockTherapyConfigStorage {
            config: None,
            should_fail_load: true,
            ..Default::default()
        };

        let config = get_therapy_config(&storage).await;
        // Debe dar configuración predeterminada
        assert_eq!(config, TherapyConfig::default());
    }

    #[tokio::test]
    async fn test_get_therapy_config_returns_stored() {
        // Crear configuracion distinta a la default
        let mut expected = TherapyConfig::default();
        expected.change_layout(Layout::Horizontal);

        // Simular comportamiento en disco
        let storage = MockTherapyConfigStorage {
            config: Some(expected.clone()),
            should_fail_load: false,
            ..Default::default()
        };

        let config = get_therapy_config(&storage).await;

        // Debe devolver exactamente lo que estaba guardado
        assert_eq!(config, expected);
    }
}
