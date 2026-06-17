//! # Almacenamiento TOML Genérico
//!
//! Este módulo proporciona un adaptador genérico para almacenar y cargar
//! configuraciones en archivos TOML.
//!
//! El adaptador es genérico sobre el tipo `T`, lo que permite usarlo con
//! cualquier estructura que implemente `Serialize` y `DeserializeOwned`.
//!
//! # Uso típico
//!
//! ```no_run
//! use terapia_visual_adapter::config_storage::TomlStorage;
//! use terapia_visual_domain::domain::TherapyConfig;
//! use terapia_visual_domain::ports::ConfigStorage;
//!
//! # async fn example() -> Result<(), Box<dyn std::error::Error>> {
//! // Crear un almacenamiento para TherapyConfig en "therapy_config.toml"
//! let storage = TomlStorage::new("./config", "therapy_config.toml");
//!
//! // Cargar la configuración (con fallback a default si no existe)
//! let config: TherapyConfig = storage.load().await.unwrap_or_default();
//!
//! // Guardar la configuración
//! storage.save(&config).await?;
//! # Ok(())
//! # }
//! ```
//!
//! # Características
//!
//! - **Genérico**: Funciona con cualquier tipo `T` serializable.
//! - **Operaciones bloqueantes en hilos dedicados**: Usa `tokio::task::spawn_blocking`
//!   para no bloquear el runtime asíncrono.
//! - **Manejo de errores**: Diferencia entre archivo no encontrado y otros errores.
//! - **Portable**: Los archivos se guardan en el directorio especificado.

use std::{
    fs,
    path::{Path, PathBuf},
};

use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};
use terapia_visual_domain::ports::{ConfigStorage, StorageError};
use tracing::{error, info};

/// Almacenamiento genérico de configuración basado en archivos TOML.
///
/// Puede almacenar cualquier tipo `T` que implemente `Serialize` y `DeserializeOwned`.
///
/// # Ejemplos
///
/// ```no_run
/// use terapia_visual_adapter::config_storage::TomlStorage;
/// use terapia_visual_domain::domain::AppSettings;
/// use terapia_visual_domain::ports::ConfigStorage;
///
/// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
/// // Almacenamiento para AppSettings en "app_config.toml"
/// let storage = TomlStorage::new("./config", "app_config.toml");
///
/// let settings: AppSettings = storage.load().await.unwrap_or_default();
/// # Ok(())
/// # }
/// ```
#[derive(Clone)]
pub struct TomlStorage {
    file_path: PathBuf,
}

impl TomlStorage {
    /// Crea un nuevo almacenamiento TOML en el directorio especificado.
    ///
    /// # Argumentos
    ///
    /// * `config_dir` - Directorio donde se guardará el archivo de configuración.
    /// * `filename` - Nombre del archivo (ej. "therapy_config.toml").
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_adapter::config_storage::TomlStorage;
    ///
    /// let storage = TomlStorage::new("./data", "config.toml");
    /// ```
    pub fn new(condfig_dir: impl AsRef<Path>, filename: &str) -> Self {
        Self {
            file_path: condfig_dir.as_ref().join(filename),
        }
    }

    /// Carga la configuración desde el archivo (versión síncrona para uso interno).
    ///
    /// # Errores
    ///
    /// - [`StorageError::NotFound`] si el archivo no existe.
    /// - [`StorageError::ReadError`] si falla la lectura.
    /// - [`StorageError::ParseError`] si el contenido TOML es inválido.
    fn load_sync<T: DeserializeOwned>(&self) -> Result<T, StorageError> {
        let content = fs::read_to_string(&self.file_path).map_err(|e| {
            if e.kind() == std::io::ErrorKind::NotFound {
                StorageError::NotFound
            } else {
                StorageError::ReadError(e.to_string())
            }
        })?;

        toml::from_str(&content).map_err(|e| StorageError::ParseError(e.to_string()))
    }

