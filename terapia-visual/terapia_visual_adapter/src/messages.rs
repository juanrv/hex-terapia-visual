//! # Sistema de Mensajes Internacionalizados
//!
//! Este módulo proporciona un sistema de mensajes traducibles para la aplicación,
//! soportando actualmente español e inglés.
//!
//! # Características
//!
//! - **Idioma global**: El idioma actual se almacena en un `RwLock` para permitir
//!   cambios en tiempo de ejecución.
//! - **Macros para mensajes**: Define mensajes con traducciones usando una macro.
//! - **Cambio en caliente**: `init_language()` permite cambiar el idioma en cualquier momento.
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_adapter::messages::{self, init_language};
//! use terapia_visual_domain::domain::{AppSettings, Language};
//!
//! let settings = AppSettings {
//!     language: Language::English,
//! };
//!
//! // Inicializar el idioma
//! init_language(&settings);
//!
//! // Obtener un mensaje traducido
//! let tooltip = messages::tooltip_app_name();
//! assert_eq!(tooltip, "Visual Therapy");
//! ```

use std::sync::RwLock;

use terapia_visual_domain::domain::AppSettings;
use terapia_visual_domain::domain::app_settings::Language;

/// Estado global del idioma actual.
///
/// Se usa un `RwLock` para permitir lecturas concurrentes y escrituras
/// ocasionales cuando se cambia el idioma.
static CURRENT_LANG: RwLock<Language> = RwLock::new(Language::Spanish);

/// Inicializa o cambia el idioma actual de la aplicación.
///
/// Esta función debe llamarse al inicio de la aplicación con el idioma
/// guardado en la configuración, y cada vez que el usuario cambie el idioma.
///
/// # Argumentos
///
/// * `settings` - La configuración de la aplicación que contiene el idioma.
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_adapter::messages::init_language;
/// use terapia_visual_domain::domain::{AppSettings, Language};
///
/// let settings = AppSettings {
///     language: Language::English,
/// };
/// init_language(&settings);
/// ```
pub fn init_language(settings: &AppSettings) {
    let lang = settings.language();
    let mut guard = CURRENT_LANG.write().unwrap();
    *guard = lang;
    println!("[DEBUG] init_language set to: {:?}", lang);
}

/// Devuelve el idioma actual.
///
/// # Retorno
///
/// El idioma actualmente configurado en la aplicación.
fn lang() -> Language {
    *CURRENT_LANG.read().unwrap()
}

/// Macro para definir mensajes traducidos.
///
/// Cada mensaje tiene dos versiones: español e inglés.
/// La macro genera una función para cada mensaje que devuelve el texto
/// correspondiente al idioma actual.
///
/// # Sintaxis
///
/// ```ignore
/// messages!(
///     message_key: "Texto en español", "Text in English";
///     another_key: "Otro texto", "Another text";
/// );
/// ```
///
/// # Ejemplo
///
/// ```
/// use terapia_visual_adapter::messages;
///
/// // El mensaje se define en la macro
/// // (ya está definido como tooltip_app_name)
/// let tooltip = messages::tooltip_app_name();
/// ```
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
