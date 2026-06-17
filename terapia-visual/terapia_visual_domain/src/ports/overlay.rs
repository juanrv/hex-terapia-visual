//! # Puerto de Superposición (Overlay)
//!
//! Define el contrato para controlar la ventana de superposición
//! que muestra la terapia visual.
//!
//! El overlay es una ventana transparente a pantalla completa que se
//! superpone a todas las demás ventanas, mostrando las zonas de color
//! de la terapia.
//!
//! # Flujo típico
//!
//! 1. El usuario inicia la terapia → se llama a `show()`.
//! 2. El usuario cambia colores/opacidad → se llama a `update_config()`.
//! 3. El usuario detiene la terapia → se llama a `hide()`.
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_domain::ports::{OverlayPort, OverlayError};
//! use terapia_visual_domain::domain::TherapyConfig;
//!
//! # async fn example(overlay: &mut dyn OverlayPort) -> Result<(), OverlayError> {
//! let config = TherapyConfig::default();
//! overlay.show(&config, 1920, 1080).await?;
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;

use crate::domain::TherapyConfig;

/// Errores que pueden ocurrir al interactuar con el overlay.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum OverlayError {
    /// Error al crear la ventana de overlay.
    #[error("Error creating the overlay window: {0}")]
    CreationError(String),

    /// Error al cerrar la ventana de overlay.
    #[error("Error closing the overlay window: {0}")]
    CloseError(String),

    /// Error al actualizar las zonas de la ventana de overlay.
    #[error("Error updating the regions of the overlay window: {0}")]
    UpdateError(String),

    /// Se intentó iniciar la terapia cuando el overlay ya está activo
    #[error("Overlay already active")]
    AlreadyActive,

    /// Se intentó detener la terapia cuando el overlay ya está inactivo.
    #[error("Overlay not active")]
    NotActive,
}

/// Puerto para controlar la ventana de overlay de la terapia visual.
///
/// Este trait define las operaciones básicas para mostrar, ocultar
/// y actualizar la ventana de superposición.
///
/// # Requisitos de implementación
///
/// - `show()` debe crear la ventana si no existe.
/// - `hide()` debe cerrar la ventana y liberar recursos.
/// - `update_config()` debe reflejar los cambios en la ventana en tiempo real.
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::ports::{OverlayPort, OverlayError};
/// use terapia_visual_domain::domain::TherapyConfig;
///
/// # async fn example(overlay: &mut dyn OverlayPort) -> Result<(), OverlayError> {
/// // Mostrar el overlay
/// let config = TherapyConfig::default();
/// overlay.show(&config, 1920, 1080).await?;
///
/// // Verificar si está activo
/// assert!(overlay.is_active());
///
/// // Ocultar el overlay
/// overlay.hide().await?;
/// # Ok(())
/// # }
/// ```
#[async_trait]
pub trait OverlayPort: Send + Sync {
    /// Muestra el overlay con la configuración dada.
    ///
    /// Calcula las zonas según el layout de la configuración y las dibuja.
    ///
    /// # Argumentos
    ///
    /// * `config` - Configuración de la terapia (colores, layout, opacidades).
    /// * `screen_width` - Ancho de la pantalla en píxeles.
    /// * `screen_height` - Alto de la pantalla en píxeles.
    ///
    /// # Errores
    ///
    /// - [`OverlayError::AlreadyActive`] si el overlay ya está activo.
    /// - [`OverlayError::CreationError`] si falla la creación de la ventana.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// # use terapia_visual_domain::ports::OverlayPort;
    /// # use terapia_visual_domain::domain::TherapyConfig;
    /// # async fn example(overlay: &mut dyn OverlayPort) {
    /// let config = TherapyConfig::default();
    /// if let Err(e) = overlay.show(&config, 1920, 1080).await {
    ///     eprintln!("Failed to show overlay: {}", e);
    /// }
    /// # }
    /// ```
    async fn show(
        &mut self,
        config: &TherapyConfig,
        screen_width: u32,
        screen_height: u32,
    ) -> Result<(), OverlayError>;

    /// Oculta el overlay y cierra la ventana.
    ///
    /// # Errores
    ///
    /// - [`OverlayError::NotActive`] si el overlay ya está inactivo.
    /// - [`OverlayError::CloseError`] si falla el cierre de la ventana.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// # use terapia_visual_domain::ports::OverlayPort;
    /// # async fn example(overlay: &mut dyn OverlayPort) {
    /// if let Err(e) = overlay.hide().await {
    ///     eprintln!("Failed to hide overlay: {}", e);
    /// }
    /// # }
    /// ```
    async fn hide(&mut self) -> Result<(), OverlayError>;

    /// Actualiza el overlay con una nueva configuración (colores, layout, opacidades).
    ///
    /// Si el overlay está activo, los cambios se aplican en tiempo real.
    /// Si el overlay está inactivo, esta operación no tiene efecto.
    ///
    /// # Argumentos
    ///
    /// * `config` - Nueva configuración de la terapia.
    /// * `screen_width` - Ancho de la pantalla en píxeles.
    /// * `screen_height` - Alto de la pantalla en píxeles.
    ///
    /// # Errores
    ///
    /// - [`OverlayError::UpdateError`] si falla la actualización de la ventana.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// # use terapia_visual_domain::ports::OverlayPort;
    /// # use terapia_visual_domain::domain::TherapyConfig;
    /// # async fn example(overlay: &mut dyn OverlayPort) {
    /// let mut config = TherapyConfig::default();
    /// // ... modificar config ...
    /// if let Err(e) = overlay.update_config(&config, 1920, 1080).await {
    ///     eprintln!("Failed to update overlay: {}", e);
    /// }
    /// # }
    /// ```
    async fn update_config(
        &mut self,
        config: &TherapyConfig,
        screen_width: u32,
        screen_height: u32,
    ) -> Result<(), OverlayError>;

    /// Devuelve `true` si el overlay está actualmente visible y activo.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// # use terapia_visual_domain::ports::OverlayPort;
    /// # fn example(overlay: &dyn OverlayPort) {
    /// if overlay.is_active() {
    ///     println!("Therapy is running");
    /// } else {
    ///     println!("Therapy is stopped");
    /// }
    /// # }
    /// ```
    fn is_active(&self) -> bool;
}
