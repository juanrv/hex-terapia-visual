// src/services/overlay.ts
import { invoke } from "@tauri-apps/api/core";
import {
  CMD_GET_OVERLAY_CONFIG,
  CMD_START_OVERLAY,
  CMD_STOP_OVERLAY,
  CMD_UPDATE_OVERLAY_CONFIG,
  CMD_CHANGE_OVERLAY_LAYOUT,
  CMD_UPDATE_OVERLAY_ZONE_COLOR,
  CMD_UPDATE_OVERLAY_ZONE_OPACITY,
  CMD_RESET_OVERLAY_CONFIG,
} from "../commands";

export async function getOverlayConfig() {
  return await invoke(CMD_GET_OVERLAY_CONFIG);
}

export async function startOverlay(screenWidth: number, screenHeight: number) {
  return await invoke(CMD_START_OVERLAY, { screenWidth, screenHeight });
}

export async function stopOverlay() {
  return await invoke(CMD_STOP_OVERLAY);
}

export async function updateOverlayConfig(
  newConfig: any,
  screenWidth: number,
  screenHeight: number,
) {
  return await invoke(CMD_UPDATE_OVERLAY_CONFIG, {
    newConfig,
    screenWidth,
    screenHeight,
  });
}

export async function changeOverlayLayout(
  newLayout: string,
  screenWidth: number,
  screenHeight: number,
) {
  return await invoke(CMD_CHANGE_OVERLAY_LAYOUT, {
    newLayout,
    screenWidth,
    screenHeight,
  });
}

export async function updateOverlayZoneColor(
  zoneIndex: number,
  newColor: string,
  screenWidth: number,
  screenHeight: number,
) {
  return await invoke(CMD_UPDATE_OVERLAY_ZONE_COLOR, {
    zoneIndex,
    newColor,
    screenWidth,
    screenHeight,
  });
}

export async function updateOverlayZoneOpacity(
  zoneIndex: number,
  newOpacity: number,
  screenWidth: number,
  screenHeight: number,
) {
  return await invoke(CMD_UPDATE_OVERLAY_ZONE_OPACITY, {
    zoneIndex,
    newOpacity,
    screenWidth,
    screenHeight,
  });
}

export async function resetOverlayConfig(
  screenWidth: number,
  screenHeight: number,
) {
  return await invoke(CMD_RESET_OVERLAY_CONFIG, {
    screenWidth,
    screenHeight,
  });
}
