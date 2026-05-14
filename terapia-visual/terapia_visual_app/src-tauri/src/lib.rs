// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/

use tauri::tray::TrayIconBuilder;
use tauri::{async_runtime, Manager};
use terapia_visual_adapter::messages::{self, init_language};
use terapia_visual_domain::ports::{ConfigStorage, SystemNotifier};
use tokio::sync::{Mutex, RwLock};

// Importaciones de dominio y adaptadores
use terapia_visual_adapter::config_storage::{TomlAppConfigStorage, TomlTherapyConfigStorage};
use terapia_visual_adapter::notifier::TauriSystemNotifier;
use terapia_visual_adapter::overlay::TauriOverlay;
use terapia_visual_domain::domain::{AppSettings, TherapyConfig};
use terapia_visual_domain::use_cases::{
    get_app_settings, start_therapy, stop_therapy, update_app_settings, update_therapy_config,
};

// Estado global de la aplicacion
pub struct AppState {
    pub therapy_storage: TomlTherapyConfigStorage,
    pub app_storage: TomlAppConfigStorage,
    pub overlay: Mutex<TauriOverlay>,
    pub notifier: TauriSystemNotifier,
    pub current_config: RwLock<TherapyConfig>,
}

// TODO: Intentar limpiar el archivo lib.rs para que no todos los comandos se encuentren aqui y
// hacer el archivo mas largo de lo que ya esta
#[tauri::command]
async fn cmd_get_therapy_config(
    state: tauri::State<'_, AppState>,
) -> Result<TherapyConfig, String> {
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
        .map_err(|e| e.to_string())?;

    state
        .notifier
        .set_tray_state(true)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn cmd_stop_therapy(state: tauri::State<'_, AppState>) -> Result<(), String> {
    let mut overlay = state.overlay.lock().await;
    stop_therapy(&mut *overlay)
        .await
        .map_err(|e| e.to_string())?;

    state
        .notifier
        .set_tray_state(false)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
async fn cmd_update_therapy_config(
    state: tauri::State<'_, AppState>,
    new_config: TherapyConfig,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    // Actualizar en almacenamiento y overlay (si esta activo)
    let mut overlay = state.overlay.lock().await;
    update_therapy_config(
        &mut *overlay,
        &state.therapy_storage as &dyn ConfigStorage<TherapyConfig>,
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

#[tauri::command]
async fn cmd_get_app_settings(state: tauri::State<'_, AppState>) -> Result<AppSettings, String> {
    let settings = get_app_settings(&state.app_storage).await;
    Ok(settings)
}

#[tauri::command]
async fn cmd_update_app_settings(
    app_handle: tauri::AppHandle,
    state: tauri::State<'_, AppState>,
    new_settings: AppSettings,
) -> Result<(), String> {
    update_app_settings(&state.app_storage, &new_settings)
        .await
        .map_err(|e| e.to_string())?;
    init_language(&new_settings);

    // Actualizar el tooltip del tray
    let app_handle = app_handle.clone();
    if let Some(tray) = app_handle.tray_by_id("main") {
        println!("[DEBUG] New tooltip name: {}", messages::tooltip_app_name());
        tray.set_tooltip(Some(messages::tooltip_app_name()))
            .map_err(|e| e.to_string())?;
    }

    // Actualizar el titulo de la ventana
    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window
            .set_title(messages::window_title())
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            // Obtener el directorio del ejecutable para el archivo config.toml
            let config_dir = app
                .path()
                .app_data_dir()
                .expect("Failed to optain app data dir");
            // Crear el directorio en caso de que no exista
            std::fs::create_dir_all(&config_dir)?;

            // Inicializar adaptadores
            let therapy_storage = TomlTherapyConfigStorage::new(&config_dir);
            let app_storage = TomlAppConfigStorage::new(&config_dir);
            let notifier = TauriSystemNotifier::new(app.handle().clone());
            let overlay = TauriOverlay::new(app.handle().clone());

            // Guardar configuración inicial en el almacenamiento
            let _initial_config = match async_runtime::block_on(therapy_storage.load()) {
                Ok(cfg) => cfg,
                Err(_) => {
                    let default = TherapyConfig::default();
                    if let Err(e) = async_runtime::block_on(therapy_storage.save(&default)) {
                        eprintln!("Error while saving default config: {}", e);
                    }
                    default
                }
            };

            // Cargar configuracion inicial desde el almacenamiento (o usar default)
            let initial_config = async_runtime::block_on(therapy_storage.load())
                .unwrap_or_else(|_| TherapyConfig::default());

            // Cargar configuracion de la app (o usar default)
            let app_settings = match async_runtime::block_on(app_storage.load()) {
                Ok(cfg) => cfg,
                Err(_) => AppSettings::default(),
            };

            // Inicializar mensajes con el idioma cargado
            init_language(&app_settings);
            println!("[DEBUG] Tooltip name: {}", messages::tooltip_app_name());

            // Inicializar el titulo de la ventana principal segun el idioma cargado

            if let Some(main_window) = app.get_webview_window("main") {
                let _ = main_window
                    .set_title(messages::window_title())
                    .map_err(|e| println!("Error setting window title: {}", e));
            }

            // Crear tray icon
            let _tray = TrayIconBuilder::with_id("main")
                .icon(app.default_window_icon().unwrap().clone())
                .tooltip(messages::tooltip_app_name())
                .build(app)?;

            // Crear el estado
            let state = AppState {
                therapy_storage,
                app_storage,
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
            cmd_get_therapy_config,
            cmd_start_therapy,
            cmd_stop_therapy,
            cmd_update_therapy_config,
            cmd_get_app_settings,
            cmd_update_app_settings,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
