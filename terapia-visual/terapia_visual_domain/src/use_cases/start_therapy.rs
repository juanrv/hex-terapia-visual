use crate::{
    domain::TherapyConfig,
    ports::{OverlayError, OverlayPort, overlay},
};

/// Inicia la terapia visual.
pub async fn start_therapy(
    overlay: &mut dyn OverlayPort,
    config: &TherapyConfig,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), overlay::OverlayError> {
    if overlay.is_active() {
        Err(OverlayError::AlreadyActive)
    } else {
        // Si el overlay no está activo, se muestra con la configuracion inicial
        overlay.show(config, screen_width, screen_height).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::*;
    use crate::use_cases::mocks::MockOverlay;

    fn sample_config() -> TherapyConfig {
        TherapyConfig::new(
            TherapyType::ColorDivision,
            Layout::Vertical,
            vec![
                ZoneConfig {
                    color: Color::new("#FF0000").unwrap(),
                    opacity: Opacity::new(0.8).unwrap(),
                },
                ZoneConfig {
                    color: Color::new("#0000FF").unwrap(),
                    opacity: Opacity::new(0.6).unwrap(),
                },
            ],
        )
        .unwrap()
    }

    #[tokio::test]
    async fn test_start_therapy_ok() {
        let mut overlay = MockOverlay::default();
        let config = sample_config();
        let result = start_therapy(&mut overlay, &config, 1920, 1080).await;
        assert!(result.is_ok());
        assert!(overlay.is_active());
        assert!(overlay.show_called);
    }

    #[tokio::test]
    async fn test_start_therapy_already_active() {
        let mut overlay = MockOverlay {
            active: true,
            ..Default::default()
        };
        let config = sample_config();
        let result = start_therapy(&mut overlay, &config, 1920, 1080).await;
        assert_eq!(result.unwrap_err(), OverlayError::AlreadyActive);
    }
}