    /// Guarda la configuración en el archivo (versión síncrona para uso interno).
    ///
    /// # Errores
    ///
    /// - [`StorageError::WriteError`] si falla la escritura.
    fn save_sync<T: Serialize>(&self, config: &T) -> Result<(), StorageError> {
        let content =
            toml::to_string(config).map_err(|e| StorageError::WriteError(e.to_string()))?;
        fs::write(&self.file_path, content).map_err(|e| StorageError::WriteError(e.to_string()))?;
        Ok(())
    }
}

#[async_trait]
impl<T> ConfigStorage<T> for TomlStorage
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync + 'static,
{
    /// Carga la configuración desde el archivo TOML.
    ///
    /// Esta operación se ejecuta en un hilo bloqueante dedicado mediante
    /// `tokio::task::spawn_blocking` para no bloquear el runtime asíncrono.
    ///
    /// # Errores
    ///
    /// - [`StorageError::NotFound`] si el archivo no existe.
    /// - [`StorageError::ReadError`] si falla la lectura.
    /// - [`StorageError::ParseError`] si el contenido TOML es inválido.
    ///
    /// # Ejemplos
    ///
    /// ```no_run
    /// use terapia_visual_adapter::config_storage::TomlStorage;
    /// use terapia_visual_domain::domain::TherapyConfig;
    /// use terapia_visual_domain::ports::ConfigStorage;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let storage = TomlStorage::new(".", "config.toml");
    ///
    /// // Intentar cargar, usando default si no existe
    /// let config: TherapyConfig = storage.load().await.unwrap_or_default();
    /// # Ok(())
    /// # }
    /// ```
    async fn load(&self) -> Result<T, StorageError> {
        info!("Loading configuration from: {:?}", self.file_path);
        let storage = self.clone();

        let result = tokio::task::spawn_blocking(move || storage.load_sync::<T>()).await;

        match result {
            Ok(inner_result) => inner_result,
            Err(e) => {
                error!("Asynchronous error loading configuration: {}", e);
                Err(StorageError::ReadError(e.to_string()))
            }
        }
    }

    /// Guarda la configuración en el archivo TOML.
    ///
    /// Esta operación se ejecuta en un hilo bloqueante dedicado mediante
    /// `tokio::task::spawn_blocking` para no bloquear el runtime asíncrono.
    ///
    /// # Errores
    ///
    /// - [`StorageError::WriteError`] si falla la escritura.
    ///
    /// # Ejemplos
    ///
    /// ```no_run
    /// use terapia_visual_adapter::config_storage::TomlStorage;
    /// use terapia_visual_domain::domain::TherapyConfig;
    /// use terapia_visual_domain::ports::ConfigStorage;
    ///
    /// # async fn example() -> Result<(), Box<dyn std::error::Error>> {
    /// let storage = TomlStorage::new(".", "config.toml");
    /// let config = TherapyConfig::default();
    ///
    /// storage.save(&config).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn save(&self, config: &T) -> Result<(), StorageError> {
        info!("Saving configuration to: {:?}", self.file_path);
        let storage = self.clone();
        let config_clone = config.clone();

        let result = tokio::task::spawn_blocking(move || storage.save_sync(&config_clone)).await;

        match result {
            Ok(inner_result) => inner_result,
            Err(e) => {
                error!("Asynchronous error saving configuration: {}", e);
                Err(StorageError::WriteError(e.to_string()))
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;
    use terapia_visual_domain::domain::{AppSettings, app_settings::Language};

    fn sample_app_settings() -> AppSettings {
        AppSettings {
            language: Language::English,
        }
    }

    #[tokio::test]
    async fn test_save_and_load_generic() {
        let dir = TempDir::new().unwrap();
        // Probar que funciona con cualquier nombre
        let storage = TomlStorage::new(dir.path(), "test_app_config.toml");
        let config = sample_app_settings();

        // Guardar
        storage.save(&config).await.unwrap();

        // Cargar
        let loaded: AppSettings = storage.load().await.unwrap();
        assert_eq!(config, loaded);
    }

    #[tokio::test]
    async fn test_load_not_found() {
        let dir = TempDir::new().unwrap();
        let storage = TomlStorage::new(dir.path(), "missing.toml");

        // Debe devolver erorr NotFound
        let result: Result<AppSettings, StorageError> = storage.load().await;
        assert!(matches!(result, Err(StorageError::NotFound)));
    }
}
