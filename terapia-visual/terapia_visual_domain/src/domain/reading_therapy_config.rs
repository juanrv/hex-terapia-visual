//! # Módulo de Terapia de Lectura
//!
//! Define el agregado principal [`ReadingTherapyConfig`], que representa
//! la configuración exclusiva para la modalidad de Terapia de Lectura.

use serde::{Deserialize, Serialize};

use crate::domain::overlay_therapy_config::ConfigError;
use crate::domain::{Color, Layout, Opacity, ReadingSettings, Zone, ZoneConfig};

/// Configuración completa para una Terapia de Lectura.
///
/// A diferencia del Overlay global, esta terapia ocurre en una ventana contenida
/// y requiere tanto la configuración de las zonas de color como las preferencias
/// tipográficas (fuente, colores de texto, etc.).
#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct ReadingTherapyConfig {
    layout: Layout,
    zones_config: Vec<ZoneConfig>,
    reading_settings: ReadingSettings,
}

impl ReadingTherapyConfig {
    /// Instancia una nueva configuración validando que el número de zonas sea el correcto.
    pub fn new(
        layout: Layout,
        zones_config: Vec<ZoneConfig>,
        reading_settings: ReadingSettings,
    ) -> Result<Self, ConfigError> {
        let expected_zones = layout.zone_count();
        let got_zones = zones_config.len();
        if got_zones != expected_zones {
            return Err(ConfigError::ZoneCountMismatch {
                expected: expected_zones,
                got: got_zones,
            });
        }
        Ok(Self {
            layout,
            zones_config,
            reading_settings,
        })
    }

    /// Cambia el layout actual y adapta dinámicamente el arreglo de zonas.
    pub fn change_layout(&mut self, new_layout: Layout) {
        let expected_zones = new_layout.zone_count();
        let current_zones = self.zones_config.len();

        if expected_zones > current_zones {
            let mut new_zones = self.zones_config.clone();
            while new_zones.len() < expected_zones {
                let source_index = new_zones.len() % current_zones;
                new_zones.push(new_zones[source_index].clone());
            }
            self.zones_config = new_zones;
        } else if expected_zones < current_zones {
            self.zones_config.truncate(expected_zones);
        }

        self.layout = new_layout;
        self.apply_sync_rules();
    }

    /// Aplica las reglas de sincronización de colores y opacidades.
    fn apply_sync_rules(&mut self) {
        match (self.layout, self.zones_config.len()) {
            (Layout::Checkerboard, 4) => {
                self.zones_config[3] = self.zones_config[0].clone();
                self.zones_config[2] = self.zones_config[1].clone();
            }
            (Layout::Vertical4Columns, 4) => {
                self.zones_config[2] = self.zones_config[0].clone();
                self.zones_config[3] = self.zones_config[1].clone();
            }
            _ => {}
        }
    }

    /// Actualiza el color de una zona específica aplicando sincronización.
    pub fn update_zone_color(
        &mut self,
        zone_index: usize,
        new_color: Color,
    ) -> Result<(), ConfigError> {
        if zone_index >= self.zones_config.len() {
            return Err(ConfigError::InvalidZoneIndex(zone_index));
        }

        self.zones_config[zone_index].color = new_color.clone();

        if let Some(pair) = self.get_sync_pair(zone_index) {
            self.zones_config[pair].color = new_color;
        }

        Ok(())
    }

    /// Actualiza la opacidad de una zona específica aplicando sincronización.
    pub fn update_zone_opacity(
        &mut self,
        zone_index: usize,
        new_opacity: Opacity,
    ) -> Result<(), ConfigError> {
        if zone_index >= self.zones_config.len() {
            return Err(ConfigError::InvalidZoneIndex(zone_index));
        }

        self.zones_config[zone_index].opacity = new_opacity;

        if let Some(pair) = self.get_sync_pair(zone_index) {
            self.zones_config[pair].opacity = new_opacity;
        }

        Ok(())
    }

    /// Obtiene el índice de la zona emparejada para sincronización.
    fn get_sync_pair(&self, zone_index: usize) -> Option<usize> {
        match self.layout {
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
        }
    }

    /// Actualiza todas las configuraciones de lectura de una vez.
    pub fn update_reading_settings(&mut self, settings: ReadingSettings) {
        self.reading_settings = settings;
    }

    pub fn layout(&self) -> Layout {
        self.layout
    }

    pub fn zones_config(&self) -> &[ZoneConfig] {
        &self.zones_config
    }

    pub fn reading_settings(&self) -> &ReadingSettings {
        &self.reading_settings
    }

