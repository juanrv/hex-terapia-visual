# Stack Tecnológico - Terapia Visual

Este documento recoge las tecnologías, bibliotecas y decisiones arquitectónicas para el desarrollo de la aplicación. El objetivo es tener una referencia única que justifique cada elección y sirva como guía durante la implementación.

## 1. Lenguajes

| Capa                                              | Lenguaje                  | Justificación                                                                                                             |
| ------------------------------------------------- | ------------------------- | ------------------------------------------------------------------------------------------------------------------------- |
| Backend (lógica de sistema, dominio, adaptadores) | **Rust** (edición 2024)   | Seguridad de memoria, rendimiento, multiplataforma. Permite implementar la arquitectura hexagonal con traits y genéricos. |
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
├── terapia_visual_domain/
├── terapia_visual_adapter/
└── terapia_visual_app/
```

- **terapia_visual_domain**: Entidades, value objects, agregados, puertos (traits), casos de uso. Sin dependencias externas.
- **terapia_visual_adapter**: Implementaciones de puertos usando Tauri, `toml`, `windows` (legado), y APIs del sistema.
- **terapia_visual_app**: Configuración de Tauri, estado global, comandos que llaman a los casos de uso.

Esta separación garantiza que se pueda cambiar la implementación de la persistencia (ej. de TOML a SQLite) o del overlay (ej. de WinAPI a `winit`) sin tocar el dominio.

## 4. Bibliotecas por crate

### terapia_visual_domain/Cargo.toml

| Crate         | Versión                          | Uso                                                                | Justificación                                                                    |
| ------------- | -------------------------------- | ------------------------------------------------------------------ | -------------------------------------------------------------------------------- |
| `serde`       | 1.0.228                          | Serialización/deserialización de entidades                         | Necesario para guardar configuración en TOML.                                    |
| `thiserror`   | 2.0.18                           | Definición de errores personalizados en dominio y puertos          | Simplifica la creación de tipos de error con mensajes claros.                    |
| `async-trait` | 0.1.89                           | Permitir traits asíncronos en los puertos                          | Los métodos de los puertos son async por las implementaciones con Tauri y tokio. |
| `tokio`       | 1.51.1 (features `rt`, `macros`) | Soporte para `#[tokio::test]` en pruebas unitarias de casos de uso | No se usa en producción en el dominio, solo para pruebas.                        |

### terapia_visual_adapter/Cargo.toml

| Crate                          | Versión                   | Uso                                                                            | Justificación                                                                   |
| ------------------------------ | ------------------------- | ------------------------------------------------------------------------------ | ------------------------------------------------------------------------------- |
| `terapia_visual_domain`        | local                     | Importar entidades, puertos y tipos de error                                   | El adaptador implementa los puertos definidos en el dominio.                    |
| `tauri`                        | 2.0 (feature `tray-icon`) | Crear ventanas overlay, manejar eventos de sistema, acceder a la bandeja       | Framework principal; la app Tauri se construye sobre él.                        |
| `tauri-plugin-notification`    | 2.0                       | Mostrar notificaciones del sistema (toast en Windows, notify en Linux)         | Para `show_message` en `SystemNotifier`.                                        |
| `tauri-plugin-opener`          | 2.0                       | Abrir URLs o archivos con la aplicación por defecto (opcional)                 |                                                                                 |
| `tauri-plugin-shell`           | 2.3.5                     | Ejecutar comandos del sistema                                                  |                                                                                 |
| `tauri-plugin-global-shortcut` | 2.3.1                     | Registrar atajos de teclado globales                                           | Para detener/iniciar terapia con tecla de acceso rápido.                        |
| `tokio`                        | 1.51.1 (feature `full`)   | Runtime asíncrono, tareas de fondo, `spawn_blocking` para operaciones de disco | El adaptador es async; se necesita tokio.                                       |
| `toml`                         | 1.1.2                     | Serialización/deserialización de `config.toml`                                 | Implementación de `TomlConfigStorage`.                                          |
| `tracing`                      | 0.1.44                    | Logging estructurado (info, warn, error)                                       | Mejor que `println!` para depuración y seguimiento.                             |
| `anyhow `                      | 1.0.102                   | Manejo de errores en tests y código interno (opcional)                         | Simplifica la propagación de errores en el adaptador (no se expone al dominio). |
| `tempfile`                     | 3.27.0 (dev-dependency)   | Crear directorios temporales para pruebas                                      | Usado en tests de `TomlConfigStorage`.                                          |
| `url`                         | 2.5                       | Parsear `local://blank` para la ventana overlay                          | Necesario para `WebviewUrl::CustomProtocol` sin depender de archivos externos. |

### terapia_visual_app/Cargo.toml

| Crate                          | Version                   | Uso                                                    | Justificación                                            |
| ------------------------------ | ------------------------- | ------------------------------------------------------ | -------------------------------------------------------- |
| `terapia_visual_domain`        | local                     | Importar tipos y casos de uso                          | Necesario para llamar a los casos de uso desde comandos. |
| `terapia_visual_adapter`       | local                     | Usar los adaptadores concretos                         | Inyección de dependencias en el estado de Tauri.         |
| tauri                          | 2.0 (feature `tray-icon`) | Configurar la app, ventanas, menús, sistema de plugins | La aplicación es una app Tauri.                          |
| `tauri-plugin-opener`          | 2.0                       | Abrir enlaces (por si se necesita en el frontend)      | Funcionalidad básica.                                    |
| `tauri-plugin-shell`           |                           | 2.3.5                                                  | Ejecutar comandos externos                               |
| `tauri-plugin-global-shortcut` | 2.3.1                     | Atajos globales                                        | Llamar a `stop_therapy ` con una tecla rápida.           |
| `tauri-plugin-notification`    | 2.0.0                     | Notificaciones del sistema                             | Delegado al adaptador, pero se registra el plugin aquí.  |
| `serde`                        | 1.0 (feature `derive`)    | Serialización necesaria para comandos Tauri            | Los comandos pueden enviar datos JSON al frontend.       |
| `serde_json`                   | 1.0                       | Manipulación de JSON en comandos                       | Típico en Tauri.                                         |

