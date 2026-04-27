// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::tray::TrayIconBuilder;
use tauri::{async_runtime, Manager};
use terapia_visual_domain::ports::ConfigStorage;
use tokio::sync::{Mutex, RwLock};

// Importaciones de dominio y adaptadores
use terapia_visual_adapter::config_storage::TomlConfigStorage;
use terapia_visual_adapter::notifier::TauriSystemNotifier;
use terapia_visual_adapter::overlay::TauriOverlay;
use terapia_visual_domain::domain::TherapyConfig;
use terapia_visual_domain::use_cases::{start_therapy, stop_therapy, update_config};

// Estado global de la aplicacion
pub struct AppState {
    pub config_storage: TomlConfigStorage,
    pub overlay: Mutex<TauriOverlay>,
    pub notifier: TauriSystemNotifier,
    pub current_config: RwLock<TherapyConfig>,
}

#[tauri::command]
async fn cmd_get_config(state: tauri::State<'_, AppState>) -> Result<TherapyConfig, String> {
    // Leer la configuracion actual del estado (no del disco, por eficiencia)
    let config = state.current_config.read().await;
    Ok(config.clone())
}

#[tauri::command]
async fn cmd_start_therapy(
    state: tauri::State<'_, AppState>,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    let config = state.current_config.read().await;
    let mut overlay = state.overlay.lock().await;
    start_therapy(&mut *overlay, &config, screen_width, screen_height)
        .await
        .map_err(|e| e.to_string())
}

#[tauri::command]
async fn cmd_stop_therapy(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut overlay = state.overlay.lock().await;
    stop_therapy(&mut *overlay).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn cmd_update_config(
    state: tauri::State<'_, AppState>,
    new_config: TherapyConfig,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    // Actualizar en almacenamiento y overlay (si esta activo)
    let mut overlay = state.overlay.lock().await;
    update_config(
        &mut *overlay,
        &state.config_storage,
        &new_config,
        screen_width,
        screen_height,
    )
    .await
    .map_err(|e| e.to_string())?;

    // Actualizar el estado en memoria
    let mut current = state.current_config.write().await;
    *current = new_config;
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Crear tray icon
            let _tray = TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip("Terapia Visual")
                .build(app)?;

            // Obtener el directorio del ejecutable para el archivo config.toml
            let config_dir = std::env::current_exe()
                .ok()
                .and_then(|exe| exe.parent().map(|p| p.to_path_buf()))
                .unwrap_or_else(|| std::path::PathBuf::from("."));

            // Inicializar adaptadores
            let config_storage = TomlConfigStorage::new(&config_dir);
            let notifier = TauriSystemNotifier::new(app.handle().clone());
            let overlay = TauriOverlay::new(app.handle().clone());

            // Cargar configuracion inicial desde el almacenamiento (o usar default)
            let initial_config = async_runtime::block_on(config_storage.load())
                .unwrap_or_else(|_| TherapyConfig::default());

            // Crear el estado
            let state = AppState {
                config_storage,
                overlay: Mutex::new(overlay),
                notifier,
                current_config: RwLock::new(initial_config),
            };
            app.manage(state);

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_notification::init())
        .invoke_handler(tauri::generate_handler![
            cmd_get_config,
            cmd_start_therapy,
            cmd_stop_therapy,
            cmd_update_config,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
