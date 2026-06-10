use tauri::async_runtime;
use tauri::{App, AppHandle, Manager};
use tokio::sync::{Mutex, RwLock};

use terapia_visual_adapter::config_storage::TomlStorage;
use terapia_visual_adapter::messages::{self, init_language};
use terapia_visual_adapter::notifier::TauriSystemNotifier;
use terapia_visual_adapter::overlay::TauriOverlay;
use terapia_visual_domain::domain::{AppSettings, TherapyConfig};
use terapia_visual_domain::ports::ConfigStorage;

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

    // Inyectar estado a Tauri
    let state = AppState {
        therapy_storage,
        app_storage,
        overlay: Mutex::new(overlay),
        notifier,
        current_config: RwLock::new(initial_config),
    };
    app.manage(state);

    Ok(())
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
