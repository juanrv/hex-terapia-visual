use serde::{Deserialize, Serialize};

use crate::domain::{Color, Layout, Opacity, Zone};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ConfigError {
    #[error("Zone count mismatch: expected {expected}, got {got}")]
    ZoneCountMismatch { expected: usize, got: usize },
    #[error("Zone index {0} out of bounds")]
    InvalidZoneIndex(usize),
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

    /// Cambia el layout actual, adaptando la cantidad de zonas para cumplir con el nuevo layout.
    pub fn change_layout(&mut self, new_layout: Layout) {
        let expected_zones = new_layout.zone_count();
        let current_zones = self.zones_config.len();

        if expected_zones > current_zones {
            let mut new_zones = self.zones_config.clone();
            while new_zones.len() < expected_zones {
                // Toma el indice de la zona que toca clonar (ciclicamente)
                let source_index = new_zones.len() % current_zones;
                new_zones.push(new_zones[source_index].clone());
            }
            self.zones_config = new_zones;
        } else if expected_zones < current_zones {
            self.zones_config.truncate(expected_zones);
        }

        self.layout = new_layout;

        // Ajedrez
        if self.layout == Layout::Checkerboard && self.zones_config.len() == 4 {
            self.zones_config[3] = self.zones_config[0].clone(); // Zona 4 comparte con Zona 1
            self.zones_config[2] = self.zones_config[1].clone(); // Zona 3 comparte con Zona 2
        }

        // 4 columnas
        if self.layout == Layout::Vertical4Columns && self.zones_config.len() == 4 {
            self.zones_config[2] = self.zones_config[0].clone(); // Zona 3 comparte con Zona 1
            self.zones_config[3] = self.zones_config[1].clone(); // Zona 4 comparte con Zona 2
        }
    }

    /// Actualiza el color de una zona especifica de forma segura
    pub fn update_zone_color(
        &mut self,
        zone_index: usize,
        new_color: Color,
    ) -> Result<(), ConfigError> {
        if zone_index >= self.zones_config.len() {
            return Err(ConfigError::InvalidZoneIndex(zone_index));
        }

        self.zones_config[zone_index].color = new_color.clone();

        // Aplica sincronizacion en las layouts con 4 zonas
        let paired_index = match self.layout {
            Layout::Checkerboard => match zone_index {
                0 => Some(3),
                3 => Some(0),
                1 => Some(2),
                2 => Some(1),
                _ => None,
            },
            Layout::Vertical4Columns => match zone_index {
                0 => Some(2),
                2 => Some(0), // 1 y 3 se sincronizan
                1 => Some(3),
                3 => Some(1), // 2 y 4 se sincronizan
                _ => None,
            },
            _ => None, // Vertical y Horizontal no tienen pares automaticos
        };

        if let Some(pair) = paired_index {
            self.zones_config[pair].color = new_color;
        }

        Ok(())
    }

    /// Actualiza la opacidad de una zona especifica de forma segura
    pub fn update_zone_opacity(
        &mut self,
        zone_index: usize,
        new_opacity: Opacity,
    ) -> Result<(), ConfigError> {
        if zone_index >= self.zones_config.len() {
            return Err(ConfigError::InvalidZoneIndex(zone_index));
        }

        self.zones_config[zone_index].opacity = new_opacity;

        let paired_index = match self.layout {
            Layout::Checkerboard => match zone_index {
                0 => Some(3),
                3 => Some(0),
                1 => Some(2),
                2 => Some(1),
                _ => None,
            },
            Layout::Vertical4Columns => match zone_index {
                0 => Some(2),
                2 => Some(0),
                1 => Some(3),
                3 => Some(1),
                _ => None,
            },
            _ => None,
        };

        if let Some(pair) = paired_index {
            self.zones_config[pair].opacity = new_opacity;
        }

        Ok(())
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
            .map(|(rect, config)| Zone::new(rect, config.color.clone(), config.opacity))
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
            .expect("Default config")
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
    fn test_change_layout_same_zone_count() {
        let mut config = TherapyConfig::default();

        config.change_layout(Layout::Horizontal);

        assert_eq!(config.layout(), Layout::Horizontal);
        assert_eq!(config.zones_config().len(), 2);
    }

    #[test]
    fn test_update_zone_color_success() {
        let mut config = TherapyConfig::default();
        let new_color = Color::new("#0000FF").unwrap(); // Azul

        let result = config.update_zone_color(0, new_color.clone());
        assert!(result.is_ok());
        assert_eq!(config.zones_config()[0].color, new_color);
    }

    #[test]
    fn test_update_zone_color_out_of_bounds() {
        let mut config = TherapyConfig::default();
        let new_color = Color::new("#0000FF").unwrap();

        // Intentamos actualizar la zona 99
        let result = config.update_zone_color(99, new_color);
        assert_eq!(result, Err(ConfigError::InvalidZoneIndex(99)));
    }

    #[test]
    fn test_update_zone_opacity_success() {
        let mut config = TherapyConfig::default();
        let new_opacity = Opacity::new(0.2).unwrap(); // Nueva opacidad

        let result = config.update_zone_opacity(1, new_opacity);
        assert!(result.is_ok());
        assert_eq!(config.zones_config()[1].opacity, new_opacity);
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

    #[test]
    fn test_change_layout_to_checkboard_expands_and_syncs() {
        let mut config = TherapyConfig::default(); // Empieza con 2 zonas (Vertical)

        config.change_layout(Layout::Checkerboard);

        assert_eq!(config.layout(), Layout::Checkerboard);
        assert_eq!(config.zones_config().len(), 4); // Crece a 4

        // Validar reglas de pares
        assert_eq!(
            config.zones_config()[0].color,
            config.zones_config()[3].color
        );
        assert_eq!(
            config.zones_config()[1].color,
            config.zones_config()[2].color
        );
    }

    #[test]
    fn test_checkerboard_syncs_color_updates() {
        let mut config = TherapyConfig::default();
        config.change_layout(Layout::Checkerboard);

        let new_color = Color::new("#FFFFFF").unwrap();

        // Actualizar la zona 0 (Top-Left)
        let _ = config.update_zone_color(0, new_color.clone());

        // Verificar la zona 3
        assert_eq!(config.zones_config()[3].color, new_color);

        // Lo mismo con la zona 1 y 2
        let another_color = Color::new("#123456").unwrap();
        let _ = config.update_zone_color(1, another_color.clone());
        assert_eq!(config.zones_config()[2].color, another_color);
    }

    #[test]
    fn test_change_layout_to_vertical4_expands_and_syncs() {
        let mut config = TherapyConfig::default(); // Inicia con 2 zonas

        // Cambio al nuevo layout de 4 columnas
        config.change_layout(Layout::Vertical4Columns);

        assert_eq!(config.layout(), Layout::Vertical4Columns);
        assert_eq!(config.zones_config().len(), 4);

        // Validar la regla Col 1 (indice 0) comparte con Col 3 (indice 2)
        assert_eq!(
            config.zones_config()[0].color,
            config.zones_config()[2].color
        );
        // Validar la regla Col 2 (indice 1) comparte con Col 4 (indice 3)
        assert_eq!(
            config.zones_config()[1].color,
            config.zones_config()[3].color
        );
    }

    #[test]
    fn test_vertical4_syncs_color_updates() {
        let mut config = TherapyConfig::default();
        config.change_layout(Layout::Vertical4Columns);

        let color_a = Color::new("#111111").unwrap();
        let color_b = Color::new("#222222").unwrap();

        // Si se actualiza la columna 1, la columna 3 debe actualizarse sola
        let _ = config.update_zone_color(0, color_a.clone());
        assert_eq!(config.zones_config()[2].color, color_a);

        // Si se actualiza la columna 4, la columna 2 debe actualizarse sola
        let _ = config.update_zone_color(3, color_b.clone());
        assert_eq!(config.zones_config()[1].color, color_b);
    }

    #[test]
    fn test_vertical4_syncs_opacity_updates() {
        let mut config = TherapyConfig::default();
        config.change_layout(Layout::Vertical4Columns);

        let new_opacity = Opacity::new(0.4).unwrap();

        // Si se actualiza la opacidad de la columna 3, la columna 1 debe seguirla
        let _ = config.update_zone_opacity(2, new_opacity);

        assert_eq!(config.zones_config()[0].opacity, new_opacity);
    }
}
