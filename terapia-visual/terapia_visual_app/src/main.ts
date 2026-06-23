// src/main.ts
import { setLanguage, translate, type Language } from "./localization/i18n";
import {
  getAppSettings,
  updateAppSettings,
  exitApp,
  getOverlayConfig,
  startOverlay,
  stopOverlay,
  resetOverlayConfig,
  changeOverlayLayout,
  updateOverlayZoneColor,
  updateOverlayZoneOpacity,
} from "./services";
import { showStatus, showError } from "./services/ui";

// Variables globales de estado del Frontend
let currentConfig: any = null;

// Referencias a elementos del DOM
const statusDiv = document.getElementById("status");
const layoutSelect = document.getElementById(
  "layout-select",
) as HTMLSelectElement;
const zonesContainer = document.getElementById("zones-container");

// --- Navegación (SPA) ---
function switchView(viewId: "view-home" | "view-color" | "view-reading") {
  document.getElementById("view-home")!.style.display = "none";
  document.getElementById("view-color")!.style.display = "none";
  document.getElementById("view-reading")!.style.display = "none";

  document.getElementById(viewId)!.style.display = "block";
  if (statusDiv) statusDiv.innerText = "";
}

// --- Funciones de UI ---

function renderZoneControls() {
  if (!zonesContainer || !currentConfig) return;
  zonesContainer.innerHTML = "";

  currentConfig.zones_config.forEach((zone: any, index: number) => {
    const card = document.createElement("div");
    card.className = "zone-card";

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
    opacityLabel.innerText = `${translate("opacity_label")} (${Math.round(zone.opacity * 100)}%)`;

    const opacityInput = document.createElement("input");
    opacityInput.type = "range";
    opacityInput.min = "0";
    opacityInput.max = "0.8";
    opacityInput.step = "0.01";
    opacityInput.value = zone.opacity;

    opacityInput.addEventListener("change", (e) => {
      const val = parseFloat((e.target as HTMLInputElement).value);
      opacityLabel.innerText = `${translate("opacity_label")} (${Math.round(val * 100)}%)`;
      updateZoneConfig(index, null, val);
    });

    opacityGroup.appendChild(opacityLabel);
    opacityGroup.appendChild(opacityInput);
    card.appendChild(opacityGroup);

    zonesContainer.appendChild(card);
  });
}

async function updateZoneConfig(
  index: number,
  newColor: string | null,
  newOpacity: number | null,
) {
  try {
    const screenWidth = window.screen.width;
    const screenHeight = window.screen.height;

    if (newColor !== null) {
      await updateOverlayZoneColor(
        index,
        newColor.toUpperCase(),
        screenWidth,
        screenHeight,
      );
    }
    if (newOpacity !== null) {
      await updateOverlayZoneOpacity(
        index,
        newOpacity,
        screenWidth,
        screenHeight,
      );
    }

    currentConfig = await getOverlayConfig();
    renderZoneControls();
    showStatus("status_updated", statusDiv);
  } catch (error) {
    console.error("Error actualizando zona:", error);
    showError(String(error), statusDiv);
  }
}

// --- Funciones de carga inicial ---

async function loadInitialState() {
  try {
    const settings = await getAppSettings();
    setLanguage(settings.language === "en" ? "en" : "es");

    currentConfig = await getOverlayConfig();
    layoutSelect.value = currentConfig.layout;
    renderZoneControls();
  } catch (err) {
    console.error("Error loading initial state: ", err);
  }
}

// --- Manejadores de eventos ---

async function changeLanguage(lang: Language) {
  try {
    await updateAppSettings(lang);
    setLanguage(lang);
    renderZoneControls();
  } catch (error) {
    console.error("Error updating language: ", error);
    showError(String(error), statusDiv);
  }
}

async function startTherapy() {
  try {
    const screenWidth = window.screen.width;
    const screenHeight = window.screen.height;
    await startOverlay(screenWidth, screenHeight);
    showStatus("status_started", statusDiv);
  } catch (error) {
    console.error("Error starting:", error);
    showError(String(error), statusDiv);
  }
}

async function stopTherapy() {
  try {
    await stopOverlay();
    showStatus("status_stopped", statusDiv);
  } catch (error) {
    console.error("Error stopping:", error);
    showError(String(error), statusDiv);
  }
}

async function resetTherapy() {
  try {
    const screenWidth = window.screen.width;
    const screenHeight = window.screen.height;

    currentConfig = await resetOverlayConfig(screenWidth, screenHeight);

    layoutSelect.value = currentConfig.layout;
    renderZoneControls();
    showStatus("status_reset", statusDiv);
  } catch (error) {
    console.error("Error reseteando configuracion:", error);
    showError(String(error), statusDiv);
  }
}

async function handleLayoutChange(e: Event) {
  const newLayout = (e.target as HTMLSelectElement).value;

  try {
    const screenWidth = window.screen.width;
    const screenHeight = window.screen.height;

    await changeOverlayLayout(newLayout, screenWidth, screenHeight);

    currentConfig = await getOverlayConfig();
    renderZoneControls();
    showStatus("status_updated", statusDiv);
  } catch (error) {
    console.error("Error cambiando layout:", error);
    showError(String(error), statusDiv);
  }
}

async function handleExitApp() {
  try {
    await exitApp();
  } catch (error) {
    console.error("No se pudo cerrar la app:", error);
    showError(String(error), statusDiv);
  }
}

// --- Event Listeners ---

window.addEventListener("DOMContentLoaded", async () => {
  await loadInitialState();

  // ---- Event Listeners de Navegacion ----
  document
    .getElementById("btn-nav-color")
    ?.addEventListener("click", () => switchView("view-color"));
  document
    .getElementById("btn-nav-reading")
    ?.addEventListener("click", () => switchView("view-reading"));
  document.getElementById("btn-exit")?.addEventListener("click", handleExitApp);

  document.querySelectorAll(".btn-back").forEach((btn) => {
    btn.addEventListener("click", () => switchView("view-home"));
  });

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
