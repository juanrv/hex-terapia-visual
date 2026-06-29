import type { TherapyModule } from "../../core/types";
import { renderReadingView } from "./view";
import {
  resetReadingConfig,
  startReading,
  stopReading,
} from "../../core/services/reading";
import { showStatus, showError } from "../../ui/components/status";
import { goHome } from "../../ui/router";
import { processReadingText } from "../../core/utils/text";

let cleanupFunctions: (() => void)[] = [];

export const readingTherapy: TherapyModule = {
  mount: async (container: HTMLElement) => {
    renderReadingView(container);

    const statusDiv = document.getElementById("status");
    const startBtn = document.getElementById("btn-start-reading");
    const stopBtn = document.getElementById("btn-stop-reading");
    const resetBtn = document.getElementById("btn-reset-reading");
    const backBtn = document.getElementById("btn-back-reading");

    const startHandler = async () => {
      const textarea = document.getElementById(
        "reading-input",
      ) as HTMLTextAreaElement;
      if (!textarea || !textarea.value.trim()) {
        showError("Por favor, pega algún texto o HTML primero.", statusDiv);
        return;
      }
      try {
        const cleanHtml = processReadingText(textarea.value);
        await startReading(cleanHtml);
        showStatus("status_reading_started", statusDiv);
      } catch (err) {
        showError(String(err), statusDiv);
      }
    };

    const stopHandler = () => {
      stopReading()
        .then(() => showStatus("status_reading_stopped", statusDiv))
        .catch((err) => showError(String(err), statusDiv));
    };

    const resetHandler = async () => {
      try {
        await resetReadingConfig();
        showStatus("status_reset", statusDiv);
      } catch (err) {
        showError(String(err), statusDiv);
      }
    };

    backBtn?.addEventListener("click", goHome);
    startBtn?.addEventListener("click", startHandler);
    stopBtn?.addEventListener("click", stopHandler);
    resetBtn?.addEventListener("click", resetHandler);

    cleanupFunctions = [
      () => backBtn?.removeEventListener("click", goHome),
      () => startBtn?.removeEventListener("click", startHandler),
      () => stopBtn?.removeEventListener("click", stopHandler),
      () => resetBtn?.removeEventListener("click", resetHandler),
    ];
  },

  unmount: async () => {
    cleanupFunctions.forEach((fn) => fn());
    cleanupFunctions = [];
  },
};
