//! # Adaptador de Notificaciones para Tauri
//!
//! Este módulo implementa el puerto [`SystemNotifier`] utilizando las capacidades
//! nativas de Tauri para mostrar notificaciones del sistema y controlar la
//! bandeja (system tray).
//!
//! # Características
//!
//! - **Notificaciones del sistema**: Muestra mensajes al usuario utilizando
//!   `tauri-plugin-notification`.
//! - **Control de la bandeja**: Cambia el ícono y el tooltip de la bandeja
//!   según el estado de la terapia (activa/inactiva).
//! - **Iconos empaquetados**: Los iconos se incrustan en el binario usando
//!   `include_bytes!`, lo que evita dependencias de archivos externos.
//!
//! # Ejemplo de uso
//!
//! ```no_run
//! use terapia_visual_adapter::notifier::TauriSystemNotifier;
//! use terapia_visual_domain::ports::SystemNotifier;
//! use tauri::AppHandle;
//!
//! # fn example(app_handle: AppHandle) {
//! // Crear el notificador con iconos empaquetados
//! # let icon_inactive = &[];
//! # let icon_active = &[];
//! let notifier = TauriSystemNotifier::new(app_handle, icon_inactive, icon_active);
//!
//! // Mostrar una notificación
//! # async fn test(notifier: &TauriSystemNotifier) -> Result<(), Box<dyn std::error::Error>> {
//! notifier.show_message("Terapia", "Terapia iniciada").await?;
//! # Ok(())
//! # }
//! # }
//! ```

use async_trait::async_trait;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;
use terapia_visual_domain::ports::{NotifierError, SystemNotifier};
use tracing::{info, warn};

use crate::messages;

/// Adaptador de notificaciones del sistema para Tauri.
///
/// Implementa el puerto [`SystemNotifier`] utilizando las APIs nativas de Tauri.
///
/// # Campos
///
/// * `app_handle` - El handle de la aplicación Tauri, necesario para acceder
///   a la bandeja y mostrar notificaciones.
/// * `icon_inactive_bytes` - Los bytes del ícono que se muestra cuando la
///   terapia está inactiva.
/// * `icon_active_bytes` - Los bytes del ícono que se muestra cuando la
///   terapia está activa.
///
/// # Ejemplos
///
/// ```no_run
/// use terapia_visual_adapter::notifier::TauriSystemNotifier;
/// use tauri::AppHandle;
///
/// # fn example(app_handle: AppHandle) {
/// # let icon_inactive = &[];
/// # let icon_active = &[];
/// let notifier = TauriSystemNotifier::new(
///     app_handle,
///     icon_inactive,
///     icon_active,
/// );
/// # }
/// ```
pub struct TauriSystemNotifier {
    app_handle: AppHandle,
    icon_inactive_bytes: &'static [u8],
    icon_active_bytes: &'static [u8],
}

impl TauriSystemNotifier {
    /// Crea un nuevo adaptador de notificaciones para Tauri.
    ///
    /// # Argumentos
    ///
    /// * `app_handle` - El handle de la aplicación Tauri.
    /// * `icon_inactive_bytes` - Bytes del ícono para el estado inactivo.
    /// * `icon_active_bytes` - Bytes del ícono para el estado activo.
    ///
    /// # Ejemplos
    ///
    /// ```no_run
    /// use terapia_visual_adapter::notifier::TauriSystemNotifier;
    /// use tauri::AppHandle;
    ///
    /// # fn example(app_handle: AppHandle) {
    /// # let icon_inactive = &[];
    /// # let icon_active = &[];
    /// let notifier = TauriSystemNotifier::new(
    ///     app_handle,
    ///     icon_inactive,
    ///     icon_active,
    /// );
    /// # }
    /// ```
    pub fn new(
        app_handle: AppHandle,
        icon_inactive_bytes: &'static [u8],
        icon_active_bytes: &'static [u8],
    ) -> Self {
        Self {
            app_handle,
            icon_inactive_bytes,
            icon_active_bytes,
        }
    }
}

#[async_trait]
impl SystemNotifier for TauriSystemNotifier {
    /// Muestra una notificación del sistema.
    ///
    /// Utiliza `tauri-plugin-notification` para mostrar un mensaje emergente
    /// al usuario.
    ///
    /// # Argumentos
    ///
    /// * `title` - Título de la notificación.
    /// * `message` - Cuerpo del mensaje.
    ///
    /// # Errores
    ///
    /// Devuelve [`NotifierError::TrayError`] si falla la visualización.
    ///
    /// # Ejemplos
    ///
    /// ```no_run
    /// # use terapia_visual_adapter::notifier::TauriSystemNotifier;
    /// # use terapia_visual_domain::ports::SystemNotifier;
    /// # async fn example(notifier: &TauriSystemNotifier) -> Result<(), Box<dyn std::error::Error>> {
    /// notifier.show_message("Éxito", "Configuración guardada").await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn show_message(&self, title: &str, message: &str) -> Result<(), NotifierError> {
        // Usar notificación del sistema (Windows toast / Linux notify)
        self.app_handle
            .notification()
            .builder()
            .title(title)
            .body(message)
            .show()
            .map_err(|e| NotifierError::TrayError(e.to_string()))?;

        info!("Mostrando notificacion: {} - {}", title, message);
        Ok(())
    }

    /// Cambia el estado visual de la bandeja (tooltip e ícono).
    ///
    /// Esta función actualiza:
    /// - El tooltip de la bandeja (traducido según el idioma configurado).
    /// - El ícono de la bandeja (activo/inactivo).
    ///
    /// # Argumentos
    ///
    /// * `active` - `true` para estado activo, `false` para inactivo.
    ///
    /// # Errores
    ///
    /// Devuelve [`NotifierError::IconError`] si no se encuentra la bandeja
    /// o falla la actualización del ícono.
    ///
    /// # Ejemplos
    ///
    /// ```no_run
    /// # use terapia_visual_adapter::notifier::TauriSystemNotifier;
    /// # use terapia_visual_domain::ports::SystemNotifier;
    /// # async fn example(notifier: &TauriSystemNotifier) -> Result<(), Box<dyn std::error::Error>> {
    /// // Cambiar a estado activo
    /// notifier.set_tray_state(true).await?;
    ///
    /// // Cambiar a estado inactivo
    /// notifier.set_tray_state(false).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn set_tray_state(&self, active: bool) -> Result<(), NotifierError> {
        let tray = self.app_handle.tray_by_id("main").ok_or_else(|| {
            let msg = "Tray icon with id 'main' was not found";
            warn!("{}", msg);
            NotifierError::IconError(msg.to_string())
        })?;

        // Actualizar el tooltip
        let new_tooltip = if active {
            messages::tooltip_therapy_active()
        } else {
            messages::tooltip_therapy_inactive()
        };

        tray.set_tooltip(Some(new_tooltip))
            .map_err(|e| NotifierError::IconError(format!("Error setting tooltip: {}", e)))?;

        // Actualizar icono
        let bytes = if active {
            self.icon_active_bytes
        } else {
            self.icon_inactive_bytes
        };

        // Convertir bytes inscrustado a una imagen
        if let Ok(icon) = tauri::image::Image::from_bytes(bytes) {
            tray.set_icon(Some(icon))
                .map_err(|e| NotifierError::IconError(format!("Error setting icon: {}", e)))?;
        }

        info!("Tray status updated to: {}", active);
        Ok(())
    }
}
