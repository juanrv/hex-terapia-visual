use serde::{Deserialize, Serialize};

use crate::domain::Color;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReadingSettings {
    /// Tamaño de la fuente en píxeles.
    pub font_size: u32,
    /// Color del texto principal.
    pub text_color: Color,
    /// Color de fondo general de la página.
    pub bg_color: Color,
    /// Altura de línea para facilitar la lectura.
    pub line_height: String,
}

impl Default for ReadingSettings {
    /// Valores terapéuticos por defecto.
    ///
    /// Estos valores están diseñados para la terapia de anti-supresión
    /// con gafas anaglifas (rojo-verde). El texto en gris oscuro (#242424) sobre fondo
    /// negro (#000000) bajo un filtro de color puro (rojo o verde) provoca una cancelación
    /// visual en el ojo con el lente opuesto, forzando al cerebro a usar el ojo deseado.
    fn default() -> Self {
        Self {
            font_size: 22,                              // Tamaño cómodo para la vista
            text_color: Color::new("#242424").unwrap(), // Gris muy oscuro (clave para la cancelación)
            bg_color: Color::new("#000000").unwrap(),   // Fondo negro puro
            line_height: "1.65".to_string(),            // Espaciado óptimo
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reading_settings_default() {
        let settings = ReadingSettings::default();
        assert_eq!(settings.font_size, 22);
        assert_eq!(settings.text_color.as_str(), "#242424");
        assert_eq!(settings.bg_color.as_str(), "#000000");
    }
}
