use async_trait::async_trait;

/// Errores que pueden ocurrir en las notificaciones del sistema.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum NotifierError {
    #[error("Failed to display tray message: {0}")]
    TrayError(String),
    #[error("Failed to load tray icon: {0}")]
    IconError(String),
}

/// Puerto para notificaciones al usurio a través del sistema operativo (tray, notificaciones, etc).
#[async_trait]
pub trait SystemNotifier: Send + Sync {
    /// Muestra un mensaje temporal en la bandeja del sistema
    async fn show_message(&self, title: &str, message: &str) -> Result<(), NotifierError>;

    /// Cambia el icono de la bandeja del sistema para indicar el estado de la terapia (activo/inactivo).
    async fn set_tray_state(&self, active: bool) -> Result<(), NotifierError>;
}
