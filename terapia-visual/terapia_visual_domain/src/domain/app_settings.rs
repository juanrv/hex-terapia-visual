use serde::{Deserialize, Serialize};

/// Idiomas soportados por la aplicacion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Language {
    #[serde(rename = "es")]
    #[default]
    Spanish,
    #[serde(rename = "en")]
    English,
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Spanish => "es",
            Language::English => "en",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Default)]
pub struct AppSettings {
    pub language: Language,
    // Pueden ir mas preferencias de la app en el futuro (theme, startup, etc)
}

impl AppSettings {
    pub fn language(&self) -> Language {
        self.language
    }

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
