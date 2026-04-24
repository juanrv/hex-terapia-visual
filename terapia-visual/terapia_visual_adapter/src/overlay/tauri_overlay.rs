// src/overlay/tauri_overlay.rs
use async_trait::async_trait;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tauri::{WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tracing::info;

use terapia_visual_domain::domain::{TherapyConfig, Zone};
use terapia_visual_domain::ports::{OverlayError, OverlayPort};

pub struct TauriOverlay {
    window: Option<WebviewWindow>,
    is_active: Arc<AtomicBool>,
    app_handle: tauri::AppHandle,
}

impl TauriOverlay {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self {
            window: None,
            is_active: Arc::new(AtomicBool::new(false)),
            app_handle,
        }
    }

    /// Genera el contenido HTML para el overlay
    fn generate_overlay_html(zones: &[Zone]) -> String {
        let mut styles = String::new();
        for (i, zone) in zones.iter().enumerate() {
            let rect = zone.rect();
            let color = zone.color().as_str();
            let opacity = zone.opacity().value();
            styles.push_str(&format!(
                r#"
                .zone-{} {{
                    position: absolute;
                    left: {}px;
                    top: {}px;
                    width: {}px;
                    height: {}px;
                    background-color: {};
                    opacity: {};
                    pointer-events: none;
                }}
                "#,
                i, rect.x, rect.y, rect.width, rect.height, color, opacity
            ));
        }

        format!(
            r#"<!DOCTYPE html>
            <html>
            <head>
                <meta charset="UTF-8">
                <style>
                    body {{
                        margin: 0;
                        padding: 0;
                        overflow: hidden;
                        background-color: transparent;
                    }}
                    {}
                </style>
            </head>
            <body>
                {}
            </body>
            </html>"#,
            styles,
            zones
                .iter()
                .enumerate()
                .map(|(i, _)| format!(r#"<div class="zone-{}"></div>"#, i))
                .collect::<Vec<_>>()
                .join("\n")
        )
    }

    /// Inyecta HTML en la ventana existente
    fn update_window_content(
        &self,
        window: &WebviewWindow,
        config: &TherapyConfig,
        screen_width: u32,
        screen_height: u32,
    ) -> Result<(), OverlayError> {
        let html = Self::generate_overlay_html(&config.generate_zones(screen_width, screen_height));
        let js = format!("document.body.innerHTML = `{}`;", html.replace('`', "\\`"));
        window
            .eval(&js)
            .map_err(|e| OverlayError::UpdateError(e.to_string()))
    }
}

#[async_trait]
impl OverlayPort for TauriOverlay {
    async fn show(
        &mut self,
        config: &TherapyConfig,
        screen_width: u32,
        screen_height: u32,
    ) -> Result<(), OverlayError> {
        if self.is_active() {
            return Err(OverlayError::AlreadyActive);
        }

        // Crear nueva ventana
        let window = WebviewWindowBuilder::new(
            &self.app_handle,
            "therapy_overlay",
            WebviewUrl::App("overlay.html".into()),
        )
        .title("Visual Therapy Overlay")
        .inner_size(screen_width as f64, screen_height as f64)
        .position(0.0, 0.0)
        .decorations(false)
        .always_on_top(true)
        .transparent(true)
        .skip_taskbar(true)
        .resizable(false)
        .build()
        .map_err(|e| OverlayError::CreationError(e.to_string()))?;

        // Click-through
        window
            .set_ignore_cursor_events(true)
            .map_err(|e| OverlayError::CreationError(e.to_string()))?;

        // Inyectar contenido HTML
        self.update_window_content(&window, config, screen_width, screen_height)?;

        // Mostrar ventana
        window
            .show()
            .map_err(|e| OverlayError::CreationError(e.to_string()))?;

        // Guardar y marcar activo
        self.window = Some(window);
        self.is_active.store(true, Ordering::SeqCst);
        info!("Tauri overlay shown");
        Ok(())
    }

    async fn hide(&mut self) -> Result<(), OverlayError> {
        if let Some(window) = self.window.take() {
            window
                .close()
                .map_err(|e| OverlayError::CloseError(e.to_string()))?;
            self.is_active.store(false, Ordering::SeqCst);
            info!("Tauri overlay hidden");
            Ok(())
        } else {
            Err(OverlayError::NotActive)
        }
    }

    async fn update_config(
        &mut self,
        config: &TherapyConfig,
        screen_width: u32,
        screen_height: u32,
    ) -> Result<(), OverlayError> {
        if let Some(window) = &self.window {
            self.update_window_content(window, config, screen_width, screen_height)?;
        }
        Ok(())
    }

    fn is_active(&self) -> bool {
        self.is_active.load(Ordering::SeqCst)
    }
}