    pub fn generate_zones(&self, screen_width: u32, screen_height: u32) -> Vec<Zone> {
        let zone_rects = self.layout.calculate_zones(screen_width, screen_height);
        zone_rects
            .into_iter()
            .zip(self.zones_config.iter())
            .map(|(rect, config)| Zone::new(rect, config.color.clone(), config.opacity))
            .collect()
    }
}

impl Default for ReadingTherapyConfig {
    /// Genera la configuración estándar para iniciar la lectura.
    /// Utiliza un layout Horizontal por defecto (suele ser más cómodo para leer)
    /// y carga las opciones óptimas de anti-supresión.
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
        Self::new(
            Layout::Horizontal,
            default_zones,
            ReadingSettings::default(),
        )
        .expect("Default reading config should always be valid")
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

    // ==========================================
    //  LECTURA ESPECÍFICA
    // ==========================================

    #[test]
    fn test_reading_config_creation() {
        let config = ReadingTherapyConfig::default();
        assert_eq!(config.layout(), Layout::Horizontal);
        assert_eq!(config.zones_config().len(), 2);
        assert_eq!(config.reading_settings().font_size, 22);
    }

    #[test]
    fn test_reading_config_update_settings() {
        let mut config = ReadingTherapyConfig::default();
        let mut new_settings = ReadingSettings::default();
        new_settings.font_size = 30;

        config.update_reading_settings(new_settings);
        assert_eq!(config.reading_settings().font_size, 30);
    }

    // ==========================================
    //  CREACIÓN Y GENERACIÓN BÁSICA
    // ==========================================

    #[test]
    fn test_new_config_valid() {
        let config = ReadingTherapyConfig::new(
            Layout::Vertical,
            two_zones_config(),
            ReadingSettings::default(),
        );
        assert!(config.is_ok());
        assert_eq!(config.unwrap().layout(), Layout::Vertical);
    }

    #[test]
    fn test_new_config_invalid_zone_count() {
        let zones = vec![ZoneConfig {
            color: Color::default(),
            opacity: Opacity::default(),
        }];
        let result = ReadingTherapyConfig::new(Layout::Vertical, zones, ReadingSettings::default());
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
        let config = ReadingTherapyConfig::new(
            Layout::Vertical,
            two_zones_config(),
            ReadingSettings::default(),
        )
        .unwrap();
        let zones = config.generate_zones(1920, 1080);
        assert_eq!(zones.len(), 2);
        assert_eq!(zones[0].rect().width, 960);
        assert_eq!(zones[1].color().as_str(), "#00FF00");
    }

    // ==========================================
    //  CAMBIOS DE LAYOUT
    // ==========================================

    #[test]
    fn test_change_layout_truncates_when_reducing_zones() {
        let mut config = ReadingTherapyConfig::default();
        config.change_layout(Layout::Checkerboard); // Sube a 4
        config.change_layout(Layout::Horizontal); // Baja a 2
        assert_eq!(config.layout(), Layout::Horizontal);
        assert_eq!(config.zones_config().len(), 2);
    }

    #[test]
    fn test_change_layout_to_vertical4_expands_and_syncs() {
        let mut config = ReadingTherapyConfig::default();
        config.change_layout(Layout::Vertical4Columns);
        assert_eq!(config.layout(), Layout::Vertical4Columns);
        assert_eq!(config.zones_config().len(), 4);
        // Regla: 0 sincroniza con 2, 1 sincroniza con 3
        assert_eq!(
            config.zones_config()[0].color,
            config.zones_config()[2].color
        );
        assert_eq!(
            config.zones_config()[1].color,
            config.zones_config()[3].color
        );
    }

    // ==========================================
    //  ACTUALIZACIONES BÁSICAS Y ERRORES
    // ==========================================

    #[test]
    fn test_update_zone_color_success_and_out_of_bounds() {
        let mut config = ReadingTherapyConfig::default();
        let new_color = Color::new("#0000FF").unwrap();

        assert!(config.update_zone_color(0, new_color.clone()).is_ok());
        assert_eq!(config.zones_config()[0].color, new_color);

        assert_eq!(
            config.update_zone_color(99, new_color),
            Err(ConfigError::InvalidZoneIndex(99))
        );
    }

    #[test]
    fn test_update_zone_opacity_success_and_out_of_bounds() {
        let mut config = ReadingTherapyConfig::default();
        let new_opacity = Opacity::new(0.2).unwrap();

        assert!(config.update_zone_opacity(1, new_opacity).is_ok());
        assert_eq!(config.zones_config()[1].opacity, new_opacity);

        assert_eq!(
            config.update_zone_opacity(99, new_opacity),
            Err(ConfigError::InvalidZoneIndex(99))
        );
    }

