import type { TherapyModule } from "../../core/types";
import { renderOverlayView } from "./view";
import {
  getOverlayConfig,
  startOverlay,
  stopOverlay,
  resetOverlayConfig,
  changeOverlayLayout,
  updateOverlayZoneColor,
  updateOverlayZoneOpacity,
} from "../../core/services/overlay";
import { renderZoneControls } from "../../ui/components/zone-controls";
import { showStatus, showError } from "../../ui/components/status";
import { goHome } from "../../ui/router";

let cleanupFunctions: (() => void)[] = [];

export const overlayTherapy: TherapyModule = {
  mount: async (container: HTMLElement) => {
    // Renderizar HTML
    renderOverlayView(container);

    const statusDiv = document.getElementById("status");
    const layoutSelect = document.getElementById(
      "layout-select",
    ) as HTMLSelectElement;

    // Cargar configuracion y renderizar controles
    async function loadConfig() {
      try {
        const config = await getOverlayConfig();
        renderZoneControls(config, (index, newColor, newOpacity) => {
          updateZoneConfig(index, newColor, newOpacity);
        });
        if (layoutSelect) layoutSelect.value = config.layout;
      } catch (error) {
        console.error("Error loading overlay config:", error);
        showError(String(error), statusDiv);
      }
    }

    await loadConfig();

    // Handlers de eventos
    const startHandler = () => {
      startOverlay(window.screen.width, window.screen.height)
        .then(() => showStatus("status_started", statusDiv))
        .catch((err) => showError(String(err), statusDiv));
    };

    const stopHandler = () => {
      stopOverlay()
        .then(() => showStatus("status_stopped", statusDiv))
        .catch((err) => showError(String(err), statusDiv));
    };

    const resetHandler = async () => {
      try {
        await resetOverlayConfig(window.screen.width, window.screen.height);
        await loadConfig();
        showStatus("status_reset", statusDiv);
      } catch (err) {
        showError(String(err), statusDiv);
      }
    };

    const layoutHandler = async (e: Event) => {
      const newLayout = (e.target as HTMLSelectElement).value;
      try {
        await changeOverlayLayout(
          newLayout,
          window.screen.width,
          window.screen.height,
        );
        await loadConfig();
        showStatus("status_updated", statusDiv);
      } catch (err) {
        showError(String(err), statusDiv);
      }
    };

    const startBtn = document.getElementById("btn-start");
    const stopBtn = document.getElementById("btn-stop");
    const resetBtn = document.getElementById("btn-reset");
    const backBtn = document.getElementById("btn-back-overlay");

    startBtn?.addEventListener("click", startHandler);
    stopBtn?.addEventListener("click", stopHandler);
    resetBtn?.addEventListener("click", resetHandler);
    layoutSelect?.addEventListener("change", layoutHandler);
    backBtn?.addEventListener("click", goHome);

    // Guardar funciones de limpieza
    cleanupFunctions = [
      () => startBtn?.removeEventListener("click", startHandler),
      () => stopBtn?.removeEventListener("click", stopHandler),
      () => resetBtn?.removeEventListener("click", resetHandler),
      () => layoutSelect?.removeEventListener("change", layoutHandler),
      () => backBtn?.removeEventListener("click", goHome),
    ];
  },

  unmount: async () => {
    cleanupFunctions.forEach((fn) => fn());
    cleanupFunctions = [];
  },
};

// Funcion auxiliar para actualizar zonas
async function updateZoneConfig(
  index: number,
  newColor: string | null,
  newOpacity: number | null,
) {
  const statusDiv = document.getElementById("status");

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

    // Recargar configuracion para actualizar la UI
    const config = await getOverlayConfig();
    renderZoneControls(config, (i, c, o) => {
      updateZoneConfig(i, c, o);
    });
    showStatus("status_updated", statusDiv);
  } catch (err) {
    showError(String(err), statusDiv);
  }
}
