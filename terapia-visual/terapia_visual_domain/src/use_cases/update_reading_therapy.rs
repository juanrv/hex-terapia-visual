use crate::{
    domain::reading_therapy_config::ReadingTherapyConfig,
    ports::{ConfigStorage, ReadingWindowError, ReadingWindowPort, StorageError},
};

#[derive(Debug, thiserror::Error)]
pub enum UpdateReadingConfigError {
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error(transparent)]
    Window(#[from] ReadingWindowError),
}

/// Actualiza la configuracion y la aplica a la ventana si esta abierta
pub async fn update_reading_therapy(
    window: &mut dyn ReadingWindowPort,
    storage: &dyn ConfigStorage<ReadingTherapyConfig>,
    new_config: &ReadingTherapyConfig,
) -> Result<(), UpdateReadingConfigError> {
    storage.save(new_config).await?;

    if window.is_active() {
        window.update_config(new_config).await?;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::use_cases::mocks::{MockReadingConfigStorage, MockReadingWindow};

    #[tokio::test]
    async fn test_update_reading_config_active() {
        let mut window = MockReadingWindow {
            active: true,
            ..Default::default()
        };
        let storage = MockReadingConfigStorage::default();
        let config = ReadingTherapyConfig::default();

        let result = update_reading_therapy(&mut window, &storage, &config).await;
        assert!(result.is_ok());
        assert!(window.update_config_called);
    }

    #[tokio::test]
    async fn test_update_reading_config_inactive() {
        let mut window = MockReadingWindow::default();
        let storage = MockReadingConfigStorage::default();
        let config = ReadingTherapyConfig::default();

        let result = update_reading_therapy(&mut window, &storage, &config).await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_update_reading_config_storage_fails() {
        let mut window = MockReadingWindow::default();
        // simular que falla al guardar en disco
        let storage = MockReadingConfigStorage {
            should_fail_save: true,
            ..Default::default()
        };
        let config = ReadingTherapyConfig::default();

        let result = update_reading_therapy(&mut window, &storage, &config).await;
        assert!(matches!(
            result.unwrap_err(),
            UpdateReadingConfigError::Storage(_)
        ));
    }

    #[tokio::test]
    async fn test_update_reading_config_window_fails() {
        // Simular que la ventana falla al actualizarse
        let mut window = MockReadingWindow {
            active: true,
            should_fail: true,
            ..Default::default()
        };
        let storage = MockReadingConfigStorage::default();
        let config = ReadingTherapyConfig::default();

        let result = update_reading_therapy(&mut window, &storage, &config).await;
        assert!(matches!(
            result.unwrap_err(),
            UpdateReadingConfigError::Window(_)
        ));
    }
}
