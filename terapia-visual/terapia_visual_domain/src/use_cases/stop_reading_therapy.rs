use crate::ports::{ReadingWindowError, ReadingWindowPort};

pub async fn stop_reading_therapy(
    window: &mut dyn ReadingWindowPort,
) -> Result<(), ReadingWindowError> {
    if window.is_active() {
        window.hide().await
    } else {
        Err(ReadingWindowError::NotActive)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_cases::mocks::MockReadingWindow;

    #[tokio::test]
    async fn test_stop_reading_therapy_ok() {
        let mut window = MockReadingWindow {
            active: true,
            ..Default::default()
        };
        let result = stop_reading_therapy(&mut window).await;
        assert!(result.is_ok());
        assert!(!window.is_active());
    }

    #[tokio::test]
    async fn test_stop_reading_therapy_not_active() {
        let mut window = MockReadingWindow::default();
        let result = stop_reading_therapy(&mut window).await;
        assert_eq!(result.unwrap_err(), ReadingWindowError::NotActive);
    }

    #[tokio::test]
    async fn test_stop_reading_therapy_fails() {
        let mut window = MockReadingWindow {
            active: true,
            should_fail: true, // Simular fallo de ventana
            ..Default::default()
        };
        let result = stop_reading_therapy(&mut window).await;
        assert!(result.is_err());
    }
}
