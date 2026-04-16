use crate::ports::{OverlayError, OverlayPort};

/// Detiene la terapia (Oculta el overlay)
pub async fn stop_therapy(overlay: &mut dyn OverlayPort) -> Result<(), OverlayError> {
    if overlay.is_active() {
        overlay.hide().await
    } else {
        Err(OverlayError::NotActive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_cases::mocks::MockOverlay;

    #[tokio::test]
    async fn test_stop_therapy_ok() {
        let mut overlay = MockOverlay {
            active: true,
            ..Default::default()
        };
        let result = stop_therapy(&mut overlay).await;
        assert!(result.is_ok());
        assert!(!overlay.is_active());
        assert!(overlay.hide_called);
    }

    #[tokio::test]
    async fn test_stop_therapy_not_active() {
        let mut overlay = MockOverlay::default();
        let result = stop_therapy(&mut overlay).await;
        assert_eq!(result.unwrap_err(), OverlayError::NotActive);
    }
}
