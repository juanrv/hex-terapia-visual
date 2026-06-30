import { listen } from "@tauri-apps/api/event";
import "./reader.css";
import { initSettingsPanel, updatePanelControls } from "./settings-panel";
import { setLanguage, applyTranslations } from "../../core/localization/i18n";

import {
  getAppSettings,
  notifyReadingWindowResized,
} from "../../core/services";
import { ReadingConfig, ReadingPayload, Zone } from "../../core/types";

const contentDiv = document.getElementById("chaptercontent")!;
const zonesContainer = document.getElementById("zones-container")!;

let currentConfig: ReadingConfig | null = null;
let currentZones: Zone[] = [];

// Inicializar los listeners del panel lateral
initSettingsPanel();

// Preguntar a Rust el idioma actual al cargar la ventana
window.addEventListener("DOMContentLoaded", async () => {
  try {
    const settings = await getAppSettings();
    setLanguage(settings.language === "en" ? "en" : "es");
    applyTranslations();
  } catch (error) {
    console.error("Error al cargar idioma:", error);
  }
});

// Escuchar los eventos IPC que manda Rust
listen<ReadingPayload>("update-reading-view", (event) => {
  console.log("Datos recibidos de Rust:", event.payload);

  currentConfig = event.payload.config as ReadingConfig;
  currentZones = event.payload.zones as Zone[];

  contentDiv.innerHTML = event.payload.html_content;

  if (currentConfig) {
    applyReadingSettings();
    renderZones();
    updatePanelControls(currentConfig);
  }
});

// Escuchar cambios de idioma
listen("app-language-changed", (event: any) => {
  const lang = event.payload;
  setLanguage(lang);
  applyTranslations();

  if (currentConfig) {
    updatePanelControls(currentConfig);
  }
});

// Aplicar colores y fuentes al texto
function applyReadingSettings() {
  if (!currentConfig) return;

  const settings = currentConfig.reading_settings;
  document.body.style.backgroundColor = settings.bg_color;

  contentDiv.style.fontSize = `${settings.font_size}px`;
  contentDiv.style.color = settings.text_color;
  contentDiv.style.lineHeight = settings.line_height;

  contentDiv.querySelectorAll("p").forEach((p) => {
    p.style.color = settings.text_color;
    p.style.lineHeight = settings.line_height;
  });
}

// Calcular y dibujar las zonas
function renderZones() {
  if (!currentConfig || currentZones.length === 0) return;
  zonesContainer.innerHTML = "";

  currentZones.forEach((zone: Zone) => {
    const div = document.createElement("div");
    div.style.position = "absolute";
    div.style.left = `${zone.rect.x}px`;
    div.style.top = `${zone.rect.y}px`;
    div.style.width = `${zone.rect.width}px`;
    div.style.height = `${zone.rect.height}px`;
    div.style.backgroundColor = zone.color;
    div.style.opacity = zone.opacity.toString();

    zonesContainer.appendChild(div);
  });
}

// Avisar a Rust cuando se redimensiona la ventana
let resizeTimeout: number | undefined;
window.addEventListener("resize", () => {
  clearTimeout(resizeTimeout);
  resizeTimeout = window.setTimeout(() => {
    notifyReadingWindowResized().catch(console.error);
  }, 150);
  // Redibujar instantaneamente en local para que no haya parpadeos
  renderZones();
});
