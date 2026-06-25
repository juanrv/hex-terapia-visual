import { invoke } from "@tauri-apps/api/core";
import {
  CMD_START_READING_THERAPY,
  CMD_STOP_READING_THERAPY,
} from "../commands/reading";

export async function startReading(htmlContent: string) {
  return await invoke(CMD_START_READING_THERAPY, { htmlContent });
}

export async function stopReading() {
  return await invoke(CMD_STOP_READING_THERAPY);
}
