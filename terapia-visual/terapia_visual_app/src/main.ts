import { invoke } from "@tauri-apps/api/core";
import { setLanguage, translate, type Language } from "./localization/i18n";

// Funcion para mosntrar mensajes de estado (traducidos)
function showStatus(
  messageKey: keyof typeof import("./localization/i18n").translations.es,
) {
  const statusDiv = document.getElementById("status");
  if (statusDiv) statusDiv.innerText = translate(messageKey);
}

// Cargar idioma guardado al iniciar
async function loadSavedLanguage() {
  try {
    const settings: { language: string } = await invoke("cmd_get_app_settings");
    const lang = settings.language === "en" ? "en" : "es";
    setLanguage(lang);
    // TODO: Actualizar el tooltip de la bandeja requiere exponer un comando en el tauri
  } catch (err) {
    console.error("Error loading app settings: ", err);
  }
}

// Cambiar el idioma
async function changeLanguage(lang: Language) {
  try {
    await invoke("cmd_update_tray_tooltip");
    await invoke("cmd_update_app_settings", {
      // Tauri traduce el newSettings a new_settings
      newSettings: { language: lang },
    });
    setLanguage(lang);
    console.log("Language changed: ", lang);
  } catch (err) {
    console.error("Error updating language: ", err);
    showStatus("error_generic");
  }
}

// Función para obtener configuración actual
async function getTherapyConfig() {
  try {
    const config = await invoke("cmd_get_therapy_config");
    console.log("Current config:", config);
  } catch (error) {
    console.error("Error getting configuration:", error);
    showStatus("error_generic");
  }
}

// Iniciar terapia
async function startTherapy() {
  try {
    // Usar resolución de pantalla real
    const screenWidth = window.screen.width;
    const screenHeight = window.screen.height;
    await invoke("cmd_start_therapy", { screenWidth, screenHeight });
    console.log("Therapy started");
    showStatus("status_started");
  } catch (error) {
    console.error("Error starting:", error);
    showStatus("error_generic");
  }
}

// Detener terapia
async function stopTherapy() {
  try {
    await invoke("cmd_stop_therapy");
    console.log("Therapy stopped");
    showStatus("status_stopped");
  } catch (error) {
    console.error("Error stopping:", error);
    showStatus("error_generic");
  }
}

// Actualizar configuración (ejemplo con valores fijos)
async function updateTherapyConfig() {
  const newConfig = {
    therapy_type: "ColorDivision",
    layout: "Vertical",
    zones_config: [
      { color: "#FF0000", opacity: 0.8 },
      { color: "#0000FF", opacity: 0.6 },
    ],
  };
  try {
    const screenWidth = window.screen.width;
    const screenHeight = window.screen.height;
    await invoke("cmd_update_therapy_config", {
      // Tauri traduce el newConfig a new_config
      newConfig: newConfig,
      screenWidth,
      screenHeight,
    });
    console.log("Therapy config updated");
    showStatus("status_updated");
  } catch (error) {
    console.error("Error updating:", error);
    showStatus("error_generic");
  }
}

// Asignar eventos cuando el DOM esté listo y cargar el idioma actual
window.addEventListener("DOMContentLoaded", async () => {
  await loadSavedLanguage();

  const btnStart = document.getElementById("btn-start");
  const btnStop = document.getElementById("btn-stop");
  const btnUpdate = document.getElementById("btn-update");
  const btnGet = document.getElementById("btn-get");
  const btnEn = document.getElementById("btn-en");
  const btnEs = document.getElementById("btn-es");

  if (btnStart) btnStart.addEventListener("click", startTherapy);
  if (btnStop) btnStop.addEventListener("click", stopTherapy);
  if (btnUpdate) btnUpdate.addEventListener("click", updateTherapyConfig);
  if (btnGet) btnGet.addEventListener("click", getTherapyConfig);
  if (btnEn) btnEn.addEventListener("click", () => changeLanguage("en"));
  if (btnEs) btnEs.addEventListener("click", () => changeLanguage("es"));
});
