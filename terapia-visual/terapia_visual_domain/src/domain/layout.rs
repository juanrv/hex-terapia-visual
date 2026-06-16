//! # Módulo de Layouts
//!
//! Define los patrones de división de la pantalla para la terapia visual.
//!
//! Cada variante de [`Layout`] determina cómo se distribuyen las zonas de color
//! sobre la pantalla del usuario. Actualmente se soportan:
//!
//! - **Vertical**: dos zonas (izquierda / derecha)
//! - **Horizontal**: dos zonas (arriba / abajo)
//! - **Checkerboard**: cuatro zonas en forma de tablero de ajedrez (2x2)
//! - **Vertical4Columns**: cuatro zonas en columnas verticales
//!
//! Cada layout implementa la lógica para calcular sus zonas en función de las dimensiones
//! de la pantalla, maneja correctamente las divisiones impares para evitar huecos,
//! y expone la cantidad de zonas que genera para validar la configuración de la terapia.

use crate::domain::ZoneRect;
use serde::{Deserialize, Serialize};

/// Patrones de distribución de zonas de color sobre la pantalla.
///
/// Determina cómo se organizan las áreas de color que el usuario ve durante la terapia.
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::domain::Layout;
///
/// // Crear un layout vertical
/// let layout = Layout::Vertical;
/// assert_eq!(layout.zone_count(), 2);
///
/// // Calcular las zonas para una pantalla de 1920x1080
/// let zones = layout.calculate_zones(1920, 1080);
/// assert_eq!(zones.len(), 2);
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Layout {
    /// División en dos mitades: izquierda y derecha.
    ///
    /// - Zona 0: mitad izquierda (x=0, ancho=screen_width/2)
    /// - Zona 1: mitad derecha (x=screen_width/2, ancho=screen_width - screen_width/2)
    Vertical,

    /// División en dos mitades: arriba y abajo.
    ///
    /// - Zona 0: mitad superior (y=0, alto=screen_height/2)
    /// - Zona 1: mitad inferior (y=screen_height/2, alto=screen_height - screen_height/2)
    Horizontal,

    /// Patrón de tablero de ajedrez (2x2).
    ///
    /// Las cuatro zonas se disponen en una cuadrícula de 2x2:
    /// - Zona 0: superior izquierda
    /// - Zona 1: superior derecha
    /// - Zona 2: inferior izquierda
    /// - Zona 3: inferior derecha
    ///
    /// **Nota**: Este layout impone una sincronización automática de colores y opacidades
    /// entre las zonas opuestas:
    /// - Zona 0 ↔ Zona 3 (diagonal principal)
    /// - Zona 1 ↔ Zona 2 (diagonal secundaria)
    Checkerboard,

    /// Cuatro columnas verticales de igual ancho.
    ///
    /// Las zonas se distribuyen de izquierda a derecha:
    /// - Zona 0: columna 1 (x=0, ancho=screen_width/4)
    /// - Zona 1: columna 2 (x=screen_width/4, ancho=screen_width/4)
    /// - Zona 2: columna 3 (x=2*screen_width/4, ancho=screen_width/4)
    /// - Zona 3: columna 4 (x=3*screen_width/4, ancho=screen_width - 3*screen_width/4)
    ///
    /// **Nota**: Este layout impone una sincronización automática de colores y opacidades
    /// entre las zonas reflejadas horizontalmente:
    /// - Zona 0 ↔ Zona 2
    /// - Zona 1 ↔ Zona 3
    Vertical4Columns,
}

impl Layout {
    /// Devuelve la cantidad de zonas que genera este layout.
    ///
    /// Este valor se usa para validar que la configuración de la terapia tenga el
    /// número correcto de zonas de color.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::Layout;
    ///
    /// assert_eq!(Layout::Vertical.zone_count(), 2);
    /// assert_eq!(Layout::Checkerboard.zone_count(), 4);
    /// ```
    pub fn zone_count(&self) -> usize {
        match self {
            Layout::Vertical => 2,
            Layout::Horizontal => 2,
            Layout::Checkerboard => 4,
            Layout::Vertical4Columns => 4,
        }
    }

    /// Calcula los rectángulos que ocupa cada zona para una resolución de pantalla dada.
    ///
    /// # Argumentos
    ///
    /// * `width` - Ancho de la pantalla (en píxeles)
    /// * `height` - Alto de la pantalla (en píxeles)
    ///
    /// # Garantías
    ///
    /// - El número de rectángulos devueltos coincide con [`Layout::zone_count`].
    /// - Las zonas cubren completamente el área de la pantalla, incluso cuando la división no es exacta.
    /// - En divisiones impares, la última zona absorbe los píxeles restantes para evitar huecos.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::{Layout, ZoneRect};
    ///
    /// let zones = Layout::Vertical.calculate_zones(1920, 1080);
    /// assert_eq!(zones[0], ZoneRect::new(0, 0, 960, 1080));
    /// assert_eq!(zones[1], ZoneRect::new(960, 0, 960, 1080));
    /// ```
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

            Layout::Vertical4Columns => {
                let column_width = width / 4;

                vec![
                    ZoneRect::new(0, 0, column_width, height), // columna 1
                    ZoneRect::new(column_width, 0, column_width, height), // Columna 2
                    ZoneRect::new(column_width * 2, 0, column_width, height), // Columna 3
                    ZoneRect::new(column_width * 3, 0, width - (column_width * 3), height), // columna 4, toma el ancho restante
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
        assert_eq!(Layout::Vertical4Columns.zone_count(), 4);
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

    #[test]
    fn test_vertical4columns_layout_calculation() {
        // Pantalla de 1920x1080 (1920 / 4 = 480 por columna)
        let zones = Layout::Vertical4Columns.calculate_zones(1920, 1080);
        assert_eq!(zones.len(), 4);

        // Columna 1
        assert_eq!(zones[0], ZoneRect::new(0, 0, 480, 1080));
        // Columna 2
        assert_eq!(zones[1], ZoneRect::new(480, 0, 480, 1080));
        // Columna 3
        assert_eq!(zones[2], ZoneRect::new(960, 0, 480, 1080));
        // Columna 4
        assert_eq!(zones[3], ZoneRect::new(1440, 0, 480, 1080));
    }

    #[test]
    fn test_vertical4columns_odd_width_handling() {
        // Pantalla de 1922 de ancho. 1922 / 4 = 480 (sobran 2 pixeles)
        let zones = Layout::Vertical4Columns.calculate_zones(1922, 1080);

        assert_eq!(zones[0].width, 480);
        assert_eq!(zones[1].width, 480);
        assert_eq!(zones[2].width, 480);
        // La última columna debe absorber los píxeles restantes para no dejar huecos
        assert_eq!(zones[3].width, 1922 - (480 * 3)); // 482
    }
}
