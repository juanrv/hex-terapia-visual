//! # Caso de Uso: Iniciar Terapia de Lectura

use crate::{
    domain::reading_therapy_config::ReadingTherapyConfig,
    ports::{ReadingWindowError, ReadingWindowPort},
};

/// Inicia la terapia de lectura en una ventana separada.
///
/// Si la ventana ya está activa, se actualiza el contenido.
pub async fn start_reading_therapy(
    window: &mut dyn ReadingWindowPort,
    config: &ReadingTherapyConfig,
    html_content: &str,
) -> Result<(), ReadingWindowError> {
    window.show(config, html_content).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_cases::mocks::MockReadingWindow;

    #[tokio::test]
    async fn test_start_reading_therapy_ok() {
        let mut window = MockReadingWindow::default();
        let config = ReadingTherapyConfig::default();
        let html = "<p>Hola</p>";

        let result = start_reading_therapy(&mut window, &config, html).await;

        assert!(result.is_ok());
        assert!(window.is_active());
        assert!(window.show_called);
        assert_eq!(window.last_html.unwrap(), html);
    }

    #[tokio::test]
    async fn test_start_reading_therapy_fails() {
        let mut window = MockReadingWindow {
            should_fail: true, // Simular un fallo al abrir la ventana
            ..Default::default()
        };
        let config = ReadingTherapyConfig::default();

        let result = start_reading_therapy(&mut window, &config, "texto").await;
        assert!(result.is_err());
    }
}
