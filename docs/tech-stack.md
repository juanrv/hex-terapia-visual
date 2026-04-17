# Stack Tecnológico - Terapia Visual

Este documento recoge las tecnologías, bibliotecas y decisiones arquitectónicas para el desarrollo de la aplicación. El objetivo es tener una referencia única que justifique cada elección y sirva como guía durante la implementación.

## 1. Lenguajes

| Capa                                              | Lenguaje                  | Justificación                                                                                                             |
| ------------------------------------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Backend (lógica de sistema, dominio, adaptadores) | **Rust** (edición 2021)   | Seguridad de memoria, rendimiento, multiplataforma. Permite implementar la arquitectura hexagonal con traits y genéricos. |
| Frontend (interfaz de control)                    | **TypeScript** + HTML/CSS | Integración nativa con Tauri, facilidad para crear interfaces accesibles y dinámicas. TypeScript añade tipado.            |

## 2. Framework principal

**Tauri** (versión 2.0)

- **Razón principal**: Permite construir un binario portable (RNF-07) con un frontend web ligero, sin empaquetar un runtime de navegador completo.
- **Rendimiento**: Bajo consumo de CPU/RAM (RNF-03).
- **Multiplataforma**: Soporta Windows (prioritario) y Linux (secundario) con el mismo código base.
- **Nota crítica**: No impone una arquitectura hexagonal pura, pero se usará de forma que el dominio (core) quede completamente aislado. Tauri actuará solo como "cáscara" (controladores y adaptadores concretos).

## 3. Arquitectura interna (Hexagonal híbrida)

El proyecto se organiza como un **workspace de Cargo** con tres crates:

```
terapia-visual
├── core/
├── tauri-adapter/
└── tauri-app/
```

- **core**: Puede compilarse y testearse sin interfaz gráfica ni sistema operativo. Contiene las reglas de negocio (terapias, layouts, colores) y los traits de los puertos.
- **tauri-adapter**: Depende de `core` e implementa los traits con bibliotecas del sistema (ventanas, click-through, persistencia, bandeja).
- **tauri-app**: Depende de ambos. Sus comandos Tauri son cortos: solo obtienen el estado, llaman a los casos de uso del core y pasan los adaptadores.

Esta separación garantiza que se pueda cambiar la implementación de la persistencia (ej. de TOML a SQLite) o del overlay (ej. de WinAPI a `winit`) sin tocar el dominio.

## 4. Bibliotecas por crate

### core/Cargo.toml

| Crate                 | Uso                                                                              |
| --------------------- | -------------------------------------------------------------------------------- |
| `serde` (con derive)  | Serialización para entidades del dominio (necesario para guardar configuración). |
| `thiserror`           | Definición de errores del dominio y casos de uso.                                |
| `async-trait`         | Traits de puertos que requieren async (ej. `OverlayPort::show`).                 |
| `tokio` (solo traits) | Para compatibilidad con `async-trait`. El runtime lo proveerá el adaptador.      |

### tauri-adapter/Cargo.toml

| Crate                                                                                   | Uso                                                                      |
| --------------------------------------------------------------------------------------- | ------------------------------------------------------------------------ |
| `tauri` (`version = "2", features = ["tray-icon"]`)                                     | Framework principal. tray-icon es feature nativa                         |
| `tokio` (con feature `full`)                                                            | Runtime asíncrono para manejar tareas de fondo (terapia, hotkeys).       |
| `serde` + `toml`                                                                        | Lectura/escritura del archivo `config.toml`.                             |
| `tracing` + `tracing-subscriber`                                                        | Logging estructurado (sustituye a `env_logger`).                         |
| `windows` (crate `0.52`, features `Win32_UI_WindowsAndMessaging`, `Win32_Graphics_Gdi`) | Para crear ventanas con click-through (transparencia) en Windows.        |
| `tauri-plugin-global-shortcut`                                                          | Atajos de teclado globales (en Windows funciona, en Linux requiere X11). |
| `tauri-plugin-opener = "2"`                                                             | Abrir URLs/archivos con programa predeterminado (opcional).              |

