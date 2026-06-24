use tauri::State;
use terapia_visual_domain::domain::reading_therapy_config::ReadingTherapyConfig;
use terapia_visual_domain::domain::{Color, Layout, Opacity, ReadingSettings};
use terapia_visual_domain::ports::{ReadingWindowPort, SystemNotifier};
use terapia_visual_domain::use_cases::{
    start_reading_therapy, stop_reading_therapy, update_reading_therapy,
};

use crate::state::AppState;

#[tauri::command]
pub async fn cmd_get_reading_config(
    state: State<'_, AppState>,
) -> Result<ReadingTherapyConfig, String> {
    let config = state.reading_config.read().await;
    Ok(config.clone())
}

#[tauri::command]
pub async fn cmd_start_reading_therapy(
    state: State<'_, AppState>,
    html_content: String,
) -> Result<(), String> {
    let config = state.reading_config.read().await;
    let mut window = state.reading_window.lock().await;

    start_reading_therapy(&mut *window, &config, &html_content)
        .await
        .map_err(|e| e.to_string())?;

    // Nota: Se podria no cambiar el icono del tray para la lectura si se quiere que
    // el tray sea exclusivo del overlay, pero por ahora se activa.
    let _ = state.notifier.set_tray_state(true).await;
    Ok(())
}

#[tauri::command]
pub async fn cmd_stop_reading_therapy(state: State<'_, AppState>) -> Result<(), String> {
    let mut window = state.reading_window.lock().await;
    stop_reading_therapy(&mut *window)
        .await
        .map_err(|e| e.to_string())?;

    let _ = state.notifier.set_tray_state(false).await;
    Ok(())
}

#[tauri::command]
pub async fn cmd_update_reading_config(
    state: State<'_, AppState>,
    new_config: ReadingTherapyConfig,
) -> Result<(), String> {
    let mut window = state.reading_window.lock().await;
    update_reading_therapy(&mut *window, &state.reading_storage, &new_config)
        .await
        .map_err(|e| e.to_string())?;

    let mut current = state.reading_config.write().await;
    *current = new_config;
    Ok(())
}

#[tauri::command]
pub async fn cmd_change_reading_layout(
    state: State<'_, AppState>,
    new_layout: Layout,
) -> Result<(), String> {
    let mut current = state.reading_config.write().await;
    current.change_layout(new_layout);

    let mut window = state.reading_window.lock().await;
    update_reading_therapy(&mut *window, &state.reading_storage, &current)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn cmd_update_reading_zone_color(
    state: State<'_, AppState>,
    zone_index: usize,
    new_color: String,
) -> Result<(), String> {
    let color = Color::new(&new_color).map_err(|e| e.to_string())?;
    let mut current = state.reading_config.write().await;

    current
        .update_zone_color(zone_index, color)
        .map_err(|e| e.to_string())?;

    let mut window = state.reading_window.lock().await;
    update_reading_therapy(&mut *window, &state.reading_storage, &current)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn cmd_update_reading_zone_opacity(
    state: State<'_, AppState>,
    zone_index: usize,
    new_opacity: f32,
) -> Result<(), String> {
    let opacity = Opacity::new(new_opacity).map_err(|e| e.to_string())?;
    let mut current = state.reading_config.write().await;

    current
        .update_zone_opacity(zone_index, opacity)
        .map_err(|e| e.to_string())?;

    let mut window = state.reading_window.lock().await;
    update_reading_therapy(&mut *window, &state.reading_storage, &current)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn cmd_update_reading_settings(
    state: State<'_, AppState>,
    new_settings: ReadingSettings,
) -> Result<(), String> {
    let mut current = state.reading_config.write().await;
    current.update_reading_settings(new_settings);

    let mut window = state.reading_window.lock().await;
    update_reading_therapy(&mut *window, &state.reading_storage, &current)
        .await
        .map_err(|e| e.to_string())?;

    Ok(())
}

#[tauri::command]
pub async fn cmd_reset_reading_config(
    state: State<'_, AppState>,
) -> Result<ReadingTherapyConfig, String> {
    let default_config = ReadingTherapyConfig::default();

    let mut window = state.reading_window.lock().await;
    update_reading_therapy(&mut *window, &state.reading_storage, &default_config)
        .await
        .map_err(|e| e.to_string())?;

    let mut current = state.reading_config.write().await;
    *current = default_config.clone();

    Ok(default_config)
}

/// Comando exclusivo para recalcular cuando el usuario estira la ventana de lectura
#[tauri::command]
pub async fn cmd_reading_window_resized(state: State<'_, AppState>) -> Result<(), String> {
    let mut window = state.reading_window.lock().await;

    if window.is_active() {
        let config = state.reading_config.read().await;
        // Al pedirle que se actualice, el Adaptador leerá el tamaño interno actual
        // de la ventana de Tauri y emitirá el JSON recalculado.
        let _ = window.update_config(&config).await;
    }

    Ok(())
}
