use crate::domain::{Color, Opacity, ZoneRect};

/// Representa una zona coloreada en la pantalla, con un color y una opacidad específicos.
#[derive(Debug, Clone, PartialEq)]
pub struct Zone {
    rect: ZoneRect,
    color: Color,
    opacity: Opacity,
}

impl Zone {
    /// Crea una zona nueva
    pub fn new(rect: ZoneRect, color: Color, opacity: Opacity) -> Self {
        Self {
            rect,
            color,
            opacity,
        }
    }

    /// Rectangulo de la zona
    pub fn rect(&self) -> &ZoneRect {
        &self.rect
    }

    /// Color de la zona
    pub fn color(&self) -> &Color {
        &self.color
    }

    /// Opacidad de la zona
    pub fn opacity(&self) -> &Opacity {
        &self.opacity
    }

    /// Verifica si un punto (x, y) está dentro de la zona.
    pub fn contains(&self, x: u32, y: u32) -> bool {
        x >= self.rect.x
            && x < self.rect.x + self.rect.width
            && y >= self.rect.y
            && y < self.rect.y + self.rect.height
    }
}

#[cfg(test)]
mod tests {

    use super::*;
    use crate::domain::{Color, Opacity, ZoneRect};

    #[test]
    fn test_zone_creation() {
        let rect = ZoneRect::new(0, 0, 100, 100);
        let color = Color::new("#FF0000").unwrap();
        let opacity = Opacity::new(0.8).unwrap();

        let zone = Zone::new(rect, color.clone(), opacity);
        assert_eq!(zone.rect(), &rect);
        assert_eq!(zone.color(), &color);
        assert_eq!(zone.opacity().value(), 0.8);
    }

    #[test]
    fn test_zone_contains_point() {
        let rect = ZoneRect::new(10, 20, 100, 80);
        let color = Color::default();
        let opacity = Opacity::default();
        let zone = Zone::new(rect, color, opacity);

        // Punto dentro
        assert!(zone.contains(50, 50));
        assert!(zone.contains(10, 20));
        assert!(zone.contains(109, 99)); // borde justo dentro (x < x+width)
        // Puntos fuera
        assert!(!zone.contains(5, 50)); // x muy izquierda
        assert!(!zone.contains(110, 50)); // x muy derecha
        assert!(!zone.contains(50, 15)); // y muy arriba
        assert!(!zone.contains(50, 100)); // y muy abajo (y == y+height -> fuera)
    }
}
