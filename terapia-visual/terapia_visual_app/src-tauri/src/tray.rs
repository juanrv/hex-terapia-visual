use tauri::menu::{Menu, MenuItem};
use tauri::tray::{TrayIconBuilder, TrayIconEvent};
use tauri::{App, Manager};
use terapia_visual_adapter::messages;

pub fn create_tray(app: &App) -> Result<(), Box<dyn std::error::Error>> {
    let tray = TrayIconBuilder::with_id("main")
        .icon(app.default_window_icon().unwrap().clone())
        .tooltip(messages::tooltip_app_name())
        .build(app)?;

    let menu = Menu::new(app)?;
    let quit_item = MenuItem::with_id(app, "quit", "Salir", true, None::<&str>)?;
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
