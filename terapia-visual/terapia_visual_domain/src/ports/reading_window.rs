//! # Puerto de la Ventana de Lectura
//!
//! Define el contrato para controlar la ventana interactiva donde
//! se ejecuta la terapia de lectura.
//!
//! A diferencia de la terapia de overlay global, esta ventana está pensada
//! para ser una ventana normal del sistema operativo (opaca, con bordes,
//! redimensionable y que permite interactuar con el contenido).

use async_trait::async_trait;

use crate::domain::reading_therapy_config::ReadingTherapyConfig;

/// Errores que pueden ocurrir al interactuar con la ventana de lectura.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ReadingWindowError {
    /// Error al crear o abrir la ventana de lectura.
    #[error("Error creating the reading window: {0}")]
    CreationError(String),

    /// Error al intentar cerrar o esconder la ventana de lectura.
    #[error("Error closing the reading window: {0}")]
    CloseError(String),

    /// Error al intentar actualizar el contenido o la configuración de la ventana.
    #[error("Error updating the reading window: {0}")]
    UpdateError(String),

    /// Se intentó abrir la ventana cuando ya estaba abierta.
    #[error("Reading window is already active")]
    AlreadyActive,

    /// Se intentó cerrar o actualizar la ventana cuando estaba cerrada.
    #[error("Reading window is not active")]
    NotActive,
}

/// Puerto para controlar la ventana de la terapia de lectura.
///
/// Este trait define las operaciones básicas para mostrar el texto,
/// ocultar la ventana y actualizar los colores/tamaño de letra en tiempo real.
#[async_trait]
pub trait ReadingWindowPort: Send + Sync {
    /// Abre la ventana de lectura, inyectando el texto y aplicando la configuración.
    ///
    /// # Argumentos
    ///
    /// * `config` - Configuración de la terapia (colores, layout y ajustes de fuente).
    /// * `html_content` - El texto limpio (párrafos HTML) a mostrar.
    ///
    /// # Errores
    ///
    /// - [`ReadingWindowError::AlreadyActive`] si la ventana ya está activa.
    /// - [`ReadingWindowError::CreationError`] si el sistema falla al crear la ventana.
    async fn show(
        &self,
        config: &ReadingTherapyConfig,
        html_content: &str,
    ) -> Result<(), ReadingWindowError>;

    /// Actualiza la configuración visual de la ventana en tiempo real.
    ///
    /// Esto permite cambiar el tamaño de letra, color de fondo o el layout
    /// sin perder la posición de lectura (scroll) del usuario.
    async fn update_config(
        &mut self,
        config: &ReadingTherapyConfig,
    ) -> Result<(), ReadingWindowError>;

    /// Devuelve `true` si la ventana de lectura está actualmente abierta.
    fn is_active(&self) -> bool;
}
