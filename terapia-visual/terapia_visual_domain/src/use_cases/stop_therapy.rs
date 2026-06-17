//! # Caso de Uso: Detener Terapia
//!
//! Este caso de uso se encarga de detener la terapia visual,
//! ocultando el overlay y liberando sus recursos.
//!
//! # Flujo
//!
//! 1. Verifica que el overlay esté activo.
//! 2. Si está activo, llama al puerto `OverlayPort::hide()`.
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_domain::use_cases::stop_therapy;
//! use terapia_visual_domain::ports::OverlayPort;
//!
//! # async fn example(overlay: &mut dyn OverlayPort) -> Result<(), Box<dyn std::error::Error>> {
//! stop_therapy::stop_therapy(overlay).await?;
//! # Ok(())
//! # }
//! ```

use crate::ports::{OverlayError, OverlayPort};

/// Detiene la terapia visual.
///
/// Esta función oculta el overlay si está activo, liberando sus recursos.
/// Si el overlay ya está inactivo, devuelve un error.
///
/// # Argumentos
///
/// * `overlay` - Adaptador que implementa `OverlayPort` (ej. `TauriOverlay`).
///
/// # Errores
///
/// - [`OverlayError::NotActive`] si el overlay ya está inactivo.
/// - [`OverlayError::CloseError`] si falla el cierre de la ventana.
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::use_cases::stop_therapy;
/// use terapia_visual_domain::ports::{OverlayPort, OverlayError};
///
/// # async fn example(overlay: &mut dyn OverlayPort) -> Result<(), OverlayError> {
/// stop_therapy::stop_therapy(overlay).await?;
/// # Ok(())
/// # }
/// ```
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
