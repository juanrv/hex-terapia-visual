use std::sync::OnceLock;

use terapia_visual_domain::domain::AppSettings;

static CURRENT_LANG: OnceLock<Language> = OnceLock::new();

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Language {
    Spanish,
    English,
}

impl Language {
    pub fn from_str(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "en" => Language::English,
            _ => Language::Spanish,
        }
    }
}

/// Inicializa el idioma desde la configuracion de la app
/// Se debe llamar una vez al inicio
pub fn init_language(settings: &AppSettings) {
    let lang = Language::from_str(&settings.language);
    let _ = CURRENT_LANG.set(lang);
}

fn lang() -> Language {
    *CURRENT_LANG.get().unwrap_or(&Language::Spanish)
}

macro_rules! messages {
    ($($key:ident: $sp:literal, $en:literal;)*) => {
        $(
            pub fn $key() -> &'static str {
                match lang() {
                    Language::Spanish => $sp,
                    Language::English => $en,
                }
            }
        )*
    };
}

messages!(
    info_therapy_started: "Terapia Iniciada.","Therapy Started.";
    info_therapy_stopped: "Terapia Detenida.", "Therapy Stopped.";
    info_config_updated: "Configuración Actualizada", "Configuration Updated";

    tooltip_therapy_active: "Terapia Activa", "Therapy Active";
    tooltip_therapy_inactive: "Terapia Inactiva", "Therapy Inactive";
    tooltip_app_name: "Terapia Visual", "Visual Therapy";

    error_generic: "Ha ocurrido un error inesperado", "An unexpected error ocurrred";
);
