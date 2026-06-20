//! # Comandos Tauri para el Frontend
//!
//! Este módulo contiene todos los comandos que el frontend (TypeScript) puede
//! invocar para interactuar con la aplicación.
//!
//! # Comandos disponibles
//!
//! | Comando | Propósito |
//! |---------|-----------|
//! | `cmd_get_therapy_config` | Obtener la configuración actual de la terapia |
//! | `cmd_start_therapy` | Iniciar la terapia visual |
//! | `cmd_stop_therapy` | Detener la terapia visual |
//! | `cmd_update_therapy_config` | Actualizar la configuración completa de la terapia |
//! | `cmd_change_layout` | Cambiar el layout (vertical, horizontal, etc.) |
//! | `cmd_update_zone_color` | Cambiar el color de una zona específica |
//! | `cmd_update_zone_opacity` | Cambiar la opacidad de una zona específica |
//! | `cmd_reset_therapy_config` | Restablecer la configuración a los valores por defecto |
//! | `cmd_get_app_settings` | Obtener la configuración de la aplicación (idioma, etc.) |
//! | `cmd_update_app_settings` | Actualizar la configuración de la aplicación |
//!
//! # Flujo típico
//!
//! 1. El frontend llama a un comando usando `invoke()`.
//! 2. El comando obtiene el estado de la aplicación (`AppState`).
//! 3. El comando llama al caso de uso correspondiente.
//! 4. El comando devuelve el resultado al frontend.
//!
//! # Ejemplo desde el frontend
//!
//! ```typescript
//! import { invoke } from '@tauri-apps/api/core';
//!
//! // Iniciar la terapia
//! await invoke('cmd_start_therapy', { screenWidth: 1920, screenHeight: 1080 });
//!
//! // Cambiar el layout a horizontal
//! await invoke('cmd_change_layout', { newLayout: 'Horizontal', screenWidth: 1920, screenHeight: 1080 });
//!
//! // Cambiar el color de la primera zona a azul
//! await invoke('cmd_update_zone_color', { zoneIndex: 0, newColor: '#0000FF', screenWidth: 1920, screenHeight: 1080 });
//! ```

use tauri::State;
use terapia_visual_adapter::messages::{self, init_language};
use terapia_visual_domain::domain::{AppSettings, Color, Layout, Opacity, OverlayTherapyConfig};
use terapia_visual_domain::ports::SystemNotifier;
use terapia_visual_domain::use_cases::{
    get_app_settings, start_overlay_therapy, stop_overlay_therapy, update_app_settings,
    update_overlay_therapy,
};

use tauri::Manager;

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
pub async fn cmd_get_therapy_config(
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
pub async fn cmd_start_therapy(
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
pub async fn cmd_stop_therapy(state: State<'_, AppState>) -> Result<(), String> {
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
pub async fn cmd_update_therapy_config(
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
pub async fn cmd_change_layout(
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
pub async fn cmd_update_zone_color(
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
pub async fn cmd_update_zone_opacity(
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
pub async fn cmd_reset_therapy_config(
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
