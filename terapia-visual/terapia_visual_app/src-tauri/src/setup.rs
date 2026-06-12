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
use terapia_visual_domain::ports::OverlayPort;
use terapia_visual_domain::ports::{ConfigStorage, SystemNotifier};

use crate::state::AppState;
use crate::tray::create_tray;

pub fn init(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
    // Configurar directorios
    let config_dir = app
        .path()
        .app_data_dir()
        .expect("Failed to obtain app data dir");
    std::fs::create_dir_all(&config_dir)?;

    // Inicializar adaptadores
    let therapy_storage = TomlStorage::new(&config_dir, "therapy_config.toml");
    let app_storage = TomlStorage::new(&config_dir, "app_config.toml");
    let notifier = TauriSystemNotifier::new(app.handle().clone());
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

/// Funcion para reaccionar a los atajos de teclado
pub fn global_shortcut_handler(app: &AppHandle, shortcut: &Shortcut, event: ShortcutEvent) {
    if event.state() == ShortcutState::Pressed {
        let therapy_shortcut = Shortcut::from_str("Ctrl+Shift+T").unwrap();

        if shortcut == &therapy_shortcut {
            let app_handle = app.clone();

            // Lanzar tarea asincrona para no bloquear el teclado
            tauri::async_runtime::spawn(async move {
                let state = app_handle.state::<AppState>();

                if state.is_toggling.swap(true, Ordering::SeqCst) {
                    tracing::warn!("El usuario presiono el atajao demasiado rapido, ignorando.");
                    return;
                }

                let mut overlay = state.overlay.lock().await;

                // Si esta activa, se detiene
                if overlay.is_active() {
                    if let Ok(_) =
                        terapia_visual_domain::use_cases::stop_therapy(&mut *overlay).await
                    {
                        let _ = state.notifier.set_tray_state(false).await;
                    }
                }
                // Si esta detenida, se inicia
                else {
                    // Calcular tamaño de pantlla
                    if let Ok(Some(monitor)) = app_handle.primary_monitor() {
                        let size = monitor.size();
                        let config = state.current_config.read().await;

                        if let Ok(_) = terapia_visual_domain::use_cases::start_therapy(
                            &mut *overlay,
                            &*&config,
                            size.width,
                            size.height,
                        )
                        .await
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

/// Funcion auxiliar para guardar configuracion
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
