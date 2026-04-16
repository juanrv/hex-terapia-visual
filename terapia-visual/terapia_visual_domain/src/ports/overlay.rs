use async_trait::async_trait;

use crate::domain::TherapyConfig;

/// Errores que pueden ocurrir al interactuar con la capa de presentación (overlay).
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum OverlayError {
    #[error("Error creando la ventana de overlay: {0}")]
    CreationError(String),
    #[error("Error al actualizar las zonas de la ventana de overlay: {0}")]
    UpdateError(String),
    #[error("El overlay ya está activo")]
    AlreadyActive,
    #[error("El overlay no está activo")]
    NotActive,
}

/// Puerto para controlar la ventana de overlay de la terapia visual.
#[async_trait]
pub trait OverlayPort: Send + Sync {
    /// Muestra el overlay con la configuración dada.
    /// Calcula las zonas y las dibuja
    async fn show(
        &mut self,
        config: &TherapyConfig,
        screen_width: u32,
        screen_height: u32,
    ) -> Result<(), OverlayError>;

    /// Oculta el overlay (cierra la ventana).
    async fn hide(&mut self) -> Result<(), OverlayError>;

    /// Actualiza la terapia con una nueva configuración (puede cambiar layout, colores, opacidades).
    /// Si el overlay está activo, debe reflejar los cambios inmediatamente.
    async fn update_config(
        &mut self,
        config: &TherapyConfig,
        screen_width: u32,
        screen_height: u32,
    ) -> Result<(), OverlayError>;

    /// Devuelve el estado del  overlay (si está activo o no).
    fn is_active(&self) -> bool;
}
