use core::fmt;

use serde::{Deserialize, Serialize};

#[derive(Debug, thiserror::Error, PartialEq)]
pub enum OpacityError {
    #[error("Opacity value must be between 0.0 and 0.8")]
    OutOfRange(f32),
}

/// Valor de opacidad, representado como un número flotante entre 0.0 (completamente transparente) y 1.0 (completamente opaco).
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Opacity(f32);

impl Opacity {
    pub const MAX_OPACITY: f32 = 0.8;

    /// Crea un nuevo valor de opacidad validando que esté en el rango permitido (0.0 a 1.0).
    pub fn new(value: f32) -> Result<Self, OpacityError> {
        if (0.0..=Self::MAX_OPACITY).contains(&value) {
            Ok(Opacity(value))
        } else {
            Err(OpacityError::OutOfRange(value))
        }
    }

    /// Devuelve el valor f32 de laopacidad.
    pub fn value(&self) -> f32 {
        self.0
    }
}

/// Implementación de Default para Opacity, estableciendo el valor predeterminado en 0.5.
impl Default for Opacity {
    fn default() -> Self {
        Opacity(0.5)
    }
}

impl fmt::Display for Opacity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:.2}", self.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_opacity() {
        let opacity = Opacity::new(0.5).unwrap();
        assert_eq!(opacity.value(), 0.5);
    }

    #[test]
    fn test_opacity_boundaries() {
        assert!(Opacity::new(0.0).is_ok());
        assert!(Opacity::new(0.8).is_ok());
    }

    #[test]
    fn test_invalid_opacity_too_low() {
        let result = Opacity::new(-0.1);
        assert_eq!(result, Err(OpacityError::OutOfRange(-0.1)));
    }

    #[test]
    fn test_invalid_opacity_too_high() {
        let result = Opacity::new(0.81);
        assert_eq!(result, Err(OpacityError::OutOfRange(0.81)));

        let result_max = Opacity::new(1.0);
        assert_eq!(result_max, Err(OpacityError::OutOfRange(1.0)));
    }

    #[test]
    fn test_default_opacity() {
        let opacity = Opacity::default();
        assert_eq!(opacity.value(), 0.5);
    }

    #[test]
    fn test_display() {
        let opacity = Opacity::new(0.75).unwrap();
        assert_eq!(format!("{}", opacity), "0.75");
    }
}
