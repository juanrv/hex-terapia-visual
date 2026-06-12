export type Language = "es" | "en";

export const translations = {
  es: {
    app_title: "Terapia Visual",
    btn_start: "Iniciar Terapia",
    btn_stop: "Detener Terapia",
    btn_spanish: "Español",
    btn_english: "English",
    btn_reset: "Restablecer",

    // Layouts
    layout_label: "Disposición (Layout)",
    layout_vertical: "Vertical",
    layout_horizontal: "Horizontal",
    layout_checkerboard: "Ajedrez",
    layout_vertical4: "Vertical (4 Columnas)",

    // Zonas
    zone_title: "Zona",
    color_label: "Color",
    opacity_label: "Opacidad",

    // Estados
    status_started: "Terapia iniciada",
    status_stopped: "Terapia detenida",
    status_updated: "Configuración actualizada",
    status_reset: "Valores por defecto restaurados",
    error_generic: "Ha ocurrido un error inesperado",
  },
  en: {
    app_title: "Visual Therapy",
    btn_start: "Start Therapy",
    btn_stop: "Stop Therapy",
    btn_spanish: "Spanish",
    btn_english: "English",
    btn_reset: "Reset Defaults",

    // Layouts
    layout_label: "Layout",
    layout_vertical: "Vertical",
    layout_horizontal: "Horizontal",
    layout_checkerboard: "Checkerboard",
    layout_vertical4: "Vertical (4 Columns)",

    // Zonas
    zone_title: "Zone",
    color_label: "Color",
    opacity_label: "Opacity",

    // Estados
    status_started: "Therapy started",
    status_stopped: "Therapy stopped",
    status_updated: "Configuration updated",
    status_reset: "Default values restored",
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

export function applyTranslations() {
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
