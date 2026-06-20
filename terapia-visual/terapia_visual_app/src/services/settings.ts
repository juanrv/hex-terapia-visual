// src/services/settings.ts
import { invoke } from "@tauri-apps/api/core";
import { CMD_GET_APP_SETTINGS, CMD_UPDATE_APP_SETTINGS } from "../commands";
import { type Language } from "../localization/i18n";

// Definir el tipo de respuesta de AppSettings
interface AppSettingsResponse {
  language: string;
}

export async function getAppSettings(): Promise<AppSettingsResponse> {
  return await invoke(CMD_GET_APP_SETTINGS);
}

export async function updateAppSettings(language: Language) {
  return await invoke(CMD_UPDATE_APP_SETTINGS, {
    newSettings: { language },
  });
}
