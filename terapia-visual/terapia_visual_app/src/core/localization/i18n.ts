export type Language = "es" | "en";

export const translations = {
  es: {
    // Titulo de la App
    app_title: "Terapia Visual",

    // Menu principal
    menu_color_division: "Terapia de División de Color",
    menu_reading: "Terapia de Lectura",

    // Panel Lateral
    reading_settings_title: "Ajustes de Lectura",
    font_size_label: "Tamaño de Fuente (px)",
    text_color_label: "Color de Letra",
    bg_color_label: "Color de Fondo",
    btn_close_window: "Cerrar Ventana",

    // Botones
    btn_start: "Iniciar Terapia",
    btn_stop: "Detener Terapia",
    btn_spanish: "Español",
    btn_english: "English",
    btn_reset: "Restablecer",
    btn_start_reading: "Abrir Ventana de Lectura",
    btn_stop_reading: "Cerrar Ventana",
    btn_back: "← Volver al Menú",
    btn_exit: "Salir de la Aplicación",

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

    // Placeholders
    reading_input_placeholder:
      "Pega aquí el contenido de tu novela, artículo o documento HTML...",

    // Estados
    status_started: "Terapia iniciada",
    status_stopped: "Terapia detenida",
    status_updated: "Configuración actualizada",
    status_reset: "Valores por defecto restaurados",
    status_reading_started: "Ventana de lectura abierta",
    status_reading_stopped: "Ventana de lectura cerrada",
    error_generic: "Ha ocurrido un error inesperado",
  },
  en: {
    //Titulo de la App
    app_title: "Visual Therapy",

    // Menu
    menu_color_division: "Color Division Therapy",
    menu_reading: "Reading Therapy",

    // Panel lateral
    reading_settings_title: "Reading Settings",
    font_size_label: "Font Size (px)",
    text_color_label: "Text Color",
    bg_color_label: "Background Color",
    btn_close_window: "Close Window",

    // Botones
    btn_start: "Start Therapy",
    btn_stop: "Stop Therapy",
    btn_spanish: "Spanish",
    btn_english: "English",
    btn_reset: "Reset Defaults",
    btn_start_reading: "Open Reading Window",
    btn_stop_reading: "Close Window",
    btn_back: "← Back to Menu",
    btn_exit: "Exit Application",

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

    // Placeholders
    reading_input_placeholder:
      "Paste your novel, article or HTML document here...",

    // Estados
    status_started: "Therapy started",
    status_stopped: "Therapy stopped",
    status_updated: "Configuration updated",
    status_reset: "Default values restored",
    status_reading_started: "Reading window opened",
    status_reading_stopped: "Reading window closed",
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
  // Actualizar titulo de la app
  document.title = translate("app_title");
  // Actualizar elementos con la etiqueta `data-i18n`
  document.querySelectorAll("[data-i18n]").forEach((element) => {
    const key = element.getAttribute("data-i18n");
    if (key && element instanceof HTMLElement) {
      // Si el elemento es un text area, se actualiza su placeholder
      if (
        element instanceof HTMLTextAreaElement ||
        element instanceof HTMLInputElement
      ) {
        element.placeholder = translate(key as any);
      } else {
        element.innerText = translate(key as any);
      }
    }
  });
}