### tauri-app/Cargo.toml

Depende de `core` y `tauri-adapter`. Además incluye la configuración típica de Tauri (build de frontend, etc.). No añade lógica de negocio.

## 5. Persistencia de configuración (RNF-06, RF-09)

- **Formato**: TOML (archivo `config.toml`).
- **Ubicación**: Directorio de la aplicación (para portabilidad). En Windows, junto al `.exe`.
- **Contenido mínimo**:

  ```toml
  last_therapy = "ColorDivision"
  last_layout = "Vertical"

  [zones]
  left = { color = "#FF0000", opacity = 0.7 }
  right = { color = "#0000FF", opacity = 0.7 }

  [preferences]
  floating_buttons = false
  hotkey_start_stop = "Ctrl+Alt+T"
  ```

- **Momento de guardado**: Solo al cerrar la aplicación (RF-09). No se guarda automáticamente en cada cambio.

## 6. Concurrencia y asincronía

- **Runtime**: `tokio` (multihilo). Tauri ya lo usa internamente.

- **Terapia activa**: Se ejecuta en una tarea asíncrona separada. La comunicación con el resto de la app se hace mediante canales (`tokio::sync::mpsc`).

- **Overlay**: Las operaciones de mostrar/ocultar ventanas son asíncronas (async), aunque internamente llamen a APIs síncronas de Windows (se envuelven en `spawn_blocking` si es necesario).

## 7. Testeo

|       Tipo de Test        |                      Herramienta                      |            Ubicación             |
| :-----------------------: | :---------------------------------------------------: | :------------------------------: |
|   Unitarios del Dominio   |                   `#[test]`estándar                   |         Dentro de `core          |
| Unitarios de Casos de Uso |      `#[tokio::test]` + dobles de prueba (Mocks)      |          Dentro de Core          |
|  Pruebas de Adaptadores   | Integración con Windows o Tauri (mínimas, solo smoke) | En `tauri-adapter` y `tauri-app` |
|    Pruebas end-to-end     |           `tauri::test` (lanza la app real)           |       En `tauri-app/tests`       |

## 8. Logging y Monitoreo

- **Crate**: `tracing` (con `tracing-suscriber` para formateo de consola)
- **Niveles**: error, warn, info, debug, trace
- **Salida**: Por defecto a consola (`stderr`) y opcionalmente a archivo (`tracing-appender`)
- **Eventos Clave**:
  - Inicio y parada de la app
  - Cambio de configuración
  - Fallos en creación de overlay
  - Hotkeys presionadas

## 9. Distribución y portabilidad

- **Windows**: Binario único `.exe` autónomo. No requiere instalador. Se asume que WebView2 está presente (Windows 10/11 lo incluye).
- **Linus**: Se genera un `.AppImage` que incluya los recursos necesarios.
- **Sin dependencias externas**: Toda la lógica de sistema se compila estáticamente en Rust.

## 10. Justificación de Decisiones Clave

|          Decisión           |                Alternativas descartadas                 |                                                  Razón                                                  |
| :-------------------------: | :-----------------------------------------------------: | :-----------------------------------------------------------------------------------------------------: |
|            Tauri            | Electron (pesado), `winit` + UI manual (mucho trabajo)  |                        Balance entre productividad, rendimiento y portabilidad.                         |
|    Workspace de 3 crates    |                  Todo en un solo crate                  |                          Aislamiento del dominio para tests y futuros cambios.                          |
|   TOML para configuración   |                      JSON, SQLite                       |                       Legibilidad, edición manual fácil, sin dependencias extra.                        |
|          `tracing`          |                   `env_logger`, `log`                   |                Logging estructurado y más potente, necesario para depurar concurrencia.                 |
| Overlay con `windows` crate | `winapi` (inseguro), `winit` (no soporta click-through) | `windows` es más seguro y mantenible que `winapi`, y da control total sobre las extensiones de ventana. |
|       `global-hotkey`       |                 `rdev`, `device_query`                  |                    `global-hotkey` abstrae correctamente RegisterHotKey en Windows.                     |
