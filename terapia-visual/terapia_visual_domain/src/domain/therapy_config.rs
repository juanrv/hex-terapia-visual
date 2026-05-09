use serde::{Deserialize, Serialize};

use crate::domain::{Color, Layout, Opacity, Zone};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ConfigError {
    #[error("Zone count mismatch: expected {expected}, got {got}")]
    ZoneCountMismatch { expected: usize, got: usize },
}

/// Tipo de terapia disponible. Actualmente solo se implementa la división de color, pero se pueden agregar más tipos en el futuro.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TherapyType {
    ColorDivision,
}

/// Configuracion completa para una terapia visual
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct TherapyConfig {
    therapy_type: TherapyType,
    layout: Layout,
    zones_config: Vec<ZoneConfig>,
}

/// Configuracion de una zona dentro de la terapia, incluyendo su color y opacidad.
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZoneConfig {
    pub color: Color,
    pub opacity: Opacity,
}

impl TherapyConfig {
    /// Crea una nueva configuración de terapia, validando que la cantidad de zonas coincida con lo esperado por el layout.
    pub fn new(
        therapy_type: TherapyType,
        layout: Layout,
        zones_config: Vec<ZoneConfig>,
    ) -> Result<Self, ConfigError> {
        match therapy_type {
            TherapyType::ColorDivision => {
                let expected_zones = layout.zone_count();
                let got_zones = zones_config.len();
                if zones_config.len() != expected_zones {
                    return Err(ConfigError::ZoneCountMismatch {
                        expected: expected_zones,
                        got: got_zones,
                    });
                }
                Ok(Self {
                    therapy_type,
                    layout,
                    zones_config,
                })
            }
        }
    }

    /// Devuelve el tipo de terapia.
    pub fn therapy_type(&self) -> TherapyType {
        self.therapy_type
    }

    /// Devuelve el layout de la terapia.
    pub fn layout(&self) -> Layout {
        self.layout
    }

    /// Devuelve la configuración de las zonas.
    pub fn zones_config(&self) -> &[ZoneConfig] {
        &self.zones_config
    }

    /// Genera las zonas reales (con rectanculos calculados) para una resolución de pantalla dada.
    pub fn generate_zones(&self, screen_width: u32, screen_height: u32) -> Vec<Zone> {
        let zone_rects = self.layout.calculate_zones(screen_width, screen_height);
        zone_rects
            .into_iter()
            .zip(self.zones_config.iter())
            .map(|(rect, config)| Zone::new(rect, config.color.clone(), config.opacity.clone()))
            .collect()
    }
}

impl Default for TherapyConfig {
    fn default() -> Self {
        let default_zones = vec![
            ZoneConfig {
                color: Color::new("#FF0000").unwrap(),
                opacity: Opacity::default(),
            },
            ZoneConfig {
                color: Color::new("#00FF00").unwrap(),
                opacity: Opacity::default(),
            },
        ];
        Self::new(TherapyType::ColorDivision, Layout::Vertical, default_zones)
            .expect("Configuracion por defecto")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_zones_config() -> Vec<ZoneConfig> {
        vec![
            ZoneConfig {
                color: Color::new("#FF0000").unwrap(),
                opacity: Opacity::new(0.8).unwrap(),
            },
            ZoneConfig {
                color: Color::new("#00FF00").unwrap(),
                opacity: Opacity::new(0.5).unwrap(),
            },
        ]
    }

    #[test]
    fn test_new_config_valid() {
        let config = TherapyConfig::new(
            TherapyType::ColorDivision,
            Layout::Vertical,
            two_zones_config(),
        );
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.therapy_type(), TherapyType::ColorDivision);
        assert_eq!(config.layout(), Layout::Vertical);
        assert_eq!(config.zones_config().len(), 2);
    }

    #[test]
    fn test_new_config_invalid_zone_count() {
        let zones = vec![ZoneConfig {
            color: Color::default(),
            opacity: Opacity::default(),
        }];
        let result = TherapyConfig::new(TherapyType::ColorDivision, Layout::Vertical, zones);
        assert_eq!(
            result,
            Err(ConfigError::ZoneCountMismatch {
                expected: 2,
                got: 1
            })
        );
    }

    #[test]
    fn test_generate_zones() {
        let config = TherapyConfig::new(
            TherapyType::ColorDivision,
            Layout::Vertical,
            two_zones_config(),
        )
        .unwrap();
        let zones = config.generate_zones(1920, 1080);
        assert_eq!(zones.len(), 2);
        assert_eq!(zones[0].rect().x, 0);
        assert_eq!(zones[0].rect().width, 960);
        assert_eq!(zones[1].rect().x, 960);
        assert_eq!(zones[1].rect().width, 960);
        assert_eq!(zones[0].color().as_str(), "#FF0000");
        assert_eq!(zones[1].color().as_str(), "#00FF00");
    }
}
