import { listen } from "@tauri-apps/api/event";
import "./reader.css";
import { initSettingsPanel, updatePanelControls } from "./settings-panel";
import { setLanguage, applyTranslations } from "../../core/localization/i18n";

import {
  getAppSettings,
  notifyReadingWindowResized,
} from "../../core/services";

const contentDiv = document.getElementById("chaptercontent")!;
const zonesContainer = document.getElementById("zones-container")!;
let currentConfig: any = null;

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
listen("update-reading-view", (event: any) => {
  console.log("Datos recibidos de Rust:", event.payload);
  currentConfig = event.payload.config;

  contentDiv.innerHTML = event.payload.html_content;

  applyReadingSettings();
  renderZones();
  // Sincronizar con el panel
  updatePanelControls(currentConfig);
});

// Aplicar colores y fuentes al texto
function applyReadingSettings() {
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
  if (!currentConfig) return;
  zonesContainer.innerHTML = "";

  const w = window.innerWidth;
  const h = window.innerHeight;
  const layout = currentConfig.layout;
  const zones = currentConfig.zones_config;

  let rects: { x: number; y: number; w: number; h: number }[] = [];

  if (layout === "Vertical") {
    rects = [
      { x: 0, y: 0, w: w / 2, h: h },
      { x: w / 2, y: 0, w: w / 2, h: h },
    ];
  } else if (layout === "Horizontal") {
    rects = [
      { x: 0, y: 0, w: w, h: h / 2 },
      { x: 0, y: h / 2, w: w, h: h / 2 },
    ];
  } else if (layout === "Checkerboard") {
    rects = [
      { x: 0, y: 0, w: w / 2, h: h / 2 },
      { x: w / 2, y: 0, w: w / 2, h: h / 2 },
      { x: 0, y: h / 2, w: w / 2, h: h / 2 },
      { x: w / 2, y: h / 2, w: w / 2, h: h / 2 },
    ];
  } else if (layout === "Vertical4Columns") {
    const col = w / 4;
    rects = [
      { x: 0, y: 0, w: col, h },
      { x: col, y: 0, w: col, h },
      { x: col * 2, y: 0, w: col, h },
      { x: col * 3, y: 0, w: col, h },
    ];
  }

  rects.forEach((rect, i) => {
    if (!zones[i]) return;
    const div = document.createElement("div");
    div.style.position = "absolute";
    div.style.left = `${rect.x}px`;
    div.style.top = `${rect.y}px`;
    div.style.width = `${rect.w}px`;
    div.style.height = `${rect.h}px`;
    div.style.backgroundColor = zones[i].color;
    div.style.opacity = zones[i].opacity.toString();
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
