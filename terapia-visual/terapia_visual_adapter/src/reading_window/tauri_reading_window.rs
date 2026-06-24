use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use async_trait::async_trait;
use serde::Serialize;
use tauri::{Emitter, WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use terapia_visual_domain::{
    domain::reading_therapy_config::ReadingTherapyConfig,
    ports::{ReadingWindowError, ReadingWindowPort},
};
use tracing::info;

/// Paquete de datos que se envia al frontend por eventos IPC
#[derive(Serialize, Clone)]
struct ReadingPayload {
    config: ReadingTherapyConfig,
    html_content: String,
}

pub struct TauriReadingWindow {
    window: Option<WebviewWindow>,
    is_active: Arc<AtomicBool>,
    app_handle: tauri::AppHandle,
    current_html: Option<String>,
}

impl TauriReadingWindow {
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self {
            window: None,
            is_active: Arc::new(AtomicBool::new(false)),
            app_handle,
            current_html: None,
        }
    }

    /// Emite los datos actualizados a la ventana de lectura de forma segura.
    fn emit_udpate(&self, config: &ReadingTherapyConfig) -> Result<(), ReadingWindowError> {
        if let Some(window) = &self.window {
            let content = self.current_html.clone().unwrap_or_default();
            let payload = ReadingPayload {
                config: config.clone(),
                html_content: content,
            };

            window
                .emit("update-reading-view", payload)
                .map_err(|e| ReadingWindowError::UpdateError(e.to_string()))?;
        }
        Ok(())
    }
}

#[async_trait]
impl ReadingWindowPort for TauriReadingWindow {
    async fn show(
        &mut self,
        config: &ReadingTherapyConfig,
        html_content: &str,
    ) -> Result<(), ReadingWindowError> {
        self.current_html = Some(html_content.to_string());

        // Si la ventana ya existe, se recicla y se le envian los datos
        if let Some(window) = &self.window {
            window
                .show()
                .map_err(|e| ReadingWindowError::CreationError(e.to_string()))?;
            let _ = window.set_focus();

            self.emit_udpate(config)?;
            self.is_active.store(true, Ordering::SeqCst);
            return Ok(());
        }

        // Crear una ventana ormal de windows, apuntando al frontend
        let window = WebviewWindowBuilder::new(
            &self.app_handle,
            "reading_window",
            WebviewUrl::App("src/reading.html".into()),
        )
        .title("Terapia de Lectura")
        .inner_size(1024.0, 768.0)
        .center()
        .build()
        .map_err(|e| ReadingWindowError::CreationError(e.to_string()))?;

        // Escuchar cuando el usuario cierra la ventana con la "X"
        let is_active_clone = self.is_active.clone();
        window.on_window_event(move |event| {
            if let tauri::WindowEvent::Destroyed = event {
                is_active_clone.store(false, Ordering::SeqCst);
                tracing::info!("Reading window destroyed by user");
            }
        });

        self.window = Some(window);
        self.is_active.store(true, Ordering::SeqCst);

        // Esperar un instante a que el fronten cargue antes de emitir los datos
        tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
        self.emit_udpate(config)?;

        info!("Reading window created and shown");
        Ok(())
    }

    async fn hide(&mut self) -> Result<(), ReadingWindowError> {
        if let Some(window) = &self.window {
            window
                .hide()
                .map_err(|e| ReadingWindowError::CloseError(e.to_string()))?;

            self.is_active.store(false, Ordering::SeqCst);
            info!("Reading window hidded");
            Ok(())
        } else {
            Err(ReadingWindowError::NotActive)
        }
    }

    async fn update_config(
        &mut self,
        config: &ReadingTherapyConfig,
    ) -> Result<(), ReadingWindowError> {
        self.emit_udpate(config)
    }

    fn is_active(&self) -> bool {
        self.is_active.load(Ordering::SeqCst)
    }
}
