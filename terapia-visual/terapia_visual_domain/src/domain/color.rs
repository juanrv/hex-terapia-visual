use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, thiserror::Error, PartialEq, Eq)]
pub enum ColorError {
    #[error(
        "El formato del color es inválido. Debe comenzar con '#' seguido de 6 dígitos hexadecimales."
    )]
    InvalidFormat,
    #[error(
        "El color debe contener exactamente 6 dígitos hexadecimales después del '#'. (Ejemplo: '#RRGGBB')."
    )]
    InvalidLength,
    #[error("El color contiene dígitos hexadecimales inválidos.")]
    InvalidHexDigit,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Color(String);

impl Color {
    /// Crea un nuevo color validando que el formato sea correcto (ejemplo: "#RRGGBB").
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

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new("#000000").unwrap()
    }
}

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
}
