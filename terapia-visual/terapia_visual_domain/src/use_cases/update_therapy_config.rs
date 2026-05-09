use crate::{
    domain::TherapyConfig,
    ports::{ConfigStorage, OverlayError, OverlayPort, StorageError},
};

#[derive(Debug, thiserror::Error)]
pub enum UpdateConfigError {
    #[error(transparent)]
    Storage(#[from] StorageError),
    #[error(transparent)]
    Overlay(#[from] OverlayError),
}

/// Actualiza la configuracion. Si la terapia esta activa, tambien actualiza el overlay.
pub async fn update_therapy_config(
    overlay: &mut dyn OverlayPort,
    storage: &dyn ConfigStorage<TherapyConfig>,
    new_config: &TherapyConfig,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), UpdateConfigError> {
    // Guardar en almacenamiento
    storage
        .save(new_config)
        .await
        .map_err(UpdateConfigError::Storage)?;

    // Si el overlay está activo, actualizarlo con la nueva configuración
    if overlay.is_active() {
        overlay
            .update_config(new_config, screen_width, screen_height)
            .await
            .map_err(UpdateConfigError::Overlay)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::*;
    use crate::use_cases::mocks::{MockOverlay, MockTherapyConfigStorage};

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
    async fn test_update_config_when_inactive() {
        let mut overlay = MockOverlay::default();
        let storage = MockTherapyConfigStorage::default();
        let new_config = sample_config();
        let result = update_therapy_config(&mut overlay, &storage, &new_config, 1920, 1080).await;
        assert!(result.is_ok());
        assert!(!overlay.update_config_called);
    }

    #[tokio::test]
    async fn test_update_config_when_active() {
        let mut overlay = MockOverlay {
            active: true,
            ..Default::default()
        };
        let storage = MockTherapyConfigStorage::default();
        let new_config = sample_config();
        let result = update_therapy_config(&mut overlay, &storage, &new_config, 1920, 1080).await;
        assert!(result.is_ok());
        assert!(overlay.update_config_called);
    }
}
