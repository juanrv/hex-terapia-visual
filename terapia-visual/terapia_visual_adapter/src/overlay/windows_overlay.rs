// src/overlay/windows_overlay.rs
use async_trait::async_trait;
use once_cell::sync::Lazy;

use std::collections::HashMap;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::thread;
use tracing::{info, warn};
use windows::{
    Win32::Foundation::*, Win32::Graphics::Gdi::*, Win32::System::LibraryLoader::GetModuleHandleA,
    Win32::UI::WindowsAndMessaging::*, core::*,
};

use terapia_visual_domain::domain::{TherapyConfig, Zone};
use terapia_visual_domain::ports::{OverlayError, OverlayPort};

// Función auxiliar para convertir r,g,b en COLORREF
fn rgb(r: u8, g: u8, b: u8) -> COLORREF {
    COLORREF((r as u32) | ((g as u32) << 8) | ((b as u32) << 16))
}

// Mapa global que asocia cada ventana con su puntero al objeto WindowsOverlay
static OVERLAY_MAP: Lazy<Mutex<HashMap<HWND, *mut WindowsOverlay>>> =
    Lazy::new(|| Mutex::new(HashMap::new()));

// Estructura principal
pub struct WindowsOverlay {
    hwnd: Option<HWND>,
    is_active: Arc<AtomicBool>,
    current_zones: Vec<Zone>,
    message_thread: Option<thread::JoinHandle<()>>,
}

impl WindowsOverlay {
    pub fn new() -> Self {
        Self {
            hwnd: None,
            is_active: Arc::new(AtomicBool::new(false)),
            current_zones: Vec::new(),
            message_thread: None,
        }
    }

    // Registro de la clase de ventana (llamado una sola vez, pero lo haremos en show por simplicidad)
    fn register_window_class() -> std::result::Result<(), OverlayError> {
        let instance = unsafe { GetModuleHandleA(None) }
            .map_err(|_| OverlayError::CreationError("GetModuleHandleA failed".into()))?;
        let class_name = PCSTR(b"VisualTherapyOverlay\0".as_ptr());

        let wc = WNDCLASSEXA {
            cbSize: std::mem::size_of::<WNDCLASSEXA>() as u32,
            style: CS_HREDRAW | CS_VREDRAW,
            lpfnWndProc: Some(Self::window_proc),
            hInstance: instance.into(),
            lpszClassName: class_name,
            ..Default::default()
        };

        let atom = unsafe { RegisterClassExA(&wc) };
        if atom == 0 {
            return Err(OverlayError::CreationError(
                "RegisterClassExA failed".into(),
            ));
        }
        Ok(())
    }

    // Procedimiento de ventana (callback)
    extern "system" fn window_proc(
        hwnd: HWND,
        msg: u32,
        wparam: WPARAM,
        lparam: LPARAM,
    ) -> LRESULT {
        // Obtener el puntero al objeto WindowsOverlay almacenado en GWLP_USERDATA
        let ptr = unsafe { GetWindowLongPtrA(hwnd, GWLP_USERDATA) };
        if ptr != 0 {
            let this = ptr as *mut WindowsOverlay;
            match msg {
                WM_PAINT => {
                    unsafe {
                        let mut ps = PAINTSTRUCT::default();
                        let hdc = BeginPaint(hwnd, &mut ps);
                        if hdc.0 != 0 {
                            // Llamar a draw_zones (necesitamos acceso al this)
                            let overlay = &mut *this;
                            overlay.draw_zones_with_hdc(hwnd, hdc);
                            EndPaint(hwnd, &ps);
                        }
                    }
                    LRESULT(0)
                }
                WM_DESTROY => {
                    unsafe { PostQuitMessage(0) };
                    LRESULT(0)
                }
                _ => unsafe { DefWindowProcA(hwnd, msg, wparam, lparam) },
            }
        } else {
            unsafe { DefWindowProcA(hwnd, msg, wparam, lparam) }
        }
    }

    // Versión de draw_zones que usa un HDC proporcionado (para WM_PAINT)
    fn draw_zones_with_hdc(&self, hdc: HDC) {
        if hdc.0 == 0 {
            return;
        }
        for zone in &self.current_zones {
            let rect = zone.rect();
            let color_str = zone.color().as_str();
            if color_str.len() != 7 || !color_str.starts_with('#') {
                continue;
            }
            let r = u8::from_str_radix(&color_str[1..3], 16).unwrap_or(0);
            let g = u8::from_str_radix(&color_str[3..5], 16).unwrap_or(0);
            let b = u8::from_str_radix(&color_str[5..7], 16).unwrap_or(0);

            let brush = unsafe { CreateSolidBrush(rgb(r, g, b)) };
            if brush.0 != 0 {
                let win_rect = RECT {
                    left: rect.x as i32,
                    top: rect.y as i32,
                    right: (rect.x + rect.width) as i32,
                    bottom: (rect.y + rect.height) as i32,
                };
                unsafe { FillRect(hdc, &win_rect, brush) };
                unsafe { DeleteObject(brush) };
            }
        }
    }

    // Dibuja todas las zonas (versión pública para llamar desde show/update)
    fn draw_zones(&self, hwnd: HWND) {
        let hdc = unsafe { GetDC(hwnd) };
        if hdc.0 == 0 {
            warn!("GetDC returned 0");
            return;
        }
        self.draw_zones_with_hdc(hwnd, hdc);
        unsafe { ReleaseDC(hwnd, hdc) };
    }

