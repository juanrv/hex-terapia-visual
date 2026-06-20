//! # Adaptador de Overlay para Tauri
//!
//! Este módulo implementa el puerto [`OverlayPort`] utilizando las capacidades
//! nativas de Tauri para crear y controlar una ventana de superposición
//! transparente a pantalla completa.
//!
//! # Características
//!
//! - **Ventana transparente**: Fondo transparente con `transparent(true)`.
//! - **Click-through**: Permite que los clics del ratón atraviesen la ventana
//!   con `set_ignore_cursor_events(true)`.
//! - **Pantalla completa**: La ventana se posiciona en (0, 0) con el tamaño
//!   de la pantalla.
//! - **Siempre encima**: Se mantiene sobre todas las demás ventanas con
//!   `always_on_top(true)`.
//! - **Actualización en tiempo real**: El contenido HTML se actualiza mediante
//!   `eval` cuando cambia la configuración.
//! - **Reutilización de ventana**: Si la ventana ya existe, se actualiza en
//!   lugar de recrearla.
//!
//! # Flujo típico
//!
//! 1. Se llama a `show()` con la configuración y las dimensiones de pantalla.
//! 2. La ventana se crea (o reutiliza) y se inyecta el HTML con las zonas de color.
//! 3. Al llamar a `update_config()`, se regenera el HTML y se actualiza la ventana.
//! 4. Al llamar a `hide()`, la ventana se oculta (pero no se destruye).
//!
//! # Ejemplos
//!
//! ```no_run
//! use terapia_visual_adapter::overlay::TauriOverlay;
//! use terapia_visual_domain::ports::OverlayPort;
//! use terapia_visual_domain::domain::TherapyConfig;
//! use tauri::AppHandle;
//!
//! # async fn example(app_handle: AppHandle) -> Result<(), Box<dyn std::error::Error>> {
//! let mut overlay = TauriOverlay::new(app_handle);
//! let config = TherapyConfig::default();
//!
//! // Mostrar el overlay
//! overlay.show(&config, 1920, 1080).await?;
//!
//! // Actualizar la configuración (cambiar colores, layout, etc.)
//! // let new_config = ...;
//! // overlay.update_config(&new_config, 1920, 1080).await?;
//!
//! // Ocultar el overlay
//! overlay.hide().await?;
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;
use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};
use tauri::{WebviewUrl, WebviewWindow, WebviewWindowBuilder};
use tracing::info;
use url::Url;

use terapia_visual_domain::domain::{OverlayTherapyConfig, Zone};
use terapia_visual_domain::ports::{OverlayError, OverlayPort};

/// Adaptador de overlay para Tauri.
///
/// Implementa el puerto [`OverlayPort`] utilizando ventanas nativas de Tauri.
///
/// # Campos
///
/// * `window` - La ventana de overlay (opcional, se crea en `show()`).
/// * `is_active` - Indica si el overlay está actualmente visible.
/// * `app_handle` - El handle de la aplicación Tauri, necesario para crear ventanas.
///
/// # Ejemplos
///
/// ```no_run
/// use terapia_visual_adapter::overlay::TauriOverlay;
/// use tauri::AppHandle;
///
/// # fn example(app_handle: AppHandle) {
/// let overlay = TauriOverlay::new(app_handle);
/// # }
/// ```
pub struct TauriOverlay {
    window: Option<WebviewWindow>,
    is_active: Arc<AtomicBool>,
    app_handle: tauri::AppHandle,
}

impl TauriOverlay {
    /// Crea un nuevo adaptador de overlay para Tauri.
    ///
    /// # Argumentos
    ///
    /// * `app_handle` - El handle de la aplicación Tauri.
    ///
    /// # Ejemplos
    ///
    /// ```no_run
    /// use terapia_visual_adapter::overlay::TauriOverlay;
    /// use tauri::AppHandle;
    ///
    /// # fn example(app_handle: AppHandle) {
    /// let overlay = TauriOverlay::new(app_handle);
    /// # }
    /// ```
    pub fn new(app_handle: tauri::AppHandle) -> Self {
        Self {
            window: None,
            is_active: Arc::new(AtomicBool::new(false)),
            app_handle,
        }
    }

