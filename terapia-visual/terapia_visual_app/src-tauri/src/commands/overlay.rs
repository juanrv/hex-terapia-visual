//! # Comandos de Terapia Overlay
//!
//! Este módulo contiene todos los comandos relacionados con la terapia de overlay
//! (pantalla completa con zonas de color transparentes).
//!
//! ## Comandos disponibles
//!
//! | Comando | Propósito |
//! |---------|-----------|
//! | `cmd_get_overlay_config` | Obtener la configuración actual del overlay |
//! | `cmd_start_overlay` | Iniciar la terapia de overlay |
//! | `cmd_stop_overlay` | Detener la terapia de overlay |
//! | `cmd_update_overlay_config` | Actualizar la configuración completa del overlay |
//! | `cmd_change_overlay_layout` | Cambiar el layout (vertical, horizontal, etc.) |
//! | `cmd_update_overlay_zone_color` | Cambiar el color de una zona específica |
//! | `cmd_update_overlay_zone_opacity` | Cambiar la opacidad de una zona específica |
//! | `cmd_reset_overlay_config` | Restablecer la configuración a los valores por defecto |
//!
//! ## Ejemplo desde el frontend
//!
//! ```typescript
//! import { invoke } from '@tauri-apps/api/core';
//!
//! // Obtener la configuración actual
//! const config = await invoke('cmd_get_overlay_config');
//!
//! // Iniciar la terapia
//! await invoke('cmd_start_overlay', { screenWidth: 1920, screenHeight: 1080 });
//!
//! // Cambiar el color de la primera zona
//! await invoke('cmd_update_overlay_zone_color', { zoneIndex: 0, newColor: '#FF0000', screenWidth: 1920, screenHeight: 1080 });
//! ```

use tauri::State;
use terapia_visual_domain::domain::{Color, Layout, Opacity, OverlayTherapyConfig};
use terapia_visual_domain::ports::SystemNotifier;
use terapia_visual_domain::use_cases::{
    start_overlay_therapy, stop_overlay_therapy, update_overlay_therapy,
};

use crate::state::AppState;

/// Obtiene la configuración actual de la terapia.
///
/// # Retorno
///
/// La configuración actual de la terapia.
///
/// # Ejemplo desde el frontend
///
/// ```typescript
/// const config = await invoke('cmd_get_therapy_config');
/// console.log('Configuración:', config);
/// ```
#[tauri::command]
pub async fn cmd_get_overlay_config(
    state: State<'_, AppState>,
) -> Result<OverlayTherapyConfig, String> {
    let config = state.overlay_config.read().await;
    Ok(config.clone())
}