    // Configura la ventana como layered (transparencia y click-through)
    fn set_transparent_style(hwnd: HWND) -> std::result::Result<(), OverlayError> {
        let ex_style = unsafe { GetWindowLongA(hwnd, GWL_EXSTYLE) };
        let new_ex_style =
            WINDOW_EX_STYLE(ex_style as u32) | WS_EX_LAYERED | WS_EX_TRANSPARENT | WS_EX_TOPMOST;
        unsafe { SetWindowLongA(hwnd, GWL_EXSTYLE, new_ex_style.0 as i32) };
        let res = unsafe { SetLayeredWindowAttributes(hwnd, COLORREF(0), 255, LWA_ALPHA) };
        if res.is_ok() {
            Ok(())
        } else {
            Err(OverlayError::CreationError(
                "SetLayeredWindowAttributes failed".into(),
            ))
        }
    }
}

#[async_trait]
impl OverlayPort for WindowsOverlay {
    async fn show(
        &mut self,
        config: &TherapyConfig,
        screen_width: u32,
        screen_height: u32,
    ) -> std::result::Result<(), OverlayError> {
        if self.is_active() {
            return Err(OverlayError::AlreadyActive);
        }

        // Registrar clase de ventana (si ya está registrada, no pasa nada)
        Self::register_window_class()?;

        let instance = unsafe { GetModuleHandleA(None) }
            .map_err(|_| OverlayError::CreationError("GetModuleHandleA failed".into()))?;

        let class_name = "VisualTherapyOverlay";
        let window_name = "Visual Therapy Overlay";

        let hwnd = unsafe {
            CreateWindowExA(
                WINDOW_EX_STYLE::default(),
                PCSTR(class_name.as_ptr()),
                PCSTR(window_name.as_ptr()),
                WS_POPUP,
                0,
                0,
                screen_width as i32,
                screen_height as i32,
                None,
                None,
                instance,
                None,
            )
        };

        if hwnd.0 == 0 {
            return Err(OverlayError::CreationError("CreateWindowExA failed".into()));
        }

        // Almacenar el puntero a self en la ventana
        unsafe {
            SetWindowLongPtrA(hwnd, GWLP_USERDATA, self as *mut _ as isize);
        }

        // Generar zonas
        let zones = config.generate_zones(screen_width, screen_height);
        self.current_zones = zones;

        // Configurar transparencia y click-through
        Self::set_transparent_style(hwnd)?;

        // Mostrar ventana
        unsafe { ShowWindow(hwnd, SW_SHOW) };
        self.hwnd = Some(hwnd);
        self.is_active.store(true, Ordering::SeqCst);

        // Dibujar contenido inicial
        self.draw_zones(hwnd);

        // Lanzar el bucle de mensajes en un hilo separado
        let hwnd_clone = hwnd;
        let is_active_clone = self.is_active.clone();
        let handle = thread::spawn(move || {
            let mut msg = MSG::default();
            // Bucle de mensajes: GetMessage se bloquea hasta recibir un mensaje.
            // El bucle termina cuando GetMessage devuelve 0 (WM_QUIT) o -1 (error).
            loop {
                let ret = unsafe { GetMessageA(&mut msg, HWND(0), 0, 0) };
                if ret.0 == 0 || ret.0 == -1 {
                    break;
                }
                unsafe {
                    TranslateMessage(&msg);
                    DispatchMessageA(&msg);
                }
                // Si la ventana ha sido destruida (por ejemplo, por WM_DESTROY), salimos.
                if unsafe { IsWindow(hwnd_clone) }.as_bool() == false {
                    break;
                }
            }
            // El hilo termina, el overlay ya no está activo.
            is_active_clone.store(false, Ordering::Relaxed);
        });
        self.message_thread = Some(handle);

        info!("Overlay window created with message loop");
        Ok(())
    }

    async fn hide(&mut self) -> std::result::Result<(), OverlayError> {
        if let Some(hwnd) = self.hwnd {
            // Enviar WM_QUIT para detener el bucle de mensajes
            unsafe { PostQuitMessage(0) };
            // Esperar a que el hilo termine (opcional, pero limpio)
            if let Some(handle) = self.message_thread.take() {
                let _ = handle.join(); // puede bloquear brevemente, pero hide es async.
            }
            // Destruir la ventana (esto también enviará WM_DESTROY)
            unsafe {
                DestroyWindow(hwnd)
                    .map_err(|_| OverlayError::CloseError("DestroyWindow failed".into()))?;
            }
            self.hwnd = None;
            self.is_active.store(false, Ordering::SeqCst);
            info!("Overlay window destroyed");
            Ok(())
        } else {
            Err(OverlayError::NotActive)
        }
    }

    async fn update_config(
        &mut self,
        config: &TherapyConfig,
        screen_width: u32,
        screen_height: u32,
    ) -> std::result::Result<(), OverlayError> {
        let zones = config.generate_zones(screen_width, screen_height);
        self.current_zones = zones;

        if let Some(hwnd) = self.hwnd {
            // Forzar redibujado completo
            unsafe { InvalidateRect(hwnd, None, TRUE) };
        }
        Ok(())
    }

    fn is_active(&self) -> bool {
        self.is_active.load(Ordering::SeqCst)
    }
}
