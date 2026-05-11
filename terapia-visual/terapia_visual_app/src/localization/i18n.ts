export type Language = "es" | "en";

export const translations = {
  es: {
    app_title: "Terapia Visual",
    btn_start: "Iniciar Terapia",
    btn_stop: "Detener Terapia",
    btn_update: "Actualizar Configuración",
    btn_get: "Mostrar Configuración en Consola",
    btn_spanish: "Español",
    btn_english: "English",
    status_started: "Terapia iniciada",
    status_stopped: "Terapia detenida",
    status_updated: "Configuración actualizada",
    error_generic: "Ha ocurrido un error inesperado",
  },
  en: {
    app_title: "Visual Therapy",
    btn_start: "Start Therapy",
    btn_stop: "Stop Therapy",
    btn_update: "Update Configuration",
    btn_get: "Show Configuration in Console",
    btn_spanish: "Spanish",
    btn_english: "English",
    status_started: "Therapy started",
    status_stopped: "Therapy stopped",
    status_updated: "Configuration updated",
    error_generic: "An unexpected error occurred",
  },
};

let currentLang: Language = "es";

export function setLanguage(lang: Language) {
  currentLang = lang;
  applyTranslations();
}

export function getCurrentLanguage(): Language {
  return currentLang;
}

export function translate(key: keyof typeof translations.es): string {
  return translations[currentLang][key];
}

function applyTranslations() {
  // Actualizar titulo de la pagina
  document.title = translate("app_title");
  // Actualizar textos de elementos con el atributo data-i18n
  document.querySelectorAll("[data-i18n]").forEach((element) => {
    const key = element.getAttribute("data-i18n");
    if (key && element instanceof HTMLElement) {
      element.innerText = translate(key as any);
    }
  });
}
