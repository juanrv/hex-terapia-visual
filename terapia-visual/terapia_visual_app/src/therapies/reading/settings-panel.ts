import {
  setLanguage,
  applyTranslations,
  translate,
  type Language,
} from "../../core/localization/i18n";

import {
  changeReadingLayout,
  getReadingConfig,
  resetReadingConfig,
  stopReading,
  updateAppSettings,
  updateReadingSettings,
  updateReadingZoneColor,
  updateReadingZoneOpacity,
} from "../../core/services";
import { ReadingConfig } from "../../core/types";

export function initSettingsPanel() {
  const panel = document.getElementById("settings-panel")!;
  const btnToggle = document.getElementById("floating-btn")!;
  const btnReset = document.getElementById("btn-reset-reading-panel")!;
  const btnClose = document.getElementById("btn-close-panel")!;
  const btnCloseReading = document.getElementById("btn-close-reading-window")!;
  const btnEs = document.getElementById("btn-es-reading")!;
  const btnEn = document.getElementById("btn-en-reading")!;

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

  // Abrir panel: Oculta el flotante y muestra el panel
  btnToggle.addEventListener("click", () => {
    panel.classList.add("open");
    btnToggle.classList.add("hidden");
  });

  // Cerrar panel: Oculta el panel y devuelve el flotante
  btnClose.addEventListener("click", () => {
    panel.classList.remove("open");
    btnToggle.classList.remove("hidden");
  });

  // Restablecer valor por defecto
  btnReset.addEventListener("click", async () => {
    try {
      await resetReadingConfig();
    } catch (err) {
      console.error("Error al restablecer", err);
    }
  });

  // Cerrar ventana de lectura
  btnCloseReading.addEventListener("click", async () => {
    try {
      await stopReading();
    } catch (e) {
      console.error("Error al cerrar la ventana:", e);
    }
  });

  // Logica de cambio de idioma
  const changeLang = async (lang: Language) => {
    await updateAppSettings(lang);
    setLanguage(lang);
    applyTranslations();

    getReadingConfig().then((config) => updatePanelControls(config));
  };

  btnEs.addEventListener("click", () => changeLang("es"));
  btnEn.addEventListener("click", () => changeLang("en"));

  // Enviar cambios globales a Rust
  const saveReadingSettings = async () => {
    const newSettings = {
      font_size: parseInt(inpFontSize.value),
      text_color: inpTextColor.value.toUpperCase(),
      bg_color: inpBgColor.value.toUpperCase(),
      line_height: "1.65",
    };
    await updateReadingSettings(newSettings);
  };

  inpFontSize.addEventListener("change", saveReadingSettings);
  inpTextColor.addEventListener("change", saveReadingSettings);
  inpBgColor.addEventListener("change", saveReadingSettings);

  inpLayout.addEventListener("change", (e) => {
    changeReadingLayout((e.target as HTMLSelectElement).value);
  });
}

export function updatePanelControls(config: ReadingConfig) {
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

  // Sincronizar valores visuales globales
  inpFontSize.value = config.reading_settings.font_size.toString();
  inpTextColor.value = config.reading_settings.text_color;
  inpBgColor.value = config.reading_settings.bg_color;
  inpLayout.value = config.layout;

  // Dibujar y conectar controles de zonas dinámicamente
  zonesControls.innerHTML = "";
  config.zones_config.forEach((zone: any, index: number) => {
    const group = document.createElement("div");
    group.className = "control-group";

    // Calcular porcentaje para mostrarlo
    const percentage = Math.round(zone.opacity * 100);

    group.innerHTML = `
      <label id="z-lbl-${index}">${translate("zone_title")} ${index + 1} - ${translate("opacity_label")} (${percentage}%)</label>
      <div style="display: flex; gap: 10px; align-items: center;">
        <input type="color" value="${zone.color}" id="z-color-${index}" style="flex: 1;">
        <input type="range" min="0" max="0.8" step="0.01" value="${zone.opacity}" id="z-op-${index}" style="flex: 2;">
      </div>
    `;
    zonesControls.appendChild(group);

    // Eventos del color
    document
      .getElementById(`z-color-${index}`)
      ?.addEventListener("change", (e) => {
        updateReadingZoneColor(
          index,
          (e.target as HTMLInputElement).value.toUpperCase(),
        );
      });

    // Eventos de la opacidad
    const opInput = document.getElementById(
      `z-op-${index}`,
    ) as HTMLInputElement;
    const opLabel = document.getElementById(`z-lbl-${index}`)!;

    // El evento 'input' actualiza el texto EN TIEMPO REAL mientras arrastras
    opInput?.addEventListener("input", (e) => {
      const val = parseFloat((e.target as HTMLInputElement).value);
      opLabel.innerText = `${translate("zone_title")} ${index + 1} - ${translate("opacity_label")} (${Math.round(val * 100)}%)`;
    });

    // El evento 'change' envía el valor a Rust cuando sueltas el click
    opInput?.addEventListener("change", (e) => {
      updateReadingZoneOpacity(
        index,
        parseFloat((e.target as HTMLInputElement).value),
      );
    });
  });
}