## 5. Adaptadores implementados

| Puerto           | Adaptador             | Tegnología                                                                                                                                                   |
| ---------------- | --------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------------------ |
| `ConfigStorage<TherapyConfig>`  | TomlTherapyConfigStorage     | `toml` + `tokio::fs` (operaciones bloqueantes en `spawn_blocking`)                                                                                           |
| `ConfigStorage<AppSettings>`  | TomlAppConfigStorage     | `toml` + `tokio::fs` (operaciones bloqueantes en `spawn_blocking`)                                                                                           |
| `OverlayPort`    | `TauriOverlay`        | Ventana transparente con `WebviewWindowBuilder`, click-through mediante `set_ignore_cursor_events(true)`, contenido HTML dinámico (CSS para zonas de color). |
| `SystemNotifier` | `TauriSystemNotifier` | Notificaciones del sistema con `tauri-plugin-notification`; tooltip de bandeja con `tray_by_id("main")`.                                                     |

**Nota**: _Existe una implementación alternativa WindowsOverlay (WinAPI) que se dejó como legado pero no se compila actualmente_.

## 6. Persistencia de configuración (RNF-06, RF-09)

- **Formato**: TOML (archivo `config.toml`).
- **Ubicación**: Directorio de datos de la aplicación (`%APPDATA%\com.shiro.terapia-visual-app\data\`). En Windows, portable si se usa `app_data_dir()`.
- **Contenido mínimo**:

  ```toml
  therapy_type = "ColorDivision"
  layout = "Vertical"

  [[zones_config]]
  color = "#FF0000"
  opacity = 0.8

  [[zones_config]]
  color = "#00FF00"
  opacity = 0.6
  ```

- **Momento de guardado**: Solo al cerrar la aplicación (RF-09). No se guarda automáticamente en cada cambio.

## 7. Concurrencia y asincronía

- **Runtime**: `tokio` (multihilo). Tauri ya lo usa internamente.

- **Operaciones bloqueantes** (lectura/escritura de archivos) se lanzan con `tokio::task::spawn_blocking`.

- **Overlay**: La ventana se crea y maneja en el hilo principal de Tauri; el contenido HTML se actualiza mediante `eval`.

- **Notificaciones**: `tauri-plugin-notification` maneja la asincronía internamente.

## 8. Testeo

| Tipo                  | Herramienta                           | Ubicacion                                                 |
| --------------------- | ------------------------------------- | --------------------------------------------------------- |
| Unitarias del Dominio | `#[test]`                             | Dentro de cada módulo en `terapia_visual_domain`          |
| Casos de uso          | `#[tokio::test]` + mocks              | Dentro de `use_cases/*rs` (mocks en `use_cases/mocks.rs`) |
| Adaptadores           | Pruebas de integración con `tempfile` | En `terapia_visual_adapter/src/config_storage/mod.rs`     |
| End-to-End            | `tauri::test`                         | `terapia_visual_app/tests/`                               |

## 9. Logging y Monitoreo

- **Crate**: `tracing` (con `tracing-suscriber` para formateo de consola)
- **Niveles**: error, warn, info, debug, trace
- **Salida**: Por defecto a consola (`stderr`) y opcionalmente a archivo (`tracing-appender`)
- **Eventos Clave**:
- Inicio y parada de la app
- Cambio de configuración
- Fallos en creación de overlay
- Hotkeys presionadas

## 10. Distribución y portabilidad

- **Windows**: Binario único `.exe` autónomo o instaladores MSI/NSIS generados con `cargo tauri build` Se asume que WebView2 está presente (Windows 10/11 lo incluye).
- **Linux**: Se genera un `.AppImage` que incluya los recursos necesarios, soporte para X11, Wayland puede tener limitaciones por el click-through.
- **Configuración portable**: Archivo `config.toml` junto al ejecutable.

## 11. Justificación de Decisiones Clave

|                       Decisión                       |                Alternativas descartadas                 |                                                  Razón                                                  |
| :--------------------------------------------------: | :-----------------------------------------------------: | :-----------------------------------------------------------------------------------------------------: |
|                        Tauri                         | Electron (pesado), `WinApi` + UI manual (mucho trabajo) |                        Balance entre productividad, rendimiento y portabilidad.                         |
|                Workspace de 3 crates                 |                  Todo en un solo crate                  |                          Aislamiento del dominio para tests y futuros cambios.                          |
|               TOML para configuración                |                      JSON, SQLite                       |                       Legibilidad, edición manual fácil, sin dependencias extra.                        |
|                      `tracing`                       |                   `env_logger`, `log`                   |                Logging estructurado y más potente, necesario para depurar concurrencia.                 |
|             Overlay con `windows` crate              | `winapi` (inseguro), `winit` (no soporta click-through) | `windows` es más seguro y mantenible que `winapi`, y da control total sobre las extensiones de ventana. |
| Atajos de teclado con `tauri-plugin-global-shortcut` |              `global-hotkey` crate externo              |      Plugin oficial de Tauri 2, mejor integración, funciona en Windows (y Linux con limitaciones).      |

```

```
