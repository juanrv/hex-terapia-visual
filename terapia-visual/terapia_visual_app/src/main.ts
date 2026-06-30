// main.ts
import { setLanguage, applyTranslations } from "./core/localization/i18n";
import {
  getAppSettings,
  exitApp,
  updateAppSettings,
} from "./core/services/settings";
import { switchTherapy } from "./ui/router";
import { showError } from "./ui/components/status";
import { listen } from "@tauri-apps/api/event";

async function init() {
  const statusDiv = document.getElementById("status");

  try {
    // Cargar idioma
    const settings = await getAppSettings();
    setLanguage(settings.language === "en" ? "en" : "es");
  } catch (err) {
    console.error("Error loading initial state:", err);
    showError(String(err), statusDiv);
  }

  // Listener para el cambio de idioma global
  listen("app-language-changed", (event: any) => {
    const lang = event.payload;
    setLanguage(lang);
    applyTranslations();

    window.dispatchEvent(new Event("language-changed"));
  });

  // Escuchar a la bandeja del sistema para navegar
  listen("navigate-view", (event: any) => {
    const view = event.payload;

    if (view === "overlay" || view === "reading") {
      switchTherapy(view);
    }
  });

  // Escuchar al atajo de teclado para abrir lectura
  listen("trigger-start-reading", async () => {
    await switchTherapy("reading");
    // Simulamos un click en el botón de abrir para que extraiga el texto y valide
    setTimeout(() => {
      const textarea = document.getElementById(
        "reading-input",
      ) as HTMLTextAreaElement;

      if (textarea && textarea.value.trim() !== "") {
        // Si ya hay texto, se simula el click para abrir la lectura
        const btn = document.getElementById("btn-start-reading");
        if (btn) btn.click();
      } else if (textarea) {
        textarea.focus();
      }
    }, 50);
  });

  // Configurar eventos de navegación
  document.getElementById("btn-nav-color")?.addEventListener("click", () => {
    switchTherapy("overlay");
  });

  document.getElementById("btn-nav-reading")?.addEventListener("click", () => {
    switchTherapy("reading");
  });

  // Cambio de idioma
  document.getElementById("btn-es")?.addEventListener("click", async () => {
    await updateAppSettings("es");
  });

  document.getElementById("btn-en")?.addEventListener("click", async () => {
    await updateAppSettings("en");
  });

  // Salir de la aplicación
  document.getElementById("btn-exit")?.addEventListener("click", async () => {
    try {
      await exitApp();
    } catch (err) {
      console.error("Error al cerrar la app:", err);
      showError(String(err), statusDiv);
    }
  });
}

window.addEventListener("DOMContentLoaded", init);
