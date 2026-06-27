//! # Módulo de Geometría
//!
//! Define las estructuras geométricas básicas utilizadas para posicionar
//! las zonas de color en la pantalla.
//!
//! Este módulo proporciona [`ZoneRect`], una representación de rectángulo
//! que almacena las coordenadas y dimensiones de una zona de terapia
//! dentro de la pantalla del usuario.
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_domain::domain::ZoneRect;
//!
//! // Crear un rectángulo de 100x100 en la esquina superior izquierda
//! let rect = ZoneRect::new(0, 0, 100, 100);
//! assert_eq!(rect.x, 0);
//! assert_eq!(rect.y, 0);
//! assert_eq!(rect.width, 100);
//! assert_eq!(rect.height, 100);
//! ```

use serde::{Deserialize, Serialize};

/// Representa un rectángulo que define el área de una zona en la pantalla.
///
/// Almacena las coordenadas (`x`, `y`) y las dimensiones (`width`, `height`)
/// de una zona de color dentro de la pantalla del usuario.
///
/// Esta estructura es utilizada por los diferentes layouts para calcular
/// las posiciones de las zonas de color durante la terapia visual.
///
/// # Campos
///
/// * `x` - Coordenada horizontal (en píxeles) desde el borde izquierdo de la pantalla.
/// * `y` - Coordenada vertical (en píxeles) desde el borde superior de la pantalla.
/// * `width` - Ancho de la zona en píxeles.
/// * `height` - Alto de la zona en píxeles.
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::domain::ZoneRect;
///
/// // Crear un rectángulo que representa la mitad izquierda de una pantalla de 1920x1080
/// let half_screen = ZoneRect::new(0, 0, 960, 1080);
/// assert_eq!(half_screen.x, 0);
/// assert_eq!(half_screen.width, 960);
///
/// // Crear un rectángulo para una zona pequeña en el centro de la pantalla
/// let center_zone = ZoneRect::new(500, 300, 920, 480);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct ZoneRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl ZoneRect {
    /// Crea un nuevo rectángulo con las coordenadas y dimensiones especificadas.
    ///
    /// # Argumentos
    ///
    /// * `x` - Coordenada horizontal (en píxeles) desde el borde izquierdo.
    /// * `y` - Coordenada vertical (en píxeles) desde el borde superior.
    /// * `width` - Ancho de la zona en píxeles.
    /// * `height` - Alto de la zona en píxeles.
    ///
    /// # Garantías
    ///
    /// - Todos los valores son `u32`, lo que asegura que siempre sean positivos.
    /// - La suma `x + width` y `y + height` nunca excede los límites de `u32`,
    ///   gracias al sistema de tipos de Rust.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::ZoneRect;
    ///
    /// // Rectángulo que cubre toda la pantalla (asumiendo 1920x1080)
    /// let fullscreen = ZoneRect::new(0, 0, 1920, 1080);
    ///
    /// // Rectángulo para una zona de 400x300 en la esquina inferior derecha
    /// let bottom_right = ZoneRect::new(1520, 780, 400, 300);
    /// ```
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}

// Nota: No se incluyen tests en este módulo porque la estructura es simple
// y está probada indirectamente a través de los tests de `layout.rs`.
// La función `ZoneRect::new` es trivial y no requiere validación adicional.
