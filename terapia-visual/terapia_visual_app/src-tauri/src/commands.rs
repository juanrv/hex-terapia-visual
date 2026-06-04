use tauri::State;
use terapia_visual_adapter::messages::{self, init_language};
use terapia_visual_domain::domain::{AppSettings, Color, Layout, Opacity, TherapyConfig};
use terapia_visual_domain::ports::SystemNotifier;
use terapia_visual_domain::use_cases::{
    get_app_settings, start_therapy, stop_therapy, update_app_settings, update_therapy_config,
};

use tauri::Manager;

use crate::state::AppState;

#[tauri::command]
pub async fn cmd_get_therapy_config(state: State<'_, AppState>) -> Result<TherapyConfig, String> {
    let config = state.current_config.read().await;
    Ok(config.clone())
}

#[tauri::command]
pub async fn cmd_start_therapy(
    state: State<'_, AppState>,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    let config = state.current_config.read().await;
    let mut overlay = state.overlay.lock().await;

    start_therapy(&mut *overlay, &*config, screen_width, screen_height)
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
pub async fn cmd_stop_therapy(state: State<'_, AppState>) -> Result<(), String> {
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
pub async fn cmd_update_therapy_config(
    state: State<'_, AppState>,
    new_config: TherapyConfig,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    let mut overlay = state.overlay.lock().await;

    update_therapy_config(
        &mut *overlay,
        &state.therapy_storage,
        &new_config,
        screen_width,
        screen_height,
    )
    .await
    .map_err(|e| e.to_string())?;

    let mut current = state.current_config.write().await;
    *current = new_config;
    Ok(())
}

#[tauri::command]
pub async fn cmd_get_app_settings(state: State<'_, AppState>) -> Result<AppSettings, String> {
    let settings = get_app_settings(&state.app_storage).await;
    Ok(settings)
}

#[tauri::command]
pub async fn cmd_update_app_settings(
    app_handle: tauri::AppHandle,
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

#[tauri::command]
pub async fn cmd_change_layout(
    state: State<'_, AppState>,
    new_layout: Layout,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    let mut current = state.current_config.write().await;
    current.change_layout(new_layout);

    let mut overlay = state.overlay.lock().await;
    update_therapy_config(
        &mut *overlay,
        &state.therapy_storage,
        &*current,
        screen_width,
        screen_height,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn cmd_update_zone_color(
    state: State<'_, AppState>,
    zone_index: usize,
    new_color: String,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    let color = Color::new(&new_color).map_err(|e| e.to_string())?;
    let mut current = state.current_config.write().await;

    // Método que sincroniza los colores si es Ajedrez
    current
        .update_zone_color(zone_index, color)
        .map_err(|e| e.to_string())?;

    let mut overlay = state.overlay.lock().await;
    update_therapy_config(
        &mut *overlay,
        &state.therapy_storage,
        &*current,
        screen_width,
        screen_height,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn cmd_update_zone_opacity(
    state: State<'_, AppState>,
    zone_index: usize,
    new_opacity: f32,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    let opacity = Opacity::new(new_opacity).map_err(|e| e.to_string())?;
    let mut current = state.current_config.write().await;

    // Método que sincroniza la opacidad si es Ajedrez
    current
        .update_zone_opacity(zone_index, opacity)
        .map_err(|e| e.to_string())?;

    let mut overlay = state.overlay.lock().await;
    update_therapy_config(
        &mut *overlay,
        &state.therapy_storage,
        &*current,
        screen_width,
        screen_height,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}