/// Inicia la terapia visual.
///
/// # Argumentos
///
/// * `screen_width` - Ancho de la pantalla en píxeles.
/// * `screen_height` - Alto de la pantalla en píxeles.
///
/// # Errores
///
/// - Error si el overlay ya está activo.
/// - Error si falla la creación del overlay.
///
/// # Ejemplo desde el frontend
///
/// ```typescript
/// try {
///     await invoke('cmd_start_therapy', { screenWidth: 1920, screenHeight: 1080 });
///     console.log('Terapia iniciada');
/// } catch (error) {
///     console.error('Error al iniciar:', error);
/// }
/// ```
#[tauri::command]
pub async fn cmd_start_overlay(
    state: State<'_, AppState>,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    let config = state.overlay_config.read().await;
    let mut overlay = state.overlay.lock().await;

    start_overlay_therapy(&mut *overlay, &config, screen_width, screen_height)
        .await
        .map_err(|e| e.to_string())?;

    state
        .notifier
        .set_tray_state(true)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Detiene la terapia visual.
///
/// # Errores
///
/// - Error si el overlay ya está inactivo.
/// - Error si falla el cierre del overlay.
///
/// # Ejemplo desde el frontend
///
/// ```typescript
/// try {
///     await invoke('cmd_stop_therapy');
///     console.log('Terapia detenida');
/// } catch (error) {
///     console.error('Error al detener:', error);
/// }
/// ```
#[tauri::command]
pub async fn cmd_stop_overlay(state: State<'_, AppState>) -> Result<(), String> {
    let mut overlay = state.overlay.lock().await;
    stop_overlay_therapy(&mut *overlay)
        .await
        .map_err(|e| e.to_string())?;

    state
        .notifier
        .set_tray_state(false)
        .await
        .map_err(|e| e.to_string())?;
    Ok(())
}

/// Actualiza la configuración completa de la terapia.
///
/// # Argumentos
///
/// * `new_config` - Nueva configuración completa de la terapia.
/// * `screen_width` - Ancho de la pantalla en píxeles.
/// * `screen_height` - Alto de la pantalla en píxeles.
///
/// # Ejemplo desde el frontend
///
/// ```typescript
/// const newConfig = {
///     therapy_type: 'ColorDivision',
///     layout: 'Vertical',
///     zones_config: [
///         { color: '#FF0000', opacity: 0.8 },
///         { color: '#0000FF', opacity: 0.6 }
///     ]
/// };
/// await invoke('cmd_update_therapy_config', { newConfig, screenWidth: 1920, screenHeight: 1080 });
/// ```
#[tauri::command]
pub async fn cmd_update_overlay_config(
    state: State<'_, AppState>,
    new_config: OverlayTherapyConfig,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    let mut overlay = state.overlay.lock().await;

    update_overlay_therapy(
        &mut *overlay,
        &state.overlay_storage,
        &new_config,
        screen_width,
        screen_height,
    )
    .await
    .map_err(|e| e.to_string())?;

    let mut current = state.overlay_config.write().await;
    *current = new_config;
    Ok(())
}

/// Cambia el layout de la terapia (vertical, horizontal, checkerboard, etc.).
///
/// # Argumentos
///
/// * `new_layout` - El nuevo layout a aplicar.
/// * `screen_width` - Ancho de la pantalla en píxeles.
/// * `screen_height` - Alto de la pantalla en píxeles.
///
/// # Comportamiento
///
/// - Si el nuevo layout requiere más zonas, se clonan las existentes.
/// - Si requiere menos, se truncan las zonas sobrantes.
/// - Las reglas de sincronización se aplican automáticamente.
///
/// # Ejemplo desde el frontend
///
/// ```typescript
/// await invoke('cmd_change_layout', { newLayout: 'Checkerboard', screenWidth: 1920, screenHeight: 1080 });
/// ```
#[tauri::command]
pub async fn cmd_change_overlay_layout(
    state: State<'_, AppState>,
    new_layout: Layout,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    let mut current = state.overlay_config.write().await;
    current.change_layout(new_layout);

    let mut overlay = state.overlay.lock().await;
    update_overlay_therapy(
        &mut *overlay,
        &state.overlay_storage,
        &current,
        screen_width,
        screen_height,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Cambia el color de una zona específica.
///
/// Si el layout actual tiene reglas de sincronización (Checkerboard o Vertical4Columns),
/// los colores se sincronizan automáticamente entre las zonas emparejadas.
///
/// # Argumentos
///
/// * `zone_index` - Índice de la zona (base 0).
/// * `new_color` - Nuevo color en formato hexadecimal (#RRGGBB).
/// * `screen_width` - Ancho de la pantalla en píxeles.
/// * `screen_height` - Alto de la pantalla en píxeles.
///
/// # Errores
///
/// - Error si el índice de zona no existe.
/// - Error si el formato del color es inválido.
///
/// # Ejemplo desde el frontend
///
/// ```typescript
/// await invoke('cmd_update_zone_color', {
///     zoneIndex: 0,
///     newColor: '#FF0000',
///     screenWidth: 1920,
///     screenHeight: 1080
/// });
/// ```
#[tauri::command]
pub async fn cmd_update_overlay_zone_color(
    state: State<'_, AppState>,
    zone_index: usize,
    new_color: String,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    let color = Color::new(&new_color).map_err(|e| e.to_string())?;
    let mut current = state.overlay_config.write().await;

    // Método que sincroniza los colores si es Ajedrez
    current
        .update_zone_color(zone_index, color)
        .map_err(|e| e.to_string())?;

    let mut overlay = state.overlay.lock().await;
    update_overlay_therapy(
        &mut *overlay,
        &state.overlay_storage,
        &current,
        screen_width,
        screen_height,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Cambia la opacidad de una zona específica.
///
/// Si el layout actual tiene reglas de sincronización (Checkerboard o Vertical4Columns),
/// las opacidades se sincronizan automáticamente entre las zonas emparejadas.
///
/// # Argumentos
///
/// * `zone_index` - Índice de la zona (base 0).
/// * `new_opacity` - Nueva opacidad (0.0 a 0.8).
/// * `screen_width` - Ancho de la pantalla en píxeles.
/// * `screen_height` - Alto de la pantalla en píxeles.
///
/// # Errores
///
/// - Error si el índice de zona no existe.
/// - Error si la opacidad está fuera del rango permitido.
///
/// # Ejemplo desde el frontend
///
/// ```typescript
/// await invoke('cmd_update_zone_opacity', {
///     zoneIndex: 0,
///     newOpacity: 0.5,
///     screenWidth: 1920,
///     screenHeight: 1080
/// });
/// ```
#[tauri::command]
pub async fn cmd_update_overlay_zone_opacity(
    state: State<'_, AppState>,
    zone_index: usize,
    new_opacity: f32,
    screen_width: u32,
    screen_height: u32,
) -> Result<(), String> {
    let opacity = Opacity::new(new_opacity).map_err(|e| e.to_string())?;
    let mut current = state.overlay_config.write().await;

    // Método que sincroniza la opacidad si es Ajedrez
    current
        .update_zone_opacity(zone_index, opacity)
        .map_err(|e| e.to_string())?;

    let mut overlay = state.overlay.lock().await;
    update_overlay_therapy(
        &mut *overlay,
        &state.overlay_storage,
        &current,
        screen_width,
        screen_height,
    )
    .await
    .map_err(|e| e.to_string())?;

    Ok(())
}

/// Restablece la configuración de la terapia a los valores por defecto.
///
/// # Argumentos
///
/// * `screen_width` - Ancho de la pantalla en píxeles.
/// * `screen_height` - Alto de la pantalla en píxeles.
///
/// # Retorno
///
/// La nueva configuración por defecto.
///
/// # Ejemplo desde el frontend
///
/// ```typescript
/// const defaultConfig = await invoke('cmd_reset_therapy_config', { screenWidth: 1920, screenHeight: 1080 });
/// console.log('Configuración restablecida:', defaultConfig);
/// ```
#[tauri::command]
pub async fn cmd_reset_overlay_config(
    state: State<'_, AppState>,
    screen_width: u32,
    screen_height: u32,
) -> Result<OverlayTherapyConfig, String> {
    // Obtiene la configuracion segura por defecto del dominio
    let default_config = OverlayTherapyConfig::default();

    // Aplicarla al overlay y guardar en disco
    let mut overlay = state.overlay.lock().await;
    update_overlay_therapy(
        &mut *overlay,
        &state.overlay_storage,
        &default_config,
        screen_width,
        screen_height,
    )
    .await
    .map_err(|e| e.to_string())?;

    // Actualizar en memoria
    let mut current = state.overlay_config.write().await;
    *current = default_config.clone();

    // Retornar nueva configuracion
    Ok(default_config)
}
