use std::sync::RwLock;

use terapia_visual_domain::domain::AppSettings;

static CURRENT_LANG: RwLock<Language> = RwLock::new(Language::Spanish);

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
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
    let mut guard = CURRENT_LANG.write().unwrap();
    *guard = lang;
    println!("[DEBUG] init_language set to: {:?}", lang);
}

fn lang() -> Language {
    *CURRENT_LANG.read().unwrap()
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

// TODO: Enviar este message a un archivo por fuera del adaptador
messages!(
    info_therapy_started: "Terapia Iniciada.","Therapy Started.";
    info_therapy_stopped: "Terapia Detenida.", "Therapy Stopped.";
    info_config_updated: "Configuración Actualizada", "Configuration Updated";

    window_title: "Terapia Visual", "Visual Therapy";
    tooltip_therapy_active: "Terapia Activa", "Therapy Active";
    tooltip_therapy_inactive: "Terapia Inactiva", "Therapy Inactive";
    tooltip_app_name: "Terapia Visual", "Visual Therapy";

    error_generic: "Ha ocurrido un error inesperado", "An unexpected error ocurrred";
);
