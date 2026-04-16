use crate::{domain::TherapyConfig, ports::ConfigStorage};

/// Obtiene la configuración de terapia actual desde el almacenamiento.
/// Si no se encuentra el archivo de configuración, devuelve una configuración predeterminada.
pub async fn get_config(storage: &dyn ConfigStorage) -> TherapyConfig {
    match storage.load().await {
        Ok(config) => config,
        Err(_) => TherapyConfig::default(), // Si hay un error (incluyendo NotFound), devuelve la configuración predeterminada
    }
}
