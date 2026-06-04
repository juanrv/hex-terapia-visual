// import { invoke } from "@tauri-apps/api/core";
// import { setLanguage, translate, type Language } from "./localization/i18n";

// // Funcion para mosntrar mensajes de estado (traducidos)
// function showStatus(
//   messageKey: keyof typeof import("./localization/i18n").translations.es,
// ) {
//   const statusDiv = document.getElementById("status");
//   if (statusDiv) statusDiv.innerText = translate(messageKey);
// }

// // Cargar idioma guardado al iniciar
// async function loadSavedLanguage() {
//   try {
//     const settings: { language: string } = await invoke("cmd_get_app_settings");
//     const lang = settings.language === "en" ? "en" : "es";
//     setLanguage(lang);
//     // TODO: Actualizar el tooltip de la bandeja requiere exponer un comando en el tauri
//   } catch (err) {
//     console.error("Error loading app settings: ", err);
//   }
// }

// // Cambiar el idioma
// async function changeLanguage(lang: Language) {
//   try {
//     await invoke("cmd_update_app_settings", {
//       // Tauri traduce el newSettings a new_settings
//       newSettings: { language: lang },
//     });
//     setLanguage(lang);
//     console.log("Language changed: ", lang);
//   } catch (err) {
//     console.error("Error updating language: ", err);
//     showStatus("error_generic");
//   }
// }

// // Función para obtener configuración actual
// async function getTherapyConfig() {
//   try {
//     const config = await invoke("cmd_get_therapy_config");
//     console.log("Current config:", config);
//   } catch (error) {
//     console.error("Error getting configuration:", error);
//     showStatus("error_generic");
//   }
// }

// // Iniciar terapia
// async function startTherapy() {
//   try {
//     // Usar resolución de pantalla real
//     const screenWidth = window.screen.width;
//     const screenHeight = window.screen.height;
//     await invoke("cmd_start_therapy", { screenWidth, screenHeight });
//     console.log("Therapy started");
//     showStatus("status_started");
//   } catch (error) {
//     console.error("Error starting:", error);
//     showStatus("error_generic");
//   }
// }

// // Detener terapia
// async function stopTherapy() {
//   try {
//     await invoke("cmd_stop_therapy");
//     console.log("Therapy stopped");
//     showStatus("status_stopped");
//   } catch (error) {
//     console.error("Error stopping:", error);
//     showStatus("error_generic");
//   }
// }

// // Actualizar configuración (ejemplo con valores fijos)
// async function updateTherapyConfig() {
//   const newConfig = {
//     therapy_type: "ColorDivision",
//     layout: "Vertical",
//     zones_config: [
//       { color: "#FF0000", opacity: 0.8 },
//       { color: "#0000FF", opacity: 0.6 },
//     ],
//   };
//   try {
//     const screenWidth = window.screen.width;
//     const screenHeight = window.screen.height;
//     await invoke("cmd_update_therapy_config", {
//       // Tauri traduce el newConfig a new_config
//       newConfig: newConfig,
//       screenWidth,
//       screenHeight,
//     });
//     console.log("Therapy config updated");
//     showStatus("status_updated");
//   } catch (error) {
//     console.error("Error updating:", error);
//     showStatus("error_generic");
//   }
// }

// // Obtener la configuracion actual
// async function getCurrentTherapyConfig(): Promise<any> {
//   try {
//     return await invoke("cmd_get_therapy_config");
//   } catch (error) {
//     console.error("Error al obtener la configuracion:", error);
//     return null;
//   }
// }

// // Cambiar el layout actual
// async function setLayout(layout: "Vertical" | "Horizontal") {
//   const currentConfig = await getCurrentTherapyConfig();
//   if (!currentConfig) return;

//   const newConfig = {
//     ...currentConfig,
//     layout: layout,
//   };

//   const screenWidth = window.screen.width;
//   const screenHeight = window.screen.height;

//   try {
//     await invoke("cmd_update_therapy_config", {
//       newConfig,
//       screenWidth,
//       screenHeight,
//     });
//     console.log(`Layout cambiado a ${layout}`);
//   } catch (error) {
//     console.error("Error al cambiar layout:", error);
//   }
// }

