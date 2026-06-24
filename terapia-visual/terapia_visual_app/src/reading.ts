import { invoke } from "@tauri-apps/api/core";
import { listen } from "@tauri-apps/api/event";

// Elementos del DOM
const panel = document.getElementById("settings-panel")!;
const btnToggle = document.getElementById("floating-btn")!;
const contentDiv = document.getElementById("chaptercontent")!;
const zonesContainer = document.getElementById("zones-container")!;

const inpFontSize = document.getElementById(
  "inp-font-size",
) as HTMLInputElement;
const inpTextColor = document.getElementById(
  "inp-text-color",
) as HTMLInputElement;
const inpBgColor = document.getElementById("inp-bg-color") as HTMLInputElement;
const inpLayout = document.getElementById("inp-layout") as HTMLSelectElement;
const zonesControls = document.getElementById("zones-controls")!;

let currentConfig: any = null;

// 1. Alternar Panel Lateral
btnToggle.addEventListener("click", () => {
  panel.classList.toggle("open");
});

// 2. Escuchar los eventos IPC que manda Rust
listen("update-reading-view", (event: any) => {
  console.log("Datos recibidos de Rust:", event.payload);
  currentConfig = event.payload.config;

  // Inyectar el texto
  contentDiv.innerHTML = event.payload.html_content;

  applyReadingSettings();
  renderZones();
  renderPanelControls();
});

// 3. Aplicar colores y fuentes al texto
function applyReadingSettings() {
  const settings = currentConfig.reading_settings;
  document.body.style.backgroundColor = settings.bg_color;

  contentDiv.style.fontSize = `${settings.font_size}px`;
  contentDiv.style.color = settings.text_color;
  contentDiv.style.lineHeight = settings.line_height;

  // Forzar el color a los párrafos internos
  contentDiv.querySelectorAll("p").forEach((p) => {
    p.style.color = settings.text_color;
    p.style.lineHeight = settings.line_height;
  });
}

// 4. Calcular y dibujar las zonas matemáticamente en el Frontend
function renderZones() {
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

  // Dibujar
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

// 5. Llenar el panel flotante
function renderPanelControls() {
  inpFontSize.value = currentConfig.reading_settings.font_size;
  inpTextColor.value = currentConfig.reading_settings.text_color;
  inpBgColor.value = currentConfig.reading_settings.bg_color;
  inpLayout.value = currentConfig.layout;

  zonesControls.innerHTML = "";
  currentConfig.zones_config.forEach((zone: any, index: number) => {
    const group = document.createElement("div");
    group.className = "control-group";
    group.innerHTML = `
      <label>Zona ${index + 1} - Color y Opacidad</label>
      <div style="display: flex; gap: 5px;">
        <input type="color" value="${zone.color}" id="z-color-${index}" style="flex: 1;">
        <input type="range" min="0" max="0.8" step="0.01" value="${zone.opacity}" id="z-op-${index}" style="flex: 2;">
      </div>
    `;
    zonesControls.appendChild(group);

    document
      .getElementById(`z-color-${index}`)
      ?.addEventListener("change", (e) => {
        invoke("cmd_update_reading_zone_color", {
          zoneIndex: index,
          newColor: (e.target as HTMLInputElement).value.toUpperCase(),
        });
      });
    document
      .getElementById(`z-op-${index}`)
      ?.addEventListener("change", (e) => {
        invoke("cmd_update_reading_zone_opacity", {
          zoneIndex: index,
          newOpacity: parseFloat((e.target as HTMLInputElement).value),
        });
      });
  });
}

// 6. Enviar cambios a Rust
async function saveReadingSettings() {
  const newSettings = {
    font_size: parseInt(inpFontSize.value),
    text_color: inpTextColor.value.toUpperCase(),
    bg_color: inpBgColor.value.toUpperCase(),
    line_height: "1.65",
  };
  await invoke("cmd_update_reading_settings", { newSettings });
}

inpFontSize.addEventListener("change", saveReadingSettings);
inpTextColor.addEventListener("change", saveReadingSettings);
inpBgColor.addEventListener("change", saveReadingSettings);
inpLayout.addEventListener("change", (e) => {
  invoke("cmd_change_reading_layout", {
    newLayout: (e.target as HTMLSelectElement).value,
  });
});

// 7. Si el usuario redimensiona la ventana, recalculamos las zonas instantáneamente
window.addEventListener("resize", () => {
  if (currentConfig) renderZones();
});
