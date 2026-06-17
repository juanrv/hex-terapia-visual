//! # Puerto de Almacenamiento de Configuración
//!
//! Define el contrato para guardar y cargar la configuración de la terapia
//! y la aplicación.
//!
//! Este puerto utiliza un tipo genérico `T` para permitir almacenar
//! diferentes tipos de configuración:
//! - [`crate::domain::TherapyConfig`]: Configuración de la terapia (colores, layout, etc.)
//! - [`crate::domain::AppSettings`]: Preferencias globales de la aplicación (idioma, etc.)
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_domain::ports::{ConfigStorage, StorageError};
//! use terapia_visual_domain::domain::TherapyConfig;
//!
//! async fn load_config(storage: &dyn ConfigStorage<TherapyConfig>) -> Result<TherapyConfig, StorageError> {
//!     storage.load().await
//! }
//! ```

use async_trait::async_trait;
use serde::{Serialize, de::DeserializeOwned};

/// Errores que pueden ocurrir al interactuar con el almacenamiento de configuración.
///
/// Estos errores son independientes del formato de almacenamiento (archivo, base de datos, etc.)
/// y representan fallos genéricos de lectura, escritura o parseo.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum StorageError {
    /// Error al leer el archivo de configuración (permisos, archivo corrupto, etc.).
    #[error("Error reading the configuration file: {0}")]
    ReadError(String),

    /// Error al escribir el archivo de configuración (disco lleno, permisos, etc.).
    #[error("Error writing the configuration file: {0}")]
    WriteError(String),

    /// Error al interpretar el contenido del archivo (formato inválido).
    #[error("Error parsing the configuration file: {0}")]
    ParseError(String),

    /// El archivo de configuración no existe y se usará la configuración por defecto.
    #[error("Configuration file not found, using default configuration")]
    NotFound,
}

/// Puerto para guardar y cargar la configuración de la terapia visual.
///
/// Este trait es genérico para soportar diferentes tipos de configuración:
/// - `ConfigStorage<TherapyConfig>` para la configuración de la terapia.
/// - `ConfigStorage<AppSettings>` para las preferencias globales.
///
/// # Requisitos del tipo `T`
///
/// - `Serialize` + `DeserializeOwned`: Para poder guardar/cargar desde el almacenamiento.
/// - `Clone`: Para permitir copias seguras durante las operaciones.
/// - `Send` + `Sync`: Para ser usado en entornos concurrentes (Tauri + Tokio).
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::ports::{ConfigStorage, StorageError};
/// use terapia_visual_domain::domain::TherapyConfig;
///
/// async fn save_and_load(
///     storage: &dyn ConfigStorage<TherapyConfig>,
///     config: &TherapyConfig,
/// ) -> Result<(), StorageError> {
///     storage.save(config).await?;
///     let loaded = storage.load().await?;
///     assert_eq!(config, &loaded);
///     Ok(())
/// }
/// ```
#[async_trait]
pub trait ConfigStorage<T>: Send + Sync
where
    T: Serialize + DeserializeOwned + Clone + Send + Sync,
{
    /// Guarda la configuración en el almacenamiento.
    ///
    /// # Argumentos
    ///
    /// * `config` - La configuración a guardar.
    ///
    /// # Errores
    ///
    /// Devuelve [`StorageError::WriteError`] si falla la escritura.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::TherapyConfig;
    /// # use terapia_visual_domain::ports::ConfigStorage;
    ///
    /// # async fn example(storage: &dyn ConfigStorage<TherapyConfig>) {
    /// let config = TherapyConfig::default();
    /// storage.save(&config).await.unwrap();
    /// # }
    /// ```
    async fn save(&self, config: &T) -> Result<(), StorageError>;

    /// Carga la configuración desde el almacenamiento.
    ///
    /// # Errores
    ///
    /// - [`StorageError::NotFound`] si el archivo no existe.
    /// - [`StorageError::ReadError`] si falla la lectura.
    /// - [`StorageError::ParseError`] si el contenido tiene formato inválido.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// # use terapia_visual_domain::domain::TherapyConfig;
    /// # use terapia_visual_domain::ports::ConfigStorage;
    ///
    /// # async fn example(storage: &dyn ConfigStorage<TherapyConfig>) {
    /// let config = storage.load().await.unwrap_or_else(|_| TherapyConfig::default());
    /// # }
    /// ```
    async fn load(&self) -> Result<T, StorageError>;
}
