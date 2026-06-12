use async_trait::async_trait;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;
use terapia_visual_domain::ports::{NotifierError, SystemNotifier};
use tracing::{info, warn};

use crate::messages;

pub struct TauriSystemNotifier {
    app_handle: AppHandle,
    icon_inactive_bytes: &'static [u8],
    icon_active_bytes: &'static [u8],
}

impl TauriSystemNotifier {
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
