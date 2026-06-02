use serde::{Deserialize, Serialize};

/// Idiomas soportados por la aplicacion
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    #[serde(rename = "es")]
    Spanish,
    #[serde(rename = "en")]
    English,
}

impl Default for Language {
    fn default() -> Self {
        Language::Spanish
    }
}

impl Language {
    pub fn as_str(&self) -> &'static str {
        match self {
            Language::Spanish => "es",
            Language::English => "en",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
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

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: Language::default(),
        }
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
}
