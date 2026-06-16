//! # Módulo de Colores
//!
//! Define el tipo `Color` para representar colores en formato hexadecimal (RRGGBB)
//! y su validación.
//!
//! Este módulo proporciona una capa de seguridad para trabajar con colores,
//! garantizando que siempre tengan un formato válido antes de ser utilizados
//! en la terapia visual.
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_domain::domain::Color;
//!
//! // Creación exitosa de un color rojo
//! let red = Color::new("#FF0000").unwrap();
//! assert_eq!(red.as_str(), "#FF0000");
//!
//! // Los colores se normalizan a mayúsculas automáticamente
//! let green = Color::new("#00ff00").unwrap();
//! assert_eq!(green.as_str(), "#00FF00");
//!
//! // Intento de crear un color inválido
//! let invalid = Color::new("#GGGGGG");
//! assert!(invalid.is_err());
//! ```

use serde::{Deserialize, Serialize};
use std::fmt;

/// Errores que pueden ocurrir al validar un color hexadecimal.
#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ColorError {
    /// El color no comienza con el carácter `#`.
    #[error(
        "The color format is invalid. It must start with '#' followed by 6 hexadecimal digits."
    )]
    InvalidFormat,
    /// El string tiene menos o más de 6 dígitos hexadecimales después del `#`.
    #[error(
        "The color must contain exactly 6 hexadecimal digits after the '#'. (For example: '#RRGGBB')."
    )]
    InvalidLength,
    /// El string contiene caracteres no hexadecimales (0-9, A-F).
    #[error("The color contains invalid hexadecimal digits.")]
    InvalidHexDigit,
}

/// Representa un color en formato RGB hexadecimal (#RRGGBB).
///
/// Esta estructura garantiza que el color siempre tenga un formato válido.
/// Internamente almacena el color como un string en mayúsculas, normalizado
/// para facilitar comparaciones y serialización.
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::domain::Color;
///
/// // Creación de un color personalizado
/// let custom = Color::new("#3366CC").unwrap();
/// assert_eq!(custom.as_str(), "#3366CC");
/// ```
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color(String);

impl Color {
    /// Crea un nuevo color a partir de un string hexadecimal.
    ///
    /// # Argumentos
    ///
    /// * `hex` - Un string que debe comenzar con `#` y tener exactamente 6 caracteres
    ///          hexadecimales (0-9, A-F). El formato no es sensible a mayúsculas,
    ///          ya que el color se normaliza internamente.
    ///
    /// # Errores
    ///
    /// Esta función devuelve un error si el formato del color no es válido:
    /// - [`ColorError::InvalidFormat`] si no comienza con `#`
    /// - [`ColorError::InvalidLength`] si no tiene exactamente 6 dígitos después del `#`
    /// - [`ColorError::InvalidHexDigit`] si contiene caracteres no hexadecimales
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::Color;
    ///
    /// // Colores válidos
    /// let red = Color::new("#FF0000").unwrap();
    /// let blue = Color::new("#0000ff").unwrap(); // se normaliza a mayúsculas
    ///
    /// // Colores inválidos
    /// assert!(Color::new("FF0000").is_err());   // falta '#'
    /// assert!(Color::new("#FF00").is_err());    // longitud incorrecta
    /// assert!(Color::new("#GG0000").is_err());  // carácter no hexadecimal
    /// ```
    pub fn new(hex: &str) -> Result<Self, ColorError> {
        let hex_upper = hex.to_uppercase();

        // Validar el formato del color (debe ser un string de 7 caracteres que comience con '#').
        if !hex_upper.starts_with('#') {
            return Err(ColorError::InvalidFormat);
        }

        let hex_digits = &hex_upper[1..];
        if hex_digits.len() != 6 {
            return Err(ColorError::InvalidLength);
        }

        // Validar que los caracteres sean dígitos hexadecimales.
        if !hex_digits.chars().all(|c| c.is_ascii_hexdigit()) {
            return Err(ColorError::InvalidHexDigit);
        }

        Ok(Color(hex_upper))
    }

    /// Devuelve el color como un string en formato `#RRGGBB`.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::Color;
    ///
    /// let color = Color::new("#FF0000").unwrap();
    /// assert_eq!(color.as_str(), "#FF0000");
    /// ```
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// Proporciona un color predeterminado: negro (#000000).
impl Default for Color {
    fn default() -> Self {
        Color::new("#000000").unwrap()
    }
}

/// Permite formatear el color como un string
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::domain::Color;
///
/// let color = Color::new("#FF0000").unwrap();
/// assert_eq!(format!("{}", color), "#FF0000");
/// ```
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_color() {
        let color = Color::new("#FF0000").unwrap();
        assert_eq!(color.as_str(), "#FF0000");
    }

    #[test]
    fn test_valid_color_lowercase_converted_to_uppercase() {
        let color = Color::new("#ff0000").unwrap();
        assert_eq!(color.as_str(), "#FF0000");
    }

    #[test]
    fn test_invalid_color_no_hash() {
        let result = Color::new("FF0000");
        assert_eq!(result, Err(ColorError::InvalidFormat));
    }

    #[test]
    fn test_invalid_color_wrong_length() {
        let result = Color::new("#FF00");
        assert_eq!(result, Err(ColorError::InvalidLength));
    }

    #[test]
    fn test_invalid_color_non_hex() {
        let result = Color::new("#GG0000");
        assert_eq!(result, Err(ColorError::InvalidHexDigit));
    }

    #[test]
    fn test_default_color() {
        let color = Color::default();
        assert_eq!(color.as_str(), "#000000");
    }

    #[test]
    fn test_color_display() {
        let color = Color::new("#FF0000").unwrap();
        // Al usar format!("{}", color), fuerza a Rust a ejecutar la funcion fmt::Display
        assert_eq!(format!("{}", color), "#FF0000");
    }
}
