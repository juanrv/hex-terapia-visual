use crate::domain::ZoneRect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Layout {
    Vertical,
    Horizontal,
}

impl Layout {
    /// Devuelve la cantidad de zonas que genera el layout.
    pub fn zone_count(&self) -> usize {
        match self {
            Layout::Vertical => 2,
            Layout::Horizontal => 2,
        }
    }

    /// Calcula las zonas para una resolucion de pantalla dada.
    /// Las zonas se devuelven en orden: [izquierda/arriba, derecha/abajo].
    pub fn calculate_zones(&self, width: u32, height: u32) -> Vec<ZoneRect> {
        match self {
            Layout::Vertical => {
                let half_width = width / 2;
                vec![
                    ZoneRect::new(0, 0, half_width, height), // Zona izquierda
                    ZoneRect::new(half_width, 0, width - half_width, height), // Zona derecha
                ]
            }
            Layout::Horizontal => {
                let half_height = height / 2;
                vec![
                    ZoneRect::new(0, 0, width, half_height), // Zona superior
                    ZoneRect::new(0, half_height, width, height - half_height), // Zona inferior
                ]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vertical_layout_zone_count() {
        assert_eq!(Layout::Vertical.zone_count(), 2);
    }

    #[test]
    fn test_horizontal_layout_zone_count() {
        assert_eq!(Layout::Horizontal.zone_count(), 2);
    }

    #[test]
    fn test_vertical_layout_calculation() {
        let zones = Layout::Vertical.calculate_zones(1920, 1080);
        assert_eq!(zones.len(), 2);
        assert_eq!(zones[0], ZoneRect::new(0, 0, 960, 1080));
        assert_eq!(zones[1], ZoneRect::new(960, 0, 960, 1080));
    }

    #[test]
    fn test_horizontal_layout_calculation() {
        let zones = Layout::Horizontal.calculate_zones(1920, 1080);
        assert_eq!(zones.len(), 2);
        assert_eq!(zones[0], ZoneRect::new(0, 0, 1920, 540));
        assert_eq!(zones[1], ZoneRect::new(0, 540, 1920, 540));
    }

    #[test]
    fn test_odd_width_handling() {
        let zones = Layout::Vertical.calculate_zones(1921, 1080);
        // 1921 / 2 = 960 (división entera), la segunda zona toma el resto
        assert_eq!(zones[0].width, 960);
        assert_eq!(zones[1].width, 1921 - 960);
    }
}
