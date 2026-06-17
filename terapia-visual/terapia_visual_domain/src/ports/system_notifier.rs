//! # Puerto de Notificador del Sistema
//!
//! Define el contrato para mostrar notificaciones al usuario a través
//! del sistema operativo y actualizar el estado de la bandeja del sistema.
//!
//! Este puerto se utiliza para:
//! - Mostrar mensajes temporales al usuario (notificaciones).
//! - Cambiar el ícono y tooltip de la bandeja del sistema según el estado de la terapia.
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_domain::ports::{SystemNotifier, NotifierError};
//!
//! # async fn example(notifier: &dyn SystemNotifier) -> Result<(), NotifierError> {
//! notifier.show_message("Terapia", "Terapia iniciada").await?;
//! notifier.set_tray_state(true).await?;
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;

/// Errores que pueden ocurrir al mostrar notificaciones o actualizar la bandeja.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum NotifierError {
    /// Error al mostrar un mensaje en la bandeja del sistema.
    #[error("Failed to display tray message: {0}")]
    TrayError(String),

    /// Error al cargar o cambiar el ícono de la bandeja del sistema.
    #[error("Failed to load tray icon: {0}")]
    IconError(String),
}

/// Puerto para notificaciones al usuario a través del sistema operativo.
///
/// Este trait abstrae las operaciones de notificación y bandeja del sistema,
/// permitiendo que el dominio se comunique con el usuario sin conocer
/// los detalles de implementación de cada sistema operativo.
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::ports::{SystemNotifier, NotifierError};
///
/// # async fn example(notifier: &dyn SystemNotifier) -> Result<(), NotifierError> {
/// // Mostrar una notificación de éxito
/// notifier.show_message("Éxito", "Terapia iniciada").await?;
///
/// // Cambiar el estado de la bandeja a "activo"
/// notifier.set_tray_state(true).await?;
/// # Ok(())
/// # }
/// ```
#[async_trait]
pub trait SystemNotifier: Send + Sync {
    /// Muestra un mensaje temporal al usuario (notificación del sistema).
    ///
    /// # Argumentos
    ///
    /// * `title` - Título de la notificación.
    /// * `message` - Cuerpo del mensaje de la notificación.
    ///
    /// # Errores
    ///
    /// Devuelve [`NotifierError::TrayError`] si falla la visualización.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// # use terapia_visual_domain::ports::SystemNotifier;
    /// # async fn example(notifier: &dyn SystemNotifier) {
    /// notifier.show_message("Información", "Configuración guardada").await.unwrap();
    /// # }
    /// ```
    async fn show_message(&self, title: &str, message: &str) -> Result<(), NotifierError>;

    /// Cambia el estado visual de la bandeja del sistema (ícono y tooltip).
    ///
    /// Se utiliza para reflejar el estado de la terapia visual:
    /// - `active = true`: la terapia está activa (ícono verde, tooltip "Terapia Activa").
    /// - `active = false`: la terapia está inactiva (ícono gris, tooltip "Terapia Inactiva").
    ///
    /// # Argumentos
    ///
    /// * `active` - `true` si la terapia está activa, `false` si está inactiva.
    ///
    /// # Errores
    ///
    /// Devuelve [`NotifierError::IconError`] si falla la actualización del ícono.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// # use terapia_visual_domain::ports::SystemNotifier;
    /// # async fn example(notifier: &dyn SystemNotifier) {
    /// // Cambiar a estado activo
    /// notifier.set_tray_state(true).await.unwrap();
    ///
    /// // Cambiar a estado inactivo
    /// notifier.set_tray_state(false).await.unwrap();
    /// # }
    /// ```
    async fn set_tray_state(&self, active: bool) -> Result<(), NotifierError>;
}
