//! # Inicialización de la Aplicación
//!
//! Este módulo contiene toda la lógica de inicialización de la aplicación Tauri.
//! Configura los adaptadores, carga la configuración, crea la bandeja del sistema,
//! registra los atajos de teclado y prepara el estado global.
//!
//! # Flujo de inicialización
//!
//! 1. Obtener el directorio de datos de la aplicación.
//! 2. Crear los adaptadores (almacenamiento, overlay, notificador).
//! 3. Cargar la configuración de terapia y aplicación.
//! 4. Inicializar el sistema de mensajes con el idioma guardado.
//! 5. Configurar el título de la ventana principal.
//! 6. Crear la bandeja del sistema.
//! 7. Registrar el atajo de teclado global (Ctrl+Shift+T).
//! 8. Crear el estado global e inyectarlo en Tauri.
//!
//! # Ejemplo de uso
//!
//! ```no_run
//! use tauri::App;
//! use crate::setup::init;
//!
//! # fn example(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
//! init(app)?;
//! # Ok(())
//! # }
//! ```

use std::str::FromStr;
use std::sync::atomic::{AtomicBool, Ordering};

use tauri::async_runtime;
use tauri::{App, AppHandle, Manager};
use tauri_plugin_global_shortcut::{GlobalShortcutExt, Shortcut, ShortcutEvent, ShortcutState};
use tokio::sync::{Mutex, RwLock};

use terapia_visual_adapter::config_storage::TomlStorage;
use terapia_visual_adapter::messages::{self, init_language};
use terapia_visual_adapter::notifier::TauriSystemNotifier;
use terapia_visual_adapter::overlay::TauriOverlay;
use terapia_visual_domain::domain::{AppSettings, TherapyConfig};
use terapia_visual_domain::ports::{ConfigStorage, SystemNotifier};

use crate::state::AppState;
use crate::tray::create_tray;

/// Inicializa la aplicación Tauri.
///
/// Esta función se llama desde `lib.rs` en el `setup` del `tauri::Builder`.
/// Configura todos los componentes necesarios para que la aplicación funcione.
///
/// # Argumentos
///
/// * `app` - La aplicación Tauri en construcción.
///
/// # Errores
///
/// Devuelve un error si falla la creación de directorios, la carga de configuración
/// o la creación de la bandeja.
///
/// # Ejemplo
///
/// ```no_run
/// use tauri::App;
/// use crate::setup::init;
///
/// # fn example(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
/// init(app)?;
/// # Ok(())
/// # }
/// ```
pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // Obtener el directorio de datos de la aplicación
    // En Windows: %APPDATA%\com.terapia-visual-app\data
    // En Linux: ~/.config/com.terapia-visual-app/data
    let config_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to obtain app data dir");
    std::fs::create_dir_all(&config_dir)?;

    // Cargar iconos de la bandeja empaquetados en el binario
    const ICON_ACTIVE: &[u8] = include_bytes!("../icons/tray_active.png");
    const ICON_INACTIVE: &[u8] = include_bytes!("../icons/tray_inactive.png");

    // Inicializar adaptadores
    let therapy_storage = TomlStorage::new(&config_dir, "therapy_config.toml");
    let app_storage = TomlStorage::new(&config_dir, "app_config.toml");
    let notifier = TauriSystemNotifier::new(app.handle().clone(), ICON_INACTIVE, ICON_ACTIVE);
    let overlay = TauriOverlay::new(app.handle().clone());

    // Cargar configuraciones (con valores por defecto en caso de fallo)
    let initial_config: TherapyConfig = tauri::async_runtime::block_on(therapy_storage.load())
        .unwrap_or_else(|_| TherapyConfig::default());

    // Guardar la inicial por si es la primera vez
    let _ = tauri::async_runtime::block_on(therapy_storage.save(&initial_config));

    let app_settings: AppSettings =
        tauri::async_runtime::block_on(app_storage.load()).unwrap_or_default();

    // Aplicar configuracion visual y traducciones
    init_language(&app_settings);
    if let Some(main_window) = app.get_webview_window("main") {
        let _ = main_window.set_title(messages::window_title());
    }

    // Crear bandeja del sistema
    create_tray(app)?;

    // Registrar atajo de teclado
    let therapy_shortcut = Shortcut::from_str("Ctrl+Shift+T")?;
    app.handle().global_shortcut().register(therapy_shortcut)?;

    // Inyectar estado a Tauri
    let state = AppState {
        therapy_storage,
        app_storage,
        overlay: Mutex::new(overlay),
        notifier,
        current_config: RwLock::new(initial_config),
        is_toggling: AtomicBool::new(false),
    };
    app.manage(state);

    Ok(())
}

