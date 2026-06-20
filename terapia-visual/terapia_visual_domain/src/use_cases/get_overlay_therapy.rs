//! # Caso de Uso: Obtener Configuración de Terapia
//!
//! Este caso de uso obtiene la configuración actual de la terapia
//! desde el almacenamiento.
//!
//! # Comportamiento
//!
//! - Si existe una configuración guardada, la devuelve.
//! - Si no existe o hay un error, devuelve la configuración por defecto.
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_domain::use_cases::get_therapy_config;
//! use terapia_visual_domain::ports::ConfigStorage;
//! use terapia_visual_domain::domain::TherapyConfig;
//!
//! # async fn example(storage: &dyn ConfigStorage<TherapyConfig>) {
//! let config = get_therapy_config::get_therapy_config(storage).await;
//! # }
//! ```

use crate::{domain::OverlayTherapyConfig, ports::ConfigStorage};

/// Obtiene la configuración de terapia actual desde el almacenamiento.
///
/// Si la carga falla (archivo no encontrado, error de parseo, etc.),
/// devuelve la configuración por defecto.
///
/// # Argumentos
///
/// * `storage` - Adaptador que implementa `ConfigStorage<TherapyConfig>`.
///
/// # Retorno
///
/// La configuración actual o la configuración por defecto en caso de error.
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::use_cases::get_therapy_config;
/// use terapia_visual_domain::ports::ConfigStorage;
/// use terapia_visual_domain::domain::TherapyConfig;
///
/// # async fn example(storage: &dyn ConfigStorage<TherapyConfig>) {
/// let config = get_therapy_config::get_therapy_config(storage).await;
/// assert_eq!(config, TherapyConfig::default());
/// # }
/// ```
pub async fn get_overlay_therapy(
    storage: &dyn ConfigStorage<OverlayTherapyConfig>,
) -> OverlayTherapyConfig {
    storage.load().await.unwrap_or_default()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::domain::{Layout, OverlayTherapyConfig};
    use crate::use_cases::mocks::MockTherapyConfigStorage;

    #[tokio::test]
    async fn test_get_therapy_config_returns_default_on_error() {
        let storage = MockTherapyConfigStorage {
            config: None,
            should_fail_load: true,
            ..Default::default()
        };

        let config = get_overlay_therapy(&storage).await;
        // Debe dar configuración predeterminada
        assert_eq!(config, OverlayTherapyConfig::default());
    }

    #[tokio::test]
    async fn test_get_therapy_config_returns_stored() {
        // Crear configuracion distinta a la default
        let mut expected = OverlayTherapyConfig::default();
        expected.change_layout(Layout::Horizontal);

        // Simular comportamiento en disco
        let storage = MockTherapyConfigStorage {
            config: Some(expected.clone()),
            should_fail_load: false,
            ..Default::default()
        };

        let config = get_overlay_therapy(&storage).await;

        // Debe devolver exactamente lo que estaba guardado
        assert_eq!(config, expected);
    }
}
