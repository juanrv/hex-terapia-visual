use std::{
    fs,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use terapia_visual_domain::{
    domain::AppSettings,
    ports::{ConfigStorage, StorageError},
};
use tracing::{error, info};

pub struct TomlAppConfigStorage {
    config_path: PathBuf,
}

impl TomlAppConfigStorage {
    pub fn new(config_dir: impl AsRef<Path>) -> Self {
        let config_path = config_dir.as_ref().join("app_config.toml");
        Self { config_path }
    }

    fn load_sync(&self) -> Result<AppSettings, StorageError> {
        let content = fs::read_to_string(&self.config_path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                StorageError::NotFound
            } else {
                StorageError::ReadError(e.to_string())
            }
        })?;
        let config: AppSettings =
            toml::from_str(&content).map_err(|e| StorageError::ParseError(e.to_string()))?;

        Ok(config)
    }

    fn save_sync(&self, config: &AppSettings) -> Result<(), StorageError> {
        let content =
            toml::to_string(config).map_err(|e| StorageError::WriteError(e.to_string()))?;
        fs::write(&self.config_path, content)
            .map_err(|e| StorageError::WriteError(e.to_string()))?;

        Ok(())
    }
}

#[async_trait]
impl ConfigStorage<AppSettings> for TomlAppConfigStorage {
    async fn load(&self) -> Result<AppSettings, StorageError> {
        info!("Loading app config from {:?}", self.config_path);
        let path = self.config_path.clone();
        let result = tokio::task::spawn_blocking(move || {
            let storage = TomlAppConfigStorage { config_path: path };
            storage.load_sync()
        })
        .await;
        match result {
            Ok(r) => r,
            Err(e) => {
                error!("Failedto load app_config: {}", e);
                Err(StorageError::ReadError(e.to_string()))
            }
        }
    }

    async fn save(&self, config: &AppSettings) -> Result<(), StorageError> {
        info!("Saving app_config to {:?}", self.config_path);
        let path = self.config_path.clone();
        let config = config.clone();
        let result = tokio::task::spawn_blocking(move || {
            let storage = TomlAppConfigStorage { config_path: path };
            storage.save_sync(&config)
        })
        .await;
        match result {
            Ok(r) => r,
            Err(e) => {
                error!("Failed to save app_config: {}", e);
                Err(StorageError::WriteError(e.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use tempfile::TempDir;
    use terapia_visual_domain::{
        domain::AppSettings,
        ports::{ConfigStorage, StorageError},
    };

    use crate::config_storage::TomlAppConfigStorage;

    fn sample_app_settings() -> AppSettings {
        AppSettings {
            language: "en".to_string(),
        }
    }
    #[tokio::test]
    async fn test_save_and_load() {
        let dir = TempDir::new().unwrap();
        let storage = TomlAppConfigStorage::new(dir.path());
        let config = sample_app_settings();

        storage.save(&config).await.unwrap();
        let loaded = storage.load().await.unwrap();
        assert_eq!(config, loaded);
    }

    #[tokio::test]
    async fn test_load_not_found() {
        let dir = TempDir::new().unwrap();
        let storage = TomlAppConfigStorage::new(dir.path());
        let result = storage.load().await;
        assert!(matches!(result, Err(StorageError::NotFound)));
    }

    #[tokio::test]
    async fn test_load_corrupted_file() {
        let dir = TempDir::new().unwrap();
        let config_path = dir.path().join("app_config.toml");
        std::fs::write(
            &config_path,
            "configuracion incorrecta de prueba dadssdfghj",
        )
        .unwrap();
        let storage = TomlAppConfigStorage::new(dir.path());
        let result = storage.load().await;
        assert!(matches!(result, Err(StorageError::ParseError(_))));
    }
}
