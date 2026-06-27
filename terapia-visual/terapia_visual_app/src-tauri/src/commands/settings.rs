//! # Comandos de Configuración de la Aplicación
//!
//! Este módulo contiene los comandos relacionados con la configuración global
//! de la aplicación (idioma, preferencias, etc.).
//!
//! ## Comandos disponibles
//!
//! | Comando | Propósito |
//! |---------|-----------|
//! | `cmd_get_app_settings` | Obtener la configuración de la aplicación |
//! | `cmd_update_app_settings` | Actualizar la configuración de la aplicación |
//!
//! ## Ejemplo desde el frontend
//!
//! ```typescript
//! import { invoke } from '@tauri-apps/api/core';
//!
//! // Obtener la configuración actual
//! const settings = await invoke('cmd_get_app_settings');
//! console.log('Idioma actual:', settings.language);
//!
//! // Cambiar a inglés
//! await invoke('cmd_update_app_settings', { newSettings: { language: 'en' } });
//! ```

use tauri::State;
use terapia_visual_adapter::messages::{self, init_language};
use terapia_visual_domain::domain::AppSettings;
use terapia_visual_domain::ports::ConfigStorage;
use terapia_visual_domain::use_cases::{get_app_settings, update_app_settings};

use tauri::Manager;

use crate::state::AppState;

/// Obtiene la configuración de la aplicación (idioma, etc.).
///
/// # Retorno
///
/// La configuración actual de la aplicación.
///
/// # Ejemplo desde el frontend
///
/// ```typescript
/// const settings = await invoke('cmd_get_app_settings');
/// console.log('Idioma:', settings.language);
/// ```
#[tauri::command]
pub async fn cmd_get_app_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let settings = get_app_settings(&state.app_storage).await;
    Ok(settings)
}

/// Actualiza la configuración de la aplicación (idioma, etc.).
///
/// # Argumentos
///
/// * `new_settings` - Nueva configuración de la aplicación.
///
/// # Efectos secundarios
///
/// - Reinicializa el sistema de mensajes con el nuevo idioma.
/// - Actualiza el tooltip de la bandeja.
/// - Actualiza el título de la ventana principal.
///
/// # Ejemplo desde el frontend
///
/// ```typescript
/// await invoke('cmd_update_app_settings', {
///     newSettings: { language: 'en' }
/// });
/// ```
#[tauri::command]
pub async fn cmd_update_app_settings<R: tauri::Runtime>(
    app_handle: tauri::AppHandle<R>,
    state: State<'_, AppState>,
    new_settings: AppSettings,
) -> Result<(), String> {
    update_app_settings(&state.app_storage, &new_settings)
        .await
        .map_err(|e| e.to_string())?;

    init_language(&new_settings);

    if let Some(tray) = app_handle.tray_by_id("main") {
        tray.set_tooltip(Some(messages::tooltip_app_name()))
            .map_err(|e| e.to_string())?;
    }

    if let Some(main_window) = app_handle.get_webview_window("main") {
        main_window
            .set_title(messages::window_title())
            .map_err(|e| e.to_string())?;
    }

    Ok(())
}

/// Cierra la aplicación de forma segura guardando la configuración de todas las terapias.
#[tauri::command]
pub async fn cmd_exit_app<R: tauri::Runtime>(
    state: State<'_, AppState>,
    app_handle: tauri::AppHandle<R>,
) -> Result<(), String> {
    // Guardar todo
    let overlay_config = state.overlay_config.read().await;
    if let Err(e) = state.overlay_storage.save(&*overlay_config).await {
        eprintln!("Error saving overlay config: {}", e);
    }

    let reading_config = state.reading_config.read().await;
    if let Err(e) = state.reading_storage.save(&*reading_config).await {
        eprintln!("Error saving reading config: {}", e);
    }

    let app_settings: AppSettings = state.app_storage.load().await.unwrap_or_default();
    if let Err(e) = state.app_storage.save(&app_settings).await {
        eprintln!("Error saving app config: {}", e);
    }

    // Apagado en segundo plano con micro-retraso
    tauri::async_runtime::spawn(async move {
        tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        app_handle.exit(0);
    });

    Ok(())
}
