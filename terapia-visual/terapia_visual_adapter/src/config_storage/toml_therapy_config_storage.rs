use async_trait::async_trait;
use std::fs;
use std::path::{Path, PathBuf};

use terapia_visual_domain::domain::TherapyConfig;
use terapia_visual_domain::ports::{ConfigStorage, StorageError};
use tracing::{error, info};

/// Almacenamiento de configuracion basado en achivo TOML
pub struct TomlTherapyConfigStorage {
    config_path: PathBuf,
}

impl TomlTherapyConfigStorage {
    /// Crea un nuevo almacenamiento de configuración, determinando la ruta del archivo de configuración.
    pub fn new(config_dir: impl AsRef<Path>) -> Self {
        let config_path = config_dir.as_ref().join("config.toml");
        TomlTherapyConfigStorage { config_path }
    }

    /// Intenta cargar la configuracion desde el archivo (version sincrona para uso interno).
    fn load_sync(&self) -> Result<TherapyConfig, StorageError> {
        let content = fs::read_to_string(&self.config_path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                StorageError::NotFound
            } else {
                StorageError::ReadError(e.to_string())
            }
        })?;

        let config: TherapyConfig =
            toml::from_str(&content).map_err(|e| StorageError::ParseError(e.to_string()))?;
        Ok(config)
    }

    /// Guarda la configuracion en el archivo (version sincrona para uso interno).
    fn save_sync(&self, config: &TherapyConfig) -> Result<(), StorageError> {
        let content =
            toml::to_string(config).map_err(|e| StorageError::WriteError(e.to_string()))?;
        fs::write(&self.config_path, content)
            .map_err(|e| StorageError::WriteError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl ConfigStorage<TherapyConfig> for TomlTherapyConfigStorage {
    async fn load(&self) -> Result<TherapyConfig, StorageError> {
        info!("Loading therapy configuration from: {:?}", self.config_path);
        let config_path = self.config_path.clone();
        let result = tokio::task::spawn_blocking(move || {
            let storage = TomlTherapyConfigStorage { config_path };
            storage.load_sync()
        })
        .await;

        match result {
            Ok(inner_result) => inner_result,
            Err(e) => {
                error!("Error while loading therapy configuration: {}", e);
                Err(StorageError::ReadError(e.to_string()))
            }
        }
    }

    async fn save(&self, config: &TherapyConfig) -> Result<(), StorageError> {
        info!("Saving therapy configuration to: {:?}", self.config_path);
        let config_path = self.config_path.clone();
        let config = config.clone();
        let result = tokio::task::spawn_blocking(move || {
            let storage = TomlTherapyConfigStorage { config_path };
            storage.save_sync(&config)
        })
        .await;

        match result {
            Ok(inner_result) => inner_result,
            Err(e) => {
                error!("Error while saving therapy configuration: {}", e);
                Err(StorageError::WriteError(e.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;

    use terapia_visual_domain::domain::{Color, Layout, Opacity, TherapyType, ZoneConfig};

    use super::*;

    fn sample_config() -> TherapyConfig {
        TherapyConfig::new(
            TherapyType::ColorDivision,
            Layout::Vertical,
            vec![
                ZoneConfig {
                    color: Color::new("#FF0000").unwrap(),
                    opacity: Opacity::new(0.8).unwrap(),
                },
                ZoneConfig {
                    color: Color::new("#0000FF").unwrap(),
                    opacity: Opacity::new(0.6).unwrap(),
                },
            ],
        )
        .unwrap()
    }

    #[tokio::test]
    async fn test_save_an_load() {
        let dir = TempDir::new().unwrap();
        let storage = TomlTherapyConfigStorage::new(dir.path());
        let config = sample_config();

        // Guardar
        storage.save(&config).await.unwrap();
        // Cargar
        let loaded = storage.load().await.unwrap();
        assert_eq!(config.therapy_type(), loaded.therapy_type());
        assert_eq!(config.layout(), loaded.layout());
        assert_eq!(config.zones_config().len(), loaded.zones_config().len());
        assert_eq!(
            config.zones_config()[0].color.as_str(),
            loaded.zones_config()[0].color.as_str()
        );
    }

    #[tokio::test]
    async fn test_load_not_found() {
        let dir = TempDir::new().unwrap();
        let storage = TomlTherapyConfigStorage::new(dir.path());
        // No hay archivo, debe devolver NotFound
        let result = storage.load().await;
        assert!(matches!(result, Err(StorageError::NotFound)));
    }
}
