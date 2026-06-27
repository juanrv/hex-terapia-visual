export interface ZoneConfig {
  color: string;
  opacity: number;
}

export interface OverlayConfig {
  layout: string;
  zones_config: ZoneConfig[];
}

export interface ReadingSettings {
  font_size: number;
  text_color: string;
  bg_color: string;
  line_height: string;
}

export interface ReadingConfig {
  layout: string;
  zones_config: ZoneConfig[];
  reading_settings: ReadingSettings;
}

export interface AppSettings {
  language: string;
}

export interface TherapyModule {
  mount: (container: HTMLElement) => void | Promise<void>;
  unmount: () => void | Promise<void>;
}
