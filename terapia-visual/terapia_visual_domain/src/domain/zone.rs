//! # Módulo de Zona
//!
//! Define la entidad `Zone`, que representa una zona de color en la pantalla
//! durante la terapia visual.
//!
//! Una `Zone` combina tres componentes:
//! - Un rectángulo ([`ZoneRect`]) que define su posición y tamaño en la pantalla.
//! - Un color ([`Color`]) que se muestra en esa área.
//! - Una opacidad ([`Opacity`]) que controla su transparencia.
//!
//! Esta entidad es el resultado de aplicar una configuración de terapia
//! a una resolución de pantalla concreta.
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_domain::domain::{Zone, ZoneRect, Color, Opacity};
//!
//! // Crear una zona roja con 80% de opacidad en la mitad izquierda de la pantalla
//! let rect = ZoneRect::new(0, 0, 960, 1080);
//! let color = Color::new("#FF0000").unwrap();
//! let opacity = Opacity::new(0.8).unwrap();
//!
//! let zone = Zone::new(rect, color, opacity);
//!
//! // Verificar las propiedades de la zona
//! assert_eq!(zone.rect().x, 0);
//! assert_eq!(zone.color().as_str(), "#FF0000");
//! assert_eq!(zone.opacity().value(), 0.8);
//!
//! // Verificar si un punto está dentro de la zona
//! assert!(zone.contains(100, 100));
//! assert!(!zone.contains(1000, 500)); // fuera de la mitad izquierda
//! ```

use serde::{Deserialize, Serialize};

use crate::domain::{Color, Opacity, ZoneRect};

/// Representa una zona de color en la pantalla durante la terapia visual.
///
/// Cada zona tiene una posición (`rect`), un color y una opacidad.
/// Las zonas son generadas a partir de una configuración de terapia
/// (`TherapyConfig`) y una resolución de pantalla concreta.
///
/// # Campos
///
/// * `rect` - Rectángulo que define la posición y tamaño de la zona.
/// * `color` - Color que se muestra en la zona.
/// * `opacity` - Nivel de transparencia de la zona (0.0 = transparente, 1.0 = opaco).
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::domain::{Zone, ZoneRect, Color, Opacity};
///
/// // Crear una zona azul con 60% de opacidad en la mitad derecha
/// let rect = ZoneRect::new(960, 0, 960, 1080);
/// let color = Color::new("#0000FF").unwrap();
/// let opacity = Opacity::new(0.6).unwrap();
///
/// let zone = Zone::new(rect, color, opacity);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct Zone {
    rect: ZoneRect,
    color: Color,
    opacity: Opacity,
}

impl Zone {
    /// Crea una nueva zona con el rectángulo, color y opacidad especificados.
    ///
    /// # Argumentos
    ///
    /// * `rect` - El rectángulo que define la posición y tamaño de la zona.
    /// * `color` - El color que se mostrará en la zona.
    /// * `opacity` - La opacidad de la zona (0.0 a 1.0).
    ///
    /// # Garantías
    ///
    /// - Todos los componentes ya han sido validados en su creación:
    ///   - `Color` garantiza un formato válido.
    ///   - `Opacity` garantiza un valor dentro del rango permitido.
    /// - La zona siempre representa un área válida en la pantalla.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::{Zone, ZoneRect, Color, Opacity};
    ///
    /// // Crear una zona verde en la esquina superior izquierda
    /// let zone = Zone::new(
    ///     ZoneRect::new(0, 0, 100, 100),
    ///     Color::new("#00FF00").unwrap(),
    ///     Opacity::new(0.5).unwrap(),
    /// );
    /// ```
    pub fn new(rect: ZoneRect, color: Color, opacity: Opacity) -> Self {
        Self {
            rect,
            color,
            opacity,
        }
    }

    /// Devuelve una referencia al rectángulo de la zona.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::{Zone, ZoneRect, Color, Opacity};
    ///
    /// let zone = Zone::new(
    ///     ZoneRect::new(10, 20, 100, 80),
    ///     Color::default(),
    ///     Opacity::default(),
    /// );
    ///
    /// let rect = zone.rect();
    /// assert_eq!(rect.x, 10);
    /// assert_eq!(rect.y, 20);
    /// ```
    pub fn rect(&self) -> &ZoneRect {
        &self.rect
    }

    /// Devuelve una referencia al color de la zona.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::{Zone, ZoneRect, Color, Opacity};
    ///
    /// let color = Color::new("#FF0000").unwrap();
    /// let zone = Zone::new(
    ///     ZoneRect::new(0, 0, 100, 100),
    ///     color,
    ///     Opacity::default(),
    /// );
    ///
    /// assert_eq!(zone.color().as_str(), "#FF0000");
    /// ```
    pub fn color(&self) -> &Color {
        &self.color
    }

    /// Devuelve una referencia a la opacidad de la zona.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::{Zone, ZoneRect, Color, Opacity};
    ///
    /// let opacity = Opacity::new(0.8).unwrap();
    /// let zone = Zone::new(
    ///     ZoneRect::new(0, 0, 100, 100),
    ///     Color::default(),
    ///     opacity,
    /// );
    ///
    /// assert_eq!(zone.opacity().value(), 0.8);
    /// ```
    pub fn opacity(&self) -> &Opacity {
        &self.opacity
    }

    /// Verifica si un punto (x, y) está dentro del área de la zona.
    ///
    /// # Argumentos
    ///
    /// * `x` - Coordenada horizontal (en píxeles) desde el borde izquierdo de la pantalla.
    /// * `y` - Coordenada vertical (en píxeles) desde el borde superior de la pantalla.
    ///
    /// # Comportamiento
    ///
    /// - Retorna `true` si el punto está dentro del rectángulo de la zona.
    /// - Retorna `false` si el punto está fuera.
    /// - **Nota**: Los bordes derecho e inferior son exclusivos (x < x + width, y < y + height).
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::{Zone, ZoneRect, Color, Opacity};
    ///
    /// let zone = Zone::new(
    ///     ZoneRect::new(10, 20, 100, 80),
    ///     Color::default(),
    ///     Opacity::default(),
    /// );
    ///
    /// // Puntos dentro de la zona
    /// assert!(zone.contains(50, 50));
    /// assert!(zone.contains(10, 20));  // borde superior izquierdo (inclusivo)
    /// assert!(zone.contains(109, 99)); // borde inferior derecho (exclusivo para x+width, y+height)
    ///
    /// // Puntos fuera de la zona
    /// assert!(!zone.contains(5, 50));  // demasiado a la izquierda
    /// assert!(!zone.contains(110, 50)); // demasiado a la derecha
    /// assert!(!zone.contains(50, 15)); // demasiado arriba
    /// assert!(!zone.contains(50, 100)); // demasiado abajo
    /// ```
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
