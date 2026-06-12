import { invoke } from "@tauri-apps/api/core";
import { setLanguage, translate, type Language } from "./localization/i18n";

// Variables globales de estado del Frontend
let currentConfig: any = null;

// Referencias a elementos del DOM
const statusDiv = document.getElementById("status");
const layoutSelect = document.getElementById(
  "layout-select",
) as HTMLSelectElement;
const zonesContainer = document.getElementById("zones-container");

// Utilidad para mostrar estado
function showStatus(
  messageKey: keyof typeof import("./localization/i18n").translations.es,
) {
  if (statusDiv) statusDiv.innerText = translate(messageKey);
  setTimeout(() => {
    if (statusDiv) statusDiv.innerText = "";
  }, 3000); // Limpiar a los 3 seg
}

async function loadInitialState() {
  try {
    // Cargar idioma
    const settings: { language: string } = await invoke("cmd_get_app_settings");
    setLanguage(settings.language === "en" ? "en" : "es");

    // Cargar configuracion de Terapia
    currentConfig = await invoke("cmd_get_therapy_config");

    // Renderizar la UI
    layoutSelect.value = currentConfig.layout;
    renderZoneControls();
  } catch (err) {
    console.error("Error loading initial state: ", err);
  }
}

async function changeLanguage(lang: Language) {
  try {
    await invoke("cmd_update_app_settings", {
      newSettings: { language: lang },
    });
    setLanguage(lang);
    renderZoneControls(); // Re-renderizar para traducir las zonas
  } catch (err) {
    console.error("Error updating language: ", err);
    showStatus("error_generic");
  }
}

function renderZoneControls() {
  if (!zonesContainer || !currentConfig) return;
  zonesContainer.innerHTML = ""; // Limpiar contenedor

  currentConfig.zones_config.forEach((zone: any, index: number) => {
    // Crear la tarjeta de la zona
    const card = document.createElement("div");
    card.className = "zone-card";

    // Titulo de la zona
    const title = document.createElement("h4");
    title.innerText = `${translate("zone_title")} ${index + 1}`;
    card.appendChild(title);

    // Control de color
    const colorGroup = document.createElement("div");
    colorGroup.className = "control-group";

    const colorLabel = document.createElement("label");
    colorLabel.innerText = translate("color_label");

    const colorInput = document.createElement("input");
    colorInput.type = "color";
    colorInput.value = zone.color;

    // Cuando el usuario elige un color y cierra el picker ("change")
    colorInput.addEventListener("change", (e) => {
      updateZoneConfig(index, (e.target as HTMLInputElement).value, null);
    });

    colorGroup.appendChild(colorLabel);
    colorGroup.appendChild(colorInput);
    card.appendChild(colorGroup);

    // Control de Opacidad
    const opacityGroup = document.createElement("div");
    opacityGroup.className = "control-group";

    const opacityLabel = document.createElement("label");
    // Mostrar el % actual
    opacityLabel.innerText = `${translate("opacity_label")} (${Math.round(zone.opacity * 100)}%)`;

    const opacityInput = document.createElement("input");
    opacityInput.type = "range";
    opacityInput.min = "0";
    opacityInput.max = "0.8";
    opacityInput.step = "0.01";
    opacityInput.value = zone.opacity;

    // Al soltar el slider ("change")
    opacityInput.addEventListener("change", (e) => {
      const val = parseFloat((e.target as HTMLInputElement).value);
      opacityLabel.innerText = `${translate("opacity_label")} (${Math.round(val * 100)}%)`;
      updateZoneConfig(index, null, val);
    });

    opacityGroup.appendChild(opacityLabel);
    opacityGroup.appendChild(opacityInput);
    card.appendChild(opacityGroup);

    // Añadir tarjeta al contenedor
    zonesContainer.appendChild(card);
  });
}

// Comunicación con el Backend (Rust)
async function startTherapy() {
  try {
    const screenWidth = window.screen.width;
    const screenHeight = window.screen.height;
    await invoke("cmd_start_therapy", { screenWidth, screenHeight });
    showStatus("status_started");
  } catch (error) {
    console.error("Error starting:", error);
    showStatus("error_generic");
  }
}

async function stopTherapy() {
  try {
    await invoke("cmd_stop_therapy");
    showStatus("status_stopped");
  } catch (error) {
    console.error("Error stopping:", error);
    showStatus("error_generic");
  }
}

async function resetTherapy() {
  try {
    const screenWidth = window.screen.width;
    const screenHeight = window.screen.height;

    // LLamar al backend para que reinicie todo y retorne configuracion limpia
    currentConfig = await invoke("cmd_reset_therapy_config", {
      screenWidth,
      screenHeight,
    });

    // Sincronizar UI con los nuevos valores
    layoutSelect.value = currentConfig.layout;
    renderZoneControls();
    showStatus("status_reset");
  } catch (error) {
    console.error("Error reseteando configuracion:", error);
    showStatus("error_generic");
  }
}

async function handleLayoutChange(e: Event) {
  const newLayout = (e.target as HTMLSelectElement).value;

  try {
    const screenWidth = window.screen.width;
    const screenHeight = window.screen.height;

    await invoke("cmd_change_layout", { newLayout, screenWidth, screenHeight });

    currentConfig = await invoke("cmd_get_therapy_config");
    renderZoneControls();
    showStatus("status_updated");
  } catch (error) {
    console.error("Error cambiando layout:", error);
    showStatus("error_generic");
  }
}

async function updateZoneConfig(
  index: number,
  newColor: string | null,
  newOpacity: number | null,
) {
  try {
    const screenWidth = window.screen.width;
    const screenHeight = window.screen.height;

    // Mandar el comando especifico segun lo que se toco
    if (newColor !== null) {
      await invoke("cmd_update_zone_color", {
        zoneIndex: index,
        newColor: newColor.toUpperCase(),
        screenWidth,
        screenHeight,
      });
    }
    if (newOpacity !== null) {
      await invoke("cmd_update_zone_opacity", {
        zoneIndex: index,
        newOpacity,
        screenWidth,
        screenHeight,
      });
    }

    // Refrescar la UI por si el backend sincronizo colores (como en el Ajedrez)
    currentConfig = await invoke("cmd_get_therapy_config");
    renderZoneControls();
    showStatus("status_updated");
  } catch (error) {
    console.error("Error actualizando zona:", error);
    showStatus("error_generic");
  }
}

// Event Listeners

window.addEventListener("DOMContentLoaded", async () => {
  await loadInitialState();

  document.getElementById("btn-start")?.addEventListener("click", startTherapy);
  document.getElementById("btn-stop")?.addEventListener("click", stopTherapy);
  document.getElementById("btn-reset")?.addEventListener("click", resetTherapy);
  document
    .getElementById("btn-es")
    ?.addEventListener("click", () => changeLanguage("es"));
  document
    .getElementById("btn-en")
    ?.addEventListener("click", () => changeLanguage("en"));

  layoutSelect?.addEventListener("change", handleLayoutChange);
});