    #[test]
    fn test_updates_no_sync_for_horizontal_layout() {
        let mut config = ReadingTherapyConfig::default(); // Inicia Horizontal
        let new_color = Color::new("#FFFFFF").unwrap();
        let new_opacity = Opacity::new(0.3).unwrap();

        let _ = config.update_zone_color(0, new_color.clone());
        let _ = config.update_zone_opacity(0, new_opacity);

        assert_eq!(config.zones_config()[0].color, new_color);
        assert_eq!(config.zones_config()[0].opacity, new_opacity);
        assert_ne!(config.zones_config()[1].color, new_color); // No debe sincronizar
        assert_ne!(config.zones_config()[1].opacity, new_opacity);
    }

    // ==========================================
    //  REGLAS DE SINCRONIZACIÓN (AJEDREZ)
    // ==========================================

    #[test]
    fn test_checkerboard_syncs_updates_and_reverse() {
        let mut config = ReadingTherapyConfig::default();
        config.change_layout(Layout::Checkerboard);

        let color_a = Color::new("#111111").unwrap();
        let color_b = Color::new("#222222").unwrap();
        let op_a = Opacity::new(0.1).unwrap();

        // 0 actualiza 3
        let _ = config.update_zone_color(0, color_a.clone());
        assert_eq!(config.zones_config()[3].color, color_a);

        // Camino inverso: 3 actualiza 0 (Opacidad)
        let _ = config.update_zone_opacity(3, op_a);
        assert_eq!(config.zones_config()[0].opacity, op_a);

        // Camino inverso: 2 actualiza 1
        let _ = config.update_zone_color(2, color_b.clone());
        assert_eq!(config.zones_config()[1].color, color_b);

        // 1 actualiza 2
        let _ = config.update_zone_opacity(1, op_a);
        assert_eq!(config.zones_config()[2].opacity, op_a);
    }

    // ==========================================
    //  REGLAS DE SINCRONIZACIÓN (4 COLUMNAS)
    // ==========================================

    #[test]
    fn test_vertical4_syncs_updates_and_reverse() {
        let mut config = ReadingTherapyConfig::default();
        config.change_layout(Layout::Vertical4Columns);

        let color = Color::new("#333333").unwrap();
        let op = Opacity::new(0.4).unwrap();

        // Cubrir 1 => 3
        let _ = config.update_zone_color(1, color.clone());
        assert_eq!(config.zones_config()[3].color, color);

        // Cubrir 2 => 0
        let _ = config.update_zone_opacity(2, op);
        assert_eq!(config.zones_config()[0].opacity, op);

        // Cubrir 0 => 2
        let _ = config.update_zone_color(0, color.clone());
        assert_eq!(config.zones_config()[2].color, color);

        // Cubrir 3 => 1
        let _ = config.update_zone_opacity(3, op);
        assert_eq!(config.zones_config()[1].opacity, op);
    }

    // ==========================================
    // REGLAS DE SINCRONIZACIÓN - CASOS NONE
    // ==========================================

    #[test]
    fn test_get_sync_pair_returns_none_for_vertical_layout() {
        let config = ReadingTherapyConfig::new(
            Layout::Vertical,
            two_zones_config(),
            ReadingSettings::default(),
        )
        .unwrap();
        for i in 0..config.zones_config().len() {
            assert_eq!(config.get_sync_pair(i), None);
        }
    }

    #[test]
    fn test_get_sync_pair_returns_none_for_horizontal_layout() {
        let config = ReadingTherapyConfig::default(); // Ya es Horizontal
        for i in 0..config.zones_config().len() {
            assert_eq!(config.get_sync_pair(i), None);
        }
    }

    #[test]
    fn test_get_sync_pair_returns_none_for_out_of_range_index() {
        let config = ReadingTherapyConfig::default();
        assert_eq!(config.get_sync_pair(99), None);
        assert_eq!(config.get_sync_pair(100), None);
    }

    #[test]
    fn test_get_sync_pair_returns_none_for_unpaired_index_in_checkerboard() {
        let mut config = ReadingTherapyConfig::default();
        config.change_layout(Layout::Checkerboard);
        assert_eq!(config.get_sync_pair(4), None);
        assert_eq!(config.get_sync_pair(5), None);
    }

    #[test]
    fn test_get_sync_pair_returns_none_for_unpaired_index_in_vertical4() {
        let mut config = ReadingTherapyConfig::default();
        config.change_layout(Layout::Vertical4Columns);
        assert_eq!(config.get_sync_pair(4), None);
        assert_eq!(config.get_sync_pair(5), None);
    }
}
