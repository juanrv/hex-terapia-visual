import { translate } from "../../core/localization/i18n";

let statusTimeout: number | undefined;

export function showStatus(
  messageKey: keyof typeof import("../../core/localization/i18n").translations.es,
  statusDiv: HTMLElement | null,
) {
  if (!statusDiv) return;

  statusDiv.classList.remove("status-error");
  statusDiv.innerText = translate(messageKey);

  clearTimeout(statusTimeout);
  statusTimeout = window.setTimeout(() => {
    statusDiv.innerText = "";
  }, 3000);
}

export function showError(errorMsg: string, statusDiv: HTMLElement | null) {
  if (!statusDiv) return;

  statusDiv.classList.add("status-error");
  statusDiv.innerText = `${translate("error_generic")}: ${errorMsg}`;

  clearTimeout(statusTimeout);
  statusTimeout = window.setTimeout(() => {
    statusDiv.innerText = "";
    statusDiv.classList.remove("status-error");
  }, 3000);
}
