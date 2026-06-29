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
