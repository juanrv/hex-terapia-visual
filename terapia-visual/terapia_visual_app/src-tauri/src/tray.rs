//! # Bandeja del Sistema (System Tray)
//!
//! Este módulo configura la bandeja del sistema de la aplicación, incluyendo
//! el icono, el tooltip, el menú contextual y los eventos de interacción.
//!
//! # Características
//!
//! - **Icono**: Muestra un icono en la bandeja del sistema.
//! - **Tooltip**: Texto que aparece al pasar el ratón sobre el icono (traducible).
//! - **Menú contextual**: Opción "Salir" para cerrar la aplicación.
//! - **Doble clic**: Restaura la ventana principal si está oculta.
//!
//! # Flujo típico
//!
//! 1. [`create_tray()`] se llama desde [`crate::setup::init`].
//! 2. Se crea el icono de la bandeja con el ID `"main"`.
//! 3. Se añade un menú contextual con la opción "Salir".
//! 4. Se configura el evento de doble clic para restaurar la ventana.
//!
//! # Ejemplo de uso
//!
//! ```no_run
//! use tauri::App;
//! use terapia_visual_app_lib::tray::create_tray;
//!
//! # fn example(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
//! create_tray(app)?;
//! # Ok(())
//! # }
//! ```

use tauri::menu::{Menu, MenuItem, PredefinedMenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::{App, Manager};
use terapia_visual_adapter::messages;

/// Crea y configura la bandeja del sistema.
///
/// Esta función se llama durante la inicialización de la aplicación.
/// Configura el icono, el tooltip, el menú contextual y los eventos.
///
/// # Argumentos
///
/// * `app` - La aplicación Tauri en construcción.
///
/// # Errores
///
/// Devuelve un error si falla la creación de la bandeja o del menú.
///
/// # Comportamiento
///
/// 1. Crea el icono de la bandeja con el ID `"main"`.
/// 2. El tooltip inicial se obtiene de [`messages::tooltip_app_name()`] (traducible).
/// 3. Añade un menú contextual con la opción "Salir".
/// 4. Al hacer doble clic, restaura la ventana principal si está oculta.
///
/// # Ejemplo
///
/// ```no_run
/// use tauri::App;
/// use terapia_visual_app_lib::tray::create_tray;
///
/// # fn example(app: &mut App) -> Result<(), Box<dyn std::error::Error>> {
/// create_tray(app)?;
/// # Ok(())
/// # }
/// ```
pub fn create_tray(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let tray = TrayIconBuilder::with_id("main")
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip(messages::tooltip_app_name())
        .build(app)?;

    let menu = Menu::new(app)?;

    let open_item = MenuItem::with_id(app, "open_main", messages::tray_open(), true, None::<&str>)?;

    let separator1 = PredefinedMenuItem::separator(app)?;

    let overlay_item = MenuItem::with_id(
        app,
        "nav_overlay",
        messages::tray_overlay(),
        true,
        None::<&str>,
    )?;
    let reading_item = MenuItem::with_id(
        app,
        "nav_reading",
        messages::tray_reading(),
        true,
        None::<&str>,
    )?;

    let separator2 = PredefinedMenuItem::separator(app)?;

    let quit_item = MenuItem::with_id(app, "quit", messages::tray_quit(), true, None::<&str>)?;

    menu.append(&open_item)?;
    menu.append(&separator1)?;
    menu.append(&overlay_item)?;
    menu.append(&reading_item)?;
    menu.append(&separator2)?;
    menu.append(&quit_item)?;
    tray.set_menu(Some(menu))?;

    tray.on_tray_icon_event(|tray, event| {
        if let TrayIconEvent::DoubleClick { .. } = event {
            let app_handle = tray.app_handle();
            if let Some(main_window) = app_handle.get_webview_window("main") {
                // Si la ventana esta oculta, mostrarla y traerla al frente
                let _ = main_window.show();
                let _ = main_window.set_focus();
                // También restaurar si estaba minimizada
                let _ = main_window.unminimize();
            }
        }
    });

    Ok(())
}
