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
pub fn init_language(seettings: &AppSettings) {
    let lang = Language::from_str(&seettings.language);
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
    // Errores de overlay
    error_overlay_creation: "Error creando la ventana de terapia. Detalle: ", "Error creating therapy window. Details: ";
    error_overlay_update: "Error actualizando la ventana de terapia. Detalle: ", "Error updating therapy window. Details: ";
    error_overlay_already_active: "La terapia ya está activa.", "Therapy already active.";
    error_overlay_not_active: "La terapia no está activa.", "Therapy not active.";

    // Errores de configuración
    error_config_load: "No se pudo cargar la configuración. Usando valores por defecto.", "Failed to load configuration. Using defaults.";
    error_config_save: "No se pudo guardar la configuración.", "Failed to save configuration.";

    // Información general
    info_therapy_started: "Terapia iniciada.", "Therapy started.";
    info_therapy_stopped: "Terapia detenida.", "Therapy stopped.";
    info_config_updated: "Configuración actualizada.", "Configuration updated.";

    // Tooltips de la bandeja
    tooltip_therapy_active: "Terapia activa", "Therapy active";
    tooltip_therapy_inactive: "Terapia inactiva", "Therapy inactive";
);
