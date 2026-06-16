//! # Módulo de Configuración de la Aplicación
//!
//! Define la configuración global de la aplicación, independiente de la terapia.
//!
//! Este módulo contiene:
//! - [`Language`]: los idiomas soportados por la aplicación.
//! - [`AppSettings`]: la estructura que agrupa todas las preferencias globales.
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_domain::domain::{AppSettings, Language};
//!
//! // Crear una configuración por defecto (español)
//! let settings = AppSettings::default();
//! assert_eq!(settings.language(), Language::Spanish);
//!
//! // Cambiar a inglés
//! let mut settings = AppSettings::default();
//! settings.set_language(Language::English);
//! assert_eq!(settings.language(), Language::English);
//! ```

use serde::{Deserialize, Serialize};

/// Idiomas soportados por la aplicación.
///
/// Actualmente se soportan dos idiomas:
/// - Español (por defecto)
/// - Inglés
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::domain::Language;
///
/// let spanish = Language::Spanish;
/// assert_eq!(spanish.as_str(), "es");
///
/// let english = Language::English;
/// assert_eq!(english.as_str(), "en");
/// ```
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Language {
    /// Español (código ISO: "es")
    #[serde(rename = "es")]
    #[default]
    Spanish,

    /// Inglés (código ISO: "en")
    #[serde(rename = "en")]
    English,
}

impl Language {
    /// Devuelve el código ISO del idioma como un string.
    ///
    /// # Retorno
    ///
    /// * `"es"` para español.
    /// * `"en"` para inglés.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::Language;
    ///
    /// assert_eq!(Language::Spanish.as_str(), "es");
    /// assert_eq!(Language::English.as_str(), "en");
    /// ```
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Spanish => "es",
            Language::English => "en",
        }
    }
}

/// Configuración global de la aplicación.
///
/// Agrupa todas las preferencias que no están relacionadas directamente
/// con la terapia visual, como el idioma de la interfaz.
///
/// # Campos
///
/// * `language` - El idioma seleccionado por el usuario.
///
/// # Extensibilidad
///
/// Esta estructura está diseñada para crecer en el futuro, permitiendo añadir
/// nuevas preferencias (como el tema de la UI, atajos de teclado, etc.)
/// sin romper la compatibilidad con versiones anteriores.
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::domain::{AppSettings, Language};
///
/// // Crear una configuración personalizada en inglés
/// let settings = AppSettings {
///     language: Language::English,
/// };
/// assert_eq!(settings.language(), Language::English);
/// ```
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AppSettings {
    /// Idioma seleccionado por el usuario.
    pub language: Language,
    // Pueden ir mas preferencias de la app en el futuro (theme, startup, etc)
}

impl AppSettings {
    /// Devuelve el idioma actualmente seleccionado.
    ///
    /// # Retorno
    ///
    /// El valor [`Language`] que está activo en la aplicación.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::{AppSettings, Language};
    ///
    /// let settings = AppSettings::default();
    /// assert_eq!(settings.language(), Language::Spanish);
    /// ```
    pub fn language(&self) -> Language {
        self.language
    }

    /// Cambia el idioma de la aplicación.
    ///
    /// # Argumentos
    ///
    /// * `new_language` - El nuevo idioma a establecer.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::{AppSettings, Language};
    ///
    /// let mut settings = AppSettings::default();
    /// settings.set_language(Language::English);
    /// assert_eq!(settings.language(), Language::English);
    /// ```
    pub fn set_language(&mut self, new_language: Language) {
        self.language = new_language;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_language_is_spanish() {
        let settings = AppSettings::default();
        assert_eq!(settings.language, Language::Spanish);
    }

    #[test]
    fn test_serialization_roundtrip() {
        let original = AppSettings {
            language: Language::English,
        };
        // Debería serializarse como {"language":"en"}
        let json = serde_json::to_string(&original).unwrap();
        assert!(json.contains(r#""language":"en""#));

        let deserialized: AppSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }

    #[test]
    fn test_language_getters_and_setters() {
        let mut settings = AppSettings::default();
        assert_eq!(settings.language(), Language::Spanish);

        settings.set_language(Language::English);
        assert_eq!(settings.language(), Language::English);
        assert_eq!(settings.language().as_str(), "en");

        settings.set_language(Language::Spanish);
        assert_eq!(settings.language(), Language::Spanish);
        assert_eq!(settings.language().as_str(), "es");
    }
}
