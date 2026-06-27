import { invoke } from "@tauri-apps/api/core";
import {
  CMD_GET_READING_CONFIG,
  CMD_START_READING_THERAPY,
  CMD_STOP_READING_THERAPY,
  CMD_UPDATE_READING_CONFIG,
  CMD_CHANGE_READING_LAYOUT,
  CMD_UPDATE_READING_ZONE_COLOR,
  CMD_UPDATE_READING_ZONE_OPACITY,
  CMD_UPDATE_READING_SETTINGS,
  CMD_RESET_READING_CONFIG,
  CMD_READING_WINDOW_RESIZED,
} from "../commands/reading";
import type { ReadingConfig, ReadingSettings } from "../types";

export async function getReadingConfig(): Promise<ReadingConfig> {
  return await invoke(CMD_GET_READING_CONFIG);
}

export async function startReading(htmlContent: string) {
  return await invoke(CMD_START_READING_THERAPY, { htmlContent });
}

export async function stopReading() {
  return await invoke(CMD_STOP_READING_THERAPY);
}

export async function updateReadingConfig(newConfig: ReadingConfig) {
  return await invoke(CMD_UPDATE_READING_CONFIG, { newConfig });
}

export async function changeReadingLayout(newLayout: string) {
  return await invoke(CMD_CHANGE_READING_LAYOUT, { newLayout });
}

export async function updateReadingZoneColor(
  zoneIndex: number,
  newColor: string,
) {
  return await invoke(CMD_UPDATE_READING_ZONE_COLOR, { zoneIndex, newColor });
}

export async function updateReadingZoneOpacity(
  zoneIndex: number,
  newOpacity: number,
) {
  return await invoke(CMD_UPDATE_READING_ZONE_OPACITY, {
    zoneIndex,
    newOpacity,
  });
}

export async function updateReadingSettings(newSettings: ReadingSettings) {
  return await invoke(CMD_UPDATE_READING_SETTINGS, { newSettings });
}

export async function resetReadingConfig(): Promise<ReadingConfig> {
  return await invoke(CMD_RESET_READING_CONFIG);
}

export async function notifyReadingWindowResized() {
  return await invoke(CMD_READING_WINDOW_RESIZED);
}
