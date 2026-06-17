//! # Punto de Entrada de la Aplicación
//!
//! Este es el punto de entrada principal del ejecutable.
//! Simplemente llama a [`terapia_visual_app_lib::run()`] para iniciar la aplicación.
//!
//! # Ejecución
//!
//! ```bash
//! cargo run
//! # o
//! cargo tauri dev
//! ```

// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    terapia_visual_app_lib::run()
}
