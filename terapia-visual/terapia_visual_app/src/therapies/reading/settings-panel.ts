import { invoke } from "@tauri-apps/api/core";

export function initSettingsPanel() {
  const panel = document.getElementById("settings-panel")!;
  const btnToggle = document.getElementById("floating-btn")!;
  const inpFontSize = document.getElementById(
    "inp-font-size",
  ) as HTMLInputElement;
  const inpTextColor = document.getElementById(
    "inp-text-color",
  ) as HTMLInputElement;
  const inpBgColor = document.getElementById(
    "inp-bg-color",
  ) as HTMLInputElement;
  const inpLayout = document.getElementById("inp-layout") as HTMLSelectElement;

  // Alternar apertura del panel
  btnToggle.addEventListener("click", () => {
    panel.classList.toggle("open");
  });

  // Enviar cambios globales a Rust
  const saveReadingSettings = async () => {
    const newSettings = {
      font_size: parseInt(inpFontSize.value),
      text_color: inpTextColor.value.toUpperCase(),
      bg_color: inpBgColor.value.toUpperCase(),
      line_height: "1.65",
    };
    await invoke("cmd_update_reading_settings", { newSettings });
  };

  inpFontSize.addEventListener("change", saveReadingSettings);
  inpTextColor.addEventListener("change", saveReadingSettings);
  inpBgColor.addEventListener("change", saveReadingSettings);

  inpLayout.addEventListener("change", (e) => {
    invoke("cmd_change_reading_layout", {
      newLayout: (e.target as HTMLSelectElement).value,
    });
  });
}

export function updatePanelControls(config: any) {
  const inpFontSize = document.getElementById(
    "inp-font-size",
  ) as HTMLInputElement;
  const inpTextColor = document.getElementById(
    "inp-text-color",
  ) as HTMLInputElement;
  const inpBgColor = document.getElementById(
    "inp-bg-color",
  ) as HTMLInputElement;
  const inpLayout = document.getElementById("inp-layout") as HTMLSelectElement;
  const zonesControls = document.getElementById("zones-controls")!;

  // Sincronizar valores visuales
  inpFontSize.value = config.reading_settings.font_size;
  inpTextColor.value = config.reading_settings.text_color;
  inpBgColor.value = config.reading_settings.bg_color;
  inpLayout.value = config.layout;

  // Dibujar y conectar controles de zonas dinámicamente
  zonesControls.innerHTML = "";
  config.zones_config.forEach((zone: any, index: number) => {
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

    // Eventos de Rust para cada zona
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
