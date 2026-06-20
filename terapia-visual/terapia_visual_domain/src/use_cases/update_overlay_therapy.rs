//! # Caso de Uso: Actualizar Configuración de Terapia
//!
//! Este caso de uso actualiza la configuración de la terapia visual,
//! guardándola en el almacenamiento y aplicándola al overlay si está activo.
//!
//! # Flujo
//!
//! 1. Guarda la nueva configuración en el almacenamiento.
//! 2. Si el overlay está activo, actualiza el overlay con la nueva configuración.
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_domain::use_cases::update_therapy_config;
//! use terapia_visual_domain::ports::{ConfigStorage, OverlayPort};
//! use terapia_visual_domain::domain::TherapyConfig;
//!
//! # async fn example(
//! #     overlay: &mut dyn OverlayPort,
//! #     storage: &dyn ConfigStorage<TherapyConfig>,
//! # ) -> Result<(), Box<dyn std::error::Error>> {
//! let config = TherapyConfig::default();
//! update_therapy_config::update_therapy_config(overlay, storage, &config, 1920, 1080).await?;
//! # Ok(())
//! # }
//! ```

use crate::{
    domain::OverlayTherapyConfig,
    ports::{ConfigStorage, OverlayError, OverlayPort, StorageError},
};

/// Errores que pueden ocurrir al actualizar la configuración de la terapia.
#[derive(Debug, thiserror::Error)]
pub enum UpdateConfigError {
    /// Error al guardar en el almacenamiento.
    #[error(transparent)]
    Storage(#[from] StorageError),

    /// Error al actualizar el overlay.
    #[error(transparent)]
    Overlay(#[from] OverlayError),
}

/// Actualiza la configuración de la terapia visual.
///
/// Esta función guarda la nueva configuración en el almacenamiento y,
/// si la terapia está activa, actualiza el overlay en tiempo real.
///
/// # Argumentos
///
/// * `overlay` - Adaptador que implementa `OverlayPort`.
/// * `storage` - Adaptador que implementa `ConfigStorage<TherapyConfig>`.
/// * `new_config` - La nueva configuración de la terapia.
/// * `screen_width` - Ancho de la pantalla en píxeles.
/// * `screen_height` - Alto de la pantalla en píxeles.
///
/// # Errores
///
/// - [`UpdateConfigError::Storage`] si falla al guardar en el almacenamiento.
/// - [`UpdateConfigError::Overlay`] si falla al actualizar el overlay.
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::use_cases::update_therapy_config;
/// use terapia_visual_domain::ports::{ConfigStorage, OverlayPort};
/// use terapia_visual_domain::domain::TherapyConfig;
///
/// # async fn example(
/// #     overlay: &mut dyn OverlayPort,
/// #     storage: &dyn ConfigStorage<TherapyConfig>,
/// # ) -> Result<(), Box<dyn std::error::Error>> {
/// let config = TherapyConfig::default();
/// update_therapy_config::update_therapy_config(overlay, storage, &config, 1920, 1080).await?;
/// # Ok(())
/// # }
/// ```
pub async fn update_overlay_therapy(
    overlay: &mut dyn OverlayPort,
    storage: &dyn ConfigStorage<OverlayTherapyConfig>,
    new_config: &OverlayTherapyConfig,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), UpdateConfigError> {
    // Guardar en almacenamiento
    storage.save(new_config).await?;

    // Si el overlay está activo, actualizarlo en tiempo real
    if overlay.is_active() {
        overlay
            .update_config(new_config, screen_width, screen_height)
            .await?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::*;
    use crate::use_cases::mocks::{MockOverlay, MockTherapyConfigStorage};

    fn sample_config() -> OverlayTherapyConfig {
        OverlayTherapyConfig::new(
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
        let result = update_overlay_therapy(&mut overlay, &storage, &new_config, 1920, 1080).await;
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
        let result = update_overlay_therapy(&mut overlay, &storage, &new_config, 1920, 1080).await;
        assert!(result.is_ok());
        assert!(overlay.update_config_called);
    }
}
