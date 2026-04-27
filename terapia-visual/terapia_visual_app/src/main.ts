import { invoke } from "@tauri-apps/api/core";

// Función para obtener configuración actual
async function getConfig() {
  try {
    const config = await invoke("cmd_get_config");
    console.log("Configuración actual:", config);
    // Aquí puedes actualizar la UI con los valores
  } catch (error) {
    console.error("Error al obtener configuración:", error);
  }
}

// Iniciar terapia
async function startTherapy() {
  try {
    // Usar resolución de pantalla real (puedes obtenerla con window.screen)
    const screenWidth = window.screen.width;
    const screenHeight = window.screen.height;
    await invoke("cmd_start_therapy", { screenWidth, screenHeight });
    console.log("Terapia iniciada");
  } catch (error) {
    console.error("Error al iniciar:", error);
  }
}

// Detener terapia
async function stopTherapy() {
  try {
    await invoke("cmd_stop_therapy");
    console.log("Terapia detenida");
  } catch (error) {
    console.error("Error al detener:", error);
  }
}

// Actualizar configuración (ejemplo con valores fijos)
async function updateConfig() {
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
    await invoke("cmd_update_config", { newConfig, screenWidth, screenHeight });
    console.log("Configuración actualizada");
  } catch (error) {
    console.error("Error al actualizar:", error);
  }
}

// Asignar eventos cuando el DOM esté listo
window.addEventListener("DOMContentLoaded", () => {
  // Suponiendo que tienes botones en index.html con estos ids
  const btnStart = document.getElementById("btn-start");
  const btnStop = document.getElementById("btn-stop");
  const btnUpdate = document.getElementById("btn-update");
  const btnGet = document.getElementById("btn-get");

  if (btnStart) btnStart.addEventListener("click", startTherapy);
  if (btnStop) btnStop.addEventListener("click", stopTherapy);
  if (btnUpdate) btnUpdate.addEventListener("click", updateConfig);
  if (btnGet) btnGet.addEventListener("click", getConfig);
});
