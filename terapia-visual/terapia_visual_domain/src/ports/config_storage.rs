use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

/// Errores que pueden ocurrir al interactuar con la capa de almacenamiento de configuración.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum StorageError {
    #[error("Error al leer el archivo de configuración: {0}")]
    ReadError(String),
    #[error("Error al escribir el archivo de configuración: {0}")]
    WriteError(String),
    #[error("Error al parsear el archivo de configuración: {0}")]
    ParseError(String),
    #[error("Archivo de configuración no encontrado, usando configuración predeterminada")]
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
