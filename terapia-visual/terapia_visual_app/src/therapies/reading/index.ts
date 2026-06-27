import type { TherapyModule } from "../../core/types";
import { renderReadingView } from "./view";
import { startReading, stopReading } from "../../core/services/reading";
import { showStatus, showError } from "../../ui/components/status";
import { goHome } from "../../ui/router";

let cleanupFunctions: (() => void)[] = [];

function processReadingText(rawText: string): string {
  if (rawText.includes("<p>") || rawText.includes("<div>")) {
    const parser = new DOMParser();
    const doc = parser.parseFromString(rawText, "text/html");
    doc
      .querySelectorAll("script, style, link, meta, head")
      .forEach((el) => el.remove());
    doc.querySelectorAll("*").forEach((el) => {
      el.removeAttribute("class");
      el.removeAttribute("style");
      el.removeAttribute("id");
    });
    return doc.body.innerHTML;
  }

  const escaped = rawText
    .replace(/&/g, "&amp;")
    .replace(/</g, "&lt;")
    .replace(/>/g, "&gt;");
  return escaped
    .split(/\n\n+/)
    .filter((p) => p.trim() !== "")
    .map((p) => `<p>${p.replace(/\n/g, "<br>")}</p>`)
    .join("\n");
}

export const readingTherapy: TherapyModule = {
  mount: async (container: HTMLElement) => {
    renderReadingView(container);

    const statusDiv = document.getElementById("status");
    const startBtn = document.getElementById("btn-start-reading");
    const stopBtn = document.getElementById("btn-stop-reading");
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

    backBtn?.addEventListener("click", goHome);
    startBtn?.addEventListener("click", startHandler);
    stopBtn?.addEventListener("click", stopHandler);

    cleanupFunctions = [
      () => backBtn?.removeEventListener("click", goHome),
      () => startBtn?.removeEventListener("click", startHandler),
      () => stopBtn?.removeEventListener("click", stopHandler),
    ];
  },

  unmount: async () => {
    cleanupFunctions.forEach((fn) => fn());
    cleanupFunctions = [];
  },
};