    /// Genera el contenido HTML para el overlay.
    ///
    /// # Argumentos
    ///
    /// * `zones` - Las zonas de color a dibujar.
    ///
    /// # Retorno
    ///
    /// Un string HTML con las zonas renderizadas como elementos `div` con
    /// posicionamiento absoluto.
    ///
    /// # Ejemplo de HTML generado
    ///
    /// ```html
    /// <!DOCTYPE html>
    /// <html>
    /// <head>
    ///     <style>
    ///         body { margin: 0; overflow: hidden; background-color: transparent; }
    ///         .zone-0 { position: absolute; left: 0px; top: 0px; width: 960px; height: 1080px; background-color: #FF0000; opacity: 0.8; pointer-events: none; }
    ///     </style>
    /// </head>
    /// <body>
    ///     <div class="zone-0"></div>
    /// </body>
    /// </html>
    /// ```
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

    /// Inyecta HTML en la ventana existente.
    ///
    /// # Argumentos
    ///
    /// * `window` - La ventana donde inyectar el HTML.
    /// * `config` - La configuración de la terapia.
    /// * `screen_width` - Ancho de la pantalla en píxeles.
    /// * `screen_height` - Alto de la pantalla en píxeles.
    ///
    /// # Errores
    ///
    /// Devuelve [`OverlayError::UpdateError`] si falla la inyección del HTML.
    fn update_window_content(
        &self,
        window: &WebviewWindow,
        config: &OverlayTherapyConfig,
        screen_width: u32,
        screen_height: u32,
    ) -> Result<(), OverlayError> {
        let html = Self::generate_overlay_html(&config.generate_zones(screen_width, screen_height));
        let js = format!("document.body.innerHTML = `{}`;", html.replace('`', "\\`"));
        window
            .eval(&js)
            .map_err(|e| OverlayError::UpdateError(e.to_string()))?;
        Ok(())
    }
}

