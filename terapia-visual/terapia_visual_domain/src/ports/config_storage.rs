use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

/// Errores que pueden ocurrir al interactuar con la capa de almacenamiento de configuración.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum StorageError {
    #[error("Error reading the configuration file: {0}")]
    ReadError(String),
    #[error("Error writing the configuration file: {0}")]
    WriteError(String),
    #[error("Error parsing the configuration file: {0}")]
    ParseError(String),
    #[error("Configuration file not found, using default configuration")]
    NotFound,
}

/// Puerto para almacenar y recuperar la configuración de la terapia visual.
#[async_trait]
pub trait ConfigStorage<T>: Send + Sync
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync,
{
    /// Guarda la configuración de la terapia visual en un archivo o base de datos.
    async fn save(&self, config: &T) -> Result<(), StorageError>;

    /// Carga la configuración de la terapia visual desde un archivo o base de datos.
    async fn load(&self) -> Result<T, StorageError>;
}
