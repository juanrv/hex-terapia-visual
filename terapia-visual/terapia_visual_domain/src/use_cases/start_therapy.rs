//! # Caso de Uso: Iniciar Terapia
//!
//! Este caso de uso se encarga de iniciar la terapia visual,
//! mostrando el overlay con la configuración actual.
//!
//! # Flujo
//!
//! 1. Verifica que el overlay no esté ya activo.
//! 2. Si está inactivo, llama al puerto `OverlayPort::show()`.
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_domain::use_cases::start_therapy;
//! use terapia_visual_domain::ports::OverlayPort;
//! use terapia_visual_domain::domain::TherapyConfig;
//!
//! # async fn example(overlay: &mut dyn OverlayPort) -> Result<(), Box<dyn std::error::Error>> {
//! let config = TherapyConfig::default();
//! start_therapy::start_therapy(overlay, &config, 1920, 1080).await?;
//! # Ok(())
//! # }
//! ```

use crate::{
    domain::TherapyConfig,
    ports::{OverlayError, OverlayPort, overlay},
};

/// Inicia la terapia visual.
///
/// Esta función recibe un adaptador de overlay y la configuración de la terapia,
/// y muestra la ventana de superposición si no está ya activa.
///
/// # Argumentos
///
/// * `overlay` - Adaptador que implementa `OverlayPort` (ej. `TauriOverlay`).
/// * `config` - Configuración de la terapia (colores, layout, opacidades).
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
/// ```
/// use terapia_visual_domain::use_cases::start_therapy;
/// use terapia_visual_domain::ports::{OverlayPort, OverlayError};
/// use terapia_visual_domain::domain::TherapyConfig;
///
/// # async fn example(overlay: &mut dyn OverlayPort) -> Result<(), OverlayError> {
/// let config = TherapyConfig::default();
/// start_therapy::start_therapy(overlay, &config, 1920, 1080).await?;
/// # Ok(())
/// # }
/// ```
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
