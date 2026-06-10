use crate::domain::ZoneRect;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Layout {
    Vertical,
    Horizontal,
    Checkerboard,
}

impl Layout {
    /// Devuelve la cantidad de zonas que genera el layout.
    pub fn zone_count(&self) -> usize {
        match self {
            Layout::Vertical => 2,
            Layout::Horizontal => 2,
            Layout::Checkerboard => 4,
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

            Layout::Checkerboard => {
                let half_w = width / 2;
                let half_h = height / 2;
                vec![
                    ZoneRect::new(0, 0, half_w, half_h), // superior izquierda
                    ZoneRect::new(half_w, 0, width - half_w, half_h), // superior derecha
                    ZoneRect::new(0, half_h, half_w, height - half_h), // inferior izquierda
                    ZoneRect::new(half_w, half_h, width - half_w, height - half_h), // inferior derecha
                ]
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_layout_zone_counts() {
        assert_eq!(Layout::Vertical.zone_count(), 2);
        assert_eq!(Layout::Horizontal.zone_count(), 2);
        assert_eq!(Layout::Checkerboard.zone_count(), 4);
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
    fn test_checkerboard_layout_calculation() {
        let zones = Layout::Checkerboard.calculate_zones(1920, 1080);
        assert_eq!(zones.len(), 4);
        // Arriba - Izquierda (0)
        assert_eq!(zones[0], ZoneRect::new(0, 0, 960, 540));
        // Arriba - Derecha (1)
        assert_eq!(zones[1], ZoneRect::new(960, 0, 960, 540));
        // Abajo - Izquierda (2)
        assert_eq!(zones[2], ZoneRect::new(0, 540, 960, 540));
        // Abajo - Derecha (3)
        assert_eq!(zones[3], ZoneRect::new(960, 540, 960, 540));
    }

    #[test]
    fn test_odd_width_handling() {
        let zones = Layout::Vertical.calculate_zones(1921, 1080);
        // 1921 / 2 = 960 (división entera), la segunda zona toma el resto
        assert_eq!(zones[0].width, 960);
        assert_eq!(zones[1].width, 1921 - 960);
    }
}
