use core::fmt;

use serde::{Deserialize, Serialize};

/// Errores que pueden ocurrir al instanciar una opacidad.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum OpacityError {
    /// Ocurre cuando el valor proporcionado está fuera del rango seguro (0.0 - 0.8).
    #[error("Opacity must be between 0.0 and 0.8. Provided: {0}")]
    OutOfRange(f32),
}

/// Valor de opacidad seguro para la terapia visual.
///
/// Representa un número de punto flotante. Por diseño y seguridad del usuario,
/// el valor máximo está limitado a `0.8` (80% de opacidad) para evitar que
/// la capa de la terapia bloquee por completo la visión del sistema subyacente.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub struct Opacity(f32);

impl Opacity {
    /// Límite máximo de opacidad permitido (80%).
    pub const MAX_OPACITY: f32 = 0.8;

    /// Crea un nuevo valor de opacidad validando que esté en el rango permitido (0.0 a 0.8).
    ///
    /// # Argumentos
    ///
    /// * `value` - Un `f32` que representa el nivel de opacidad deseado.
    ///
    /// # Errores
    ///
    /// Devuelve [`OpacityError::OutOfRange`] si el valor es menor que `0.0` o mayor que [`Opacity::MAX_OPACITY`].
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::opacity::Opacity;
    ///
    /// // Creación exitosa
    /// let op = Opacity::new(0.5).unwrap();
    /// assert_eq!(op.value(), 0.5);
    ///
    /// // Falla por superar el límite de seguridad
    /// let error = Opacity::new(0.9);
    /// assert!(error.is_err());
    /// ```
    pub fn new(value: f32) -> Result<Self, OpacityError> {
        if (0.0..=Self::MAX_OPACITY).contains(&value) {
            Ok(Opacity(value))
        } else {
            Err(OpacityError::OutOfRange(value))
        }
    }

    /// Devuelve el valor `f32` interno de la opacidad.
    pub fn value(&self) -> f32 {
        self.0
    }
}

/// Implementación de Default para Opacity.
///
/// El valor predeterminado es `0.5` (50% de transparencia),
/// considerado un punto medio ideal para iniciar la terapia.
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
