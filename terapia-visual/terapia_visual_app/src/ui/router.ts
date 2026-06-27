import type { TherapyModule } from "../core/types";
import { overlayTherapy } from "../therapies/overlay";
import { readingTherapy } from "../therapies/reading";
import { applyTranslations } from "../core/localization/i18n";

const therapyMap: Record<string, TherapyModule> = {
  overlay: overlayTherapy,
  reading: readingTherapy,
};

let currentTherapy: TherapyModule | null = null;

export async function switchTherapy(therapyId: "overlay" | "reading") {
  const container = document.getElementById("app-content");
  const homeView = document.getElementById("view-home");
  if (!container) return;

  // Limpiar módulo anterior
  if (currentTherapy) {
    await currentTherapy.unmount();
  }

  // Ocultar menu principal
  if (homeView) homeView.style.display = "none";

  // Limpiar DOM
  container.innerHTML = "";

  // Cargar nuevo modulo
  const module = therapyMap[therapyId];
  if (!module) return;

  currentTherapy = module;
  await module.mount(container);

  applyTranslations();
}

export function goHome() {
  const container = document.getElementById("app-content");
  const homeView = document.getElementById("view-home");

  // Limpiar módulo actual
  if (currentTherapy) {
    currentTherapy.unmount();
    currentTherapy = null;
  }

  // Mostrar el menú principal
  if (homeView) homeView.style.display = "block";

  // Limpiar el contenedor de contenido
  if (container) container.innerHTML = "";
}
