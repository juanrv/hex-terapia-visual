//! # Almacenamiento de Configuración
//!
//! Este módulo proporciona adaptadores para guardar y cargar configuración
//! en formato TOML.
//!
//! El adaptador principal es [`TomlStorage`], que es genérico y puede usarse
//! con cualquier tipo que implemente `Serialize` y `DeserializeOwned`.

pub mod toml_storage;

pub use toml_storage::TomlStorage;