/// Manejador del atajo de teclado global (Ctrl+Shift+T).
///
/// Esta función se ejecuta cuando el usuario presiona el atajo de teclado.
/// Alterna el estado de la terapia (inicia si está detenida, detiene si está activa).
///
/// # Comportamiento
///
/// 1. Si la terapia está activa, la detiene.
/// 2. Si la terapia está inactiva, la inicia usando la configuración actual.
/// 3. Utiliza un `AtomicBool` (`is_toggling`) para evitar múltiples activaciones rápidas.
///
/// # Argumentos
///
/// * `app` - Handle de la aplicación Tauri.
/// * `shortcut` - El atajo que se presionó.
/// * `event` - El evento del atajo (presionado o liberado).
///
/// # Ejemplo
///
/// ```no_run
/// use tauri_plugin_global_shortcut::{Shortcut, ShortcutEvent};
/// use crate::setup::global_shortcut_handler;
/// use tauri::AppHandle;
///
/// # fn example(app: &AppHandle, shortcut: &Shortcut, event: ShortcutEvent) {
/// global_shortcut_handler(app, shortcut, event);
/// # }
/// ```
pub fn global_shortcut_handler(app: &AppHandle, shortcut: &Shortcut, event: ShortcutEvent) {
    if event.state() == ShortcutState::Pressed {
        let therapy_shortcut = Shortcut::from_str("Ctrl+Shift+T").unwrap();

        if shortcut == &therapy_shortcut {
            let app_handle = app.clone();

            // Lanzar tarea asincrona para no bloquear el teclado
            tauri::async_runtime::spawn(async move {
                let state = app_handle.state::<AppState>();

                if state.is_toggling.swap(true, Ordering::SeqCst) {
                    tracing::warn!("El usuario presiono el atajo demasiado rapido, ignorando.");
                    return;
                }

                let mut overlay = state.overlay.lock().await;

                // Si esta activa, se detiene
                if terapia_visual_domain::use_cases::stop_therapy(&mut *overlay)
                    .await
                    .is_ok()
                {
                    let _ = state.notifier.set_tray_state(false).await;
                }
                // Si esta detenida, se inicia
                else {
                    // Calcular tamaño de pantlla
                    if let Ok(Some(monitor)) = app_handle.primary_monitor() {
                        let size = monitor.size();
                        let config = state.current_config.read().await;

                        if terapia_visual_domain::use_cases::start_therapy(
                            &mut *overlay,
                            &config,
                            size.width,
                            size.height,
                        )
                        .await
                        .is_ok()
                        {
                            let _ = state.notifier.set_tray_state(true).await;
                        }
                    }
                }

                // Cambiar la flag
                state.is_toggling.store(false, Ordering::SeqCst);
            });
        }
    }
}

/// Guarda la configuración actual en el almacenamiento.
///
/// Esta función se llama:
/// - Al ocultar la ventana principal (minimizar a bandeja).
/// - Al salir de la aplicación desde el menú de la bandeja.
///
/// # Argumentos
///
/// * `app_handle` - Handle de la aplicación Tauri.
///
/// # Ejemplo
///
/// ```no_run
/// use crate::setup::save_configs;
/// use tauri::AppHandle;
///
/// # fn example(app_handle: &AppHandle) {
/// save_configs(app_handle);
/// # }
/// ```
pub fn save_configs(app_handle: &AppHandle) {
    let state = app_handle.state::<AppState>();

    let therapy_config = async_runtime::block_on(state.current_config.read());
    println!("[DEBUG] Saving therapy config: {:?}", therapy_config);
    if let Err(e) = async_runtime::block_on(state.therapy_storage.save(&*therapy_config)) {
        eprintln!("Error saving therapy config: {}", e);
    }
    // Guardar app_settings también
    let app_settings: AppSettings =
        tauri::async_runtime::block_on(state.app_storage.load()).unwrap_or_default();
    println!("[DEBUG] Saving app settings: {:?}", app_settings);
    if let Err(e) = async_runtime::block_on(state.app_storage.save(&app_settings)) {
        eprintln!("Error saving app config: {}", e);
    }
}
