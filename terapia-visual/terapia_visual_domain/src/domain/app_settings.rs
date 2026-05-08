use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct AppSettings {
    pub language: String,
    // Pueden ir mas preferencias de la app en el futuro
}

impl AppSettings {
    pub fn language(&self) -> &str {
        &self.language
    }
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            language: "es".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_language_is_spanish() {
        let settings = AppSettings::default();
        assert_eq!(settings.language, "es");
    }

    #[test]
    fn test_serialization_roundtrip() {
        let original = AppSettings {
            language: "en".to_string(),
        };
        let json = serde_json::to_string(&original).unwrap();
        let deserialized: AppSettings = serde_json::from_str(&json).unwrap();
        assert_eq!(original, deserialized);
    }
}