// // Asignar eventos cuando el DOM esté listo y cargar el idioma actual
// window.addEventListener("DOMContentLoaded", async () => {
//   await loadSavedLanguage();

//   const btnStart = document.getElementById("btn-start");
//   const btnStop = document.getElementById("btn-stop");
//   const btnUpdate = document.getElementById("btn-update");
//   const btnGet = document.getElementById("btn-get");
//   const btnEn = document.getElementById("btn-en");
//   const btnEs = document.getElementById("btn-es");
//   const btnVertical = document.getElementById("btn-layout-vertical");
//   const btnHorizontal = document.getElementById("btn-layout-horizontal");

//   if (btnStart) btnStart.addEventListener("click", startTherapy);
//   if (btnStop) btnStop.addEventListener("click", stopTherapy);
//   if (btnUpdate) btnUpdate.addEventListener("click", updateTherapyConfig);
//   if (btnGet) btnGet.addEventListener("click", getTherapyConfig);
//   if (btnEn) btnEn.addEventListener("click", () => changeLanguage("en"));
//   if (btnEs) btnEs.addEventListener("click", () => changeLanguage("es"));
//   if (btnVertical)
//     btnVertical.addEventListener("click", () => setLayout("Vertical"));
//   if (btnHorizontal)
//     btnHorizontal.addEventListener("click", () => setLayout("Horizontal"));
// });

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

// ----------------------------------------------------
// Lógica de Inicialización e Idiomas
// ----------------------------------------------------
async function loadInitialState() {
  try {
    // 1. Cargar idioma
    const settings: { language: string } = await invoke("cmd_get_app_settings");
    setLanguage(settings.language === "en" ? "en" : "es");

    // 2. Cargar configuración de Terapia
    currentConfig = await invoke("cmd_get_therapy_config");

    // 3. Renderizar la UI
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

// ----------------------------------------------------
// Generación Dinámica de Controles (Color / Opacidad)
// ----------------------------------------------------
function renderZoneControls() {
  if (!zonesContainer || !currentConfig) return;
  zonesContainer.innerHTML = ""; // Limpiar contenedor

  currentConfig.zones_config.forEach((zone: any, index: number) => {
    // Crear la tarjeta de la zona
    const card = document.createElement("div");
    card.className = "zone-card";

    // Título de la zona
    const title = document.createElement("h4");
    title.innerText = `${translate("zone_title")} ${index + 1}`;
    card.appendChild(title);

    // --- Control de Color ---
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

    // --- Control de Opacidad ---
    const opacityGroup = document.createElement("div");
    opacityGroup.className = "control-group";

    const opacityLabel = document.createElement("label");
    // Mostramos el % actual
    opacityLabel.innerText = `${translate("opacity_label")} (${Math.round(zone.opacity * 100)}%)`;

    const opacityInput = document.createElement("input");
    opacityInput.type = "range";
    opacityInput.min = "0";
    opacityInput.max = "0.8";
    opacityInput.step = "0.01";
    opacityInput.value = zone.opacity;

    // Al soltar el slider ("change" es mejor que "input" para no saturar IPC)
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

// ----------------------------------------------------
// Comunicación con el Backend (Rust)
// ----------------------------------------------------
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

    // Mandamos el comando específico según lo que se tocó
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

    // Refrescamos la UI por si el backend sincronizó colores (como en el Ajedrez)
    currentConfig = await invoke("cmd_get_therapy_config");
    renderZoneControls();
    showStatus("status_updated");
  } catch (error) {
    console.error("Error actualizando zona:", error);
    showStatus("error_generic");
  }
}

// ----------------------------------------------------
// Event Listeners
// ----------------------------------------------------
window.addEventListener("DOMContentLoaded", async () => {
  await loadInitialState();

  document.getElementById("btn-start")?.addEventListener("click", startTherapy);
  document.getElementById("btn-stop")?.addEventListener("click", stopTherapy);
  document
    .getElementById("btn-es")
    ?.addEventListener("click", () => changeLanguage("es"));
  document
    .getElementById("btn-en")
    ?.addEventListener("click", () => changeLanguage("en"));

  layoutSelect?.addEventListener("change", handleLayoutChange);
});