/// Implementación de la interfaz `OverlayPort` para TauriOverlay
#[async_trait]
impl OverlayPort for TauriOverlay {
    /// Muestra el overlay con la configuración dada.
    ///
    /// Si el overlay ya está activo, devuelve [`OverlayError::AlreadyActive`].
    ///
    /// # Comportamiento
    ///
    /// 1. Si la ventana ya existe, se actualiza su contenido y se muestra.
    /// 2. Si no existe, se crea una nueva ventana transparente a pantalla completa.
    /// 3. Se inyecta el HTML con las zonas de color.
    /// 4. Se aplica click-through con `set_ignore_cursor_events(true)`.
    ///
    /// # Argumentos
    ///
    /// * `config` - Configuración de la terapia.
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
    /// ```no_run
    /// # use terapia_visual_adapter::overlay::TauriOverlay;
    /// # use terapia_visual_domain::ports::OverlayPort;
    /// # use terapia_visual_domain::domain::TherapyConfig;
    /// # async fn example(overlay: &mut TauriOverlay) -> Result<(), Box<dyn std::error::Error>> {
    /// let config = TherapyConfig::default();
    /// overlay.show(&config, 1920, 1080).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn show(
        &mut self,
        config: &OverlayTherapyConfig,
        screen_width: u32,
        screen_height: u32,
    ) -> Result<(), OverlayError> {
        if self.is_active() {
            return Err(OverlayError::AlreadyActive);
        }

        // Si la ventana ya existe se actualiza
        if let Some(window) = &self.window {
            self.update_window_content(window, config, screen_width, screen_height)?;
            window
                .show()
                .map_err(|e| OverlayError::CreationError(e.to_string()))?;
            self.is_active.store(true, Ordering::SeqCst);
            tracing::info!("Tauri overlay shown (reused)");
            return Ok(());
        }

        let blank_url = Url::parse("local://blank").unwrap();

        // Crear nueva ventana
        let window = WebviewWindowBuilder::new(
            &self.app_handle,
            "therapy_overlay",
            WebviewUrl::CustomProtocol(blank_url),
        )
        .title("Visual Therapy Overlay")
        .inner_size(screen_width as f64, screen_height as f64)
        .position(0.0, 0.0)
        .decorations(false) // Sin Decoraciones
        .always_on_top(true) // Siempre encima
        .transparent(true) // Fondo transparente
        .skip_taskbar(true) // No aparece en la barra de tareas
        .resizable(false) // No se puede redimensionar
        .visible(false) // Inicialmente invisible
        .build()
        .map_err(|e| OverlayError::CreationError(e.to_string()))?;

        // Click-through
        window
            .set_ignore_cursor_events(true)
            .map_err(|e| OverlayError::CreationError(e.to_string()))?;

        // Pausa de seguridad para el renderizado inicial
        tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;

        // Inyectar HTML
        self.update_window_content(&window, config, screen_width, screen_height)?;

        // Pausa para que el DOM se actualice
        tokio::time::sleep(tokio::time::Duration::from_millis(50)).await;

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

    /// Oculta el overlay (oculta la ventana, no la destruye).
    ///
    /// # Errores
    ///
    /// - [`OverlayError::NotActive`] si el overlay ya está inactivo.
    /// - [`OverlayError::CloseError`] si falla al ocultar la ventana.
    ///
    /// # Ejemplos
    ///
    /// ```no_run
    /// # use terapia_visual_adapter::overlay::TauriOverlay;
    /// # use terapia_visual_domain::ports::OverlayPort;
    /// # async fn example(overlay: &mut TauriOverlay) -> Result<(), Box<dyn std::error::Error>> {
    /// overlay.hide().await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn hide(&mut self) -> Result<(), OverlayError> {
        if let Some(window) = &self.window {
            window
                .hide()
                .map_err(|e| OverlayError::CloseError(e.to_string()))?;
            self.is_active.store(false, Ordering::SeqCst);
            info!("Tauri overlay hidden");
            Ok(())
        } else {
            Err(OverlayError::NotActive)
        }
    }

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
    /// ```no_run
    /// # use terapia_visual_adapter::overlay::TauriOverlay;
    /// # use terapia_visual_domain::ports::OverlayPort;
    /// # use terapia_visual_domain::domain::{TherapyConfig, Layout, Color, Opacity, ZoneConfig, TherapyType};
    /// # async fn example(overlay: &mut TauriOverlay) -> Result<(), Box<dyn std::error::Error>> {
    /// // Configuración inicial
    /// let mut config = TherapyConfig::default();
    /// // Cambiar colores y layout en tiempo real
    /// let mut new_config = config.clone();
    /// new_config.change_layout(Layout::Horizontal);
    /// new_config.update_zone_color(0, Color::new("#00FF00")?)?;
    /// new_config.update_zone_color(1, Color::new("#FF00FF")?)?;
    ///
    /// // Actualizar el overlay con los nuevos valores
    /// overlay.update_config(&new_config, 1920, 1080).await?;
    /// # Ok(())
    /// # }
    /// ```
    async fn update_config(
        &mut self,
        config: &OverlayTherapyConfig,
        screen_width: u32,
        screen_height: u32,
    ) -> Result<(), OverlayError> {
        if let Some(window) = &self.window {
            self.update_window_content(window, config, screen_width, screen_height)?;
        }
        Ok(())
    }

    /// Devuelve `true` si el overlay está actualmente visible y activo.
    ///
    /// # Ejemplos
    ///
    /// ```no_run
    /// # use terapia_visual_adapter::overlay::TauriOverlay;
    /// # use terapia_visual_domain::ports::OverlayPort;
    /// # fn example(overlay: &TauriOverlay) {
    /// if overlay.is_active() {
    ///     println!("Terapia activa");
    /// } else {
    ///     println!("Terapia inactiva");
    /// }
    /// # }
    /// ```
    fn is_active(&self) -> bool {
        self.is_active.load(Ordering::SeqCst)
    }
}
