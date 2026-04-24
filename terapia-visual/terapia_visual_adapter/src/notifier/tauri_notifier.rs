use async_trait::async_trait;
use tauri::AppHandle;
use tauri_plugin_notification::NotificationExt;
use terapia_visual_domain::ports::{NotifierError, SystemNotifier};
use tracing::info;

pub struct TauriSystemNotifier {
    app_handle: AppHandle,
}

impl TauriSystemNotifier {
    pub fn new(app_handle: AppHandle) -> Self {
        Self { app_handle }
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
        if let Some(tray) = self.app_handle.tray_by_id("main") {
            let new_tooltip = if active {
                "Terapia Activa"
            } else {
                "Terapia Inactiva"
            };
            tray.set_tooltip(Some(new_tooltip))
                .map_err(|e| NotifierError::IconError(e.to_string()))?;

            info!("Tooltip de la bandeja actualizado: {}", new_tooltip);
        }
        Ok(())
    }
}
