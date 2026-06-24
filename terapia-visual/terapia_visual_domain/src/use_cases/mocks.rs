//! # Mocks para pruebas unitarias
//!
//! Este módulo proporciona implementaciones falsas de los puertos del dominio
//! para poder probar los casos de uso de forma aislada, sin depender de
//! adaptadores reales (Tauri, sistema de archivos, etc.).
//!
//! # Mocks disponibles
//!
//! | Mock | Puerto que simula | Propósito |
//! |------|-------------------|-----------|
//! | `MockOverlay` | `OverlayPort` | Simula la ventana de overlay |
//! | `MockTherapyConfigStorage` | `ConfigStorage<TherapyConfig>` | Simula el almacenamiento de configuración de terapia |
//! | `MockAppConfigStorage` | `ConfigStorage<AppSettings>` | Simula el almacenamiento de configuración de la app |
//! | `MockSystemNotifier` | `SystemNotifier` | Simula notificaciones y bandeja del sistema |
//!
//! # Ejemplo de uso
//!
//! ```
//! use terapia_visual_domain::use_cases::mocks::MockOverlay;
//! use terapia_visual_domain::ports::OverlayPort;
//!
//! # async fn example() {
//! let mut overlay = MockOverlay::default();
//!
//! // Configurar el mock para simular un error
//! overlay.should_fail = true;
//!
//! // Usar el mock en un caso de uso
//! let result = some_use_case(&mut overlay).await;
//! assert!(result.is_err());
//! # }
//! # async fn some_use_case(_: &mut dyn OverlayPort) -> Result<(), Box<dyn std::error::Error>> {
//! # Ok(())
//! # }
//! ```

use async_trait::async_trait;

use crate::{
    domain::{AppSettings, OverlayTherapyConfig, reading_therapy_config::ReadingTherapyConfig},
    ports::{
        ConfigStorage, NotifierError, OverlayError, OverlayPort, ReadingWindowError,
        ReadingWindowPort, StorageError, SystemNotifier,
    },
};

/// Mock de overlay para pruebas.
///
/// Simula el comportamiento de un overlay sin crear ventanas reales.
/// Permite verificar que los casos de uso llaman a los métodos correctos.
#[derive(Debug, Default)]
pub struct MockOverlay {
    /// Indica si el overlay está activo (simulado).
    pub active: bool,
    /// Se establece a `true` cuando se llama a `show()`.
    pub show_called: bool,
    /// Se establece a `true` cuando se llama a `hide()`.
    pub hide_called: bool,
    /// Se establece a `true` cuando se llama a `update_config()`.
    pub update_config_called: bool,
    /// Última configuración recibida en `show()` o `update_config()`.
    pub last_config: Option<OverlayTherapyConfig>,
    /// Últimas dimensiones de pantalla recibidas.
    pub last_screen_size: Option<(u32, u32)>,
    /// Si es `true`, los métodos devuelven error.
    pub should_fail: bool,
}

#[async_trait]
impl OverlayPort for MockOverlay {
    async fn show(
        &mut self,
        config: &OverlayTherapyConfig,
        width: u32,
        height: u32,
    ) -> Result<(), crate::ports::OverlayError> {
        self.show_called = true;
        self.last_config = Some(config.clone());
        self.last_screen_size = Some((width, height));
        if self.should_fail {
            Err(OverlayError::CreationError("Error forzado".into()))
        } else {
            self.active = true;
            Ok(())
        }
    }

    async fn hide(&mut self) -> Result<(), OverlayError> {
        self.hide_called = true;
        if self.should_fail {
            Err(OverlayError::NotActive)
        } else {
            self.active = false;
            Ok(())
        }
    }

    async fn update_config(
        &mut self,
        config: &OverlayTherapyConfig,
        width: u32,
        height: u32,
    ) -> Result<(), OverlayError> {
        self.update_config_called = true;
        self.last_config = Some(config.clone());
        self.last_screen_size = Some((width, height));
        if self.should_fail {
            Err(OverlayError::UpdateError("Error forzado".into()))
        } else {
            Ok(())
        }
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

/// Mock de almacenamiento de configuración de terapia.
///
/// Simula el almacenamiento sin tocar el sistema de archivos.
#[derive(Debug, Default)]
pub struct MockTherapyConfigStorage {
    /// Configuración de terapia simulada (si existe).
    pub config: Option<OverlayTherapyConfig>,
    /// Se establece a `true` cuando se llama a `load()`.
    pub load_called: bool,
    /// Se establece a `true` cuando se llama a `save()`.
    pub save_called: bool,
    /// Si es `true`, `load()` devuelve error.
    pub should_fail_load: bool,
    /// Si es `true`, `save()` devuelve error.
    pub should_fail_save: bool,
}

#[async_trait]
impl ConfigStorage<OverlayTherapyConfig> for MockTherapyConfigStorage {
    async fn load(&self) -> Result<OverlayTherapyConfig, StorageError> {
        if self.should_fail_load {
            Err(StorageError::NotFound)
        } else {
            self.config.clone().ok_or(StorageError::NotFound)
        }
    }

    async fn save(&self, _config: &OverlayTherapyConfig) -> Result<(), StorageError> {
        if self.should_fail_save {
            Err(StorageError::WriteError("forced error".into()))
        } else {
            Ok(())
        }
    }
}

/// Mock de almacenamiento de configuración de la aplicación.
///
/// Simula el almacenamiento de preferencias globales.
pub struct MockAppConfigStorage {
    /// Configuración de la app simulada (si existe).
    pub app_settings: Option<AppSettings>,
    /// Si es `true`, `load()` y `save()` devuelven error.
    pub should_fail: bool,
}

#[async_trait]
impl ConfigStorage<AppSettings> for MockAppConfigStorage {
    async fn load(&self) -> Result<AppSettings, StorageError> {
        if self.should_fail {
            Err(StorageError::NotFound)
        } else {
            self.app_settings.clone().ok_or(StorageError::NotFound)
        }
    }

    async fn save(&self, _config: &AppSettings) -> Result<(), StorageError> {
        if self.should_fail {
            Err(StorageError::WriteError("forced error".into()))
        } else {
            Ok(())
        }
    }
}

/// Mock de notificador del sistema.
///
/// Simula notificaciones y bandeja del sistema sin interfaz real.
#[derive(Debug, Default)]
pub struct MockSystemNotifier {
    /// Último título recibido en `show_message()`.
    pub last_title: Option<String>,
    /// Último mensaje recibido en `show_message()`.
    pub last_message: Option<String>,
    /// Último estado recibido en `set_tray_state()`.
    pub last_state: Option<bool>,
    /// Se establece a `true` cuando se llama a `show_message()`.
    pub show_message_called: bool,
    /// Se establece a `true` cuando se llama a `set_tray_state()`.
    pub set_tray_state_called: bool,
    /// Si es `true`, los métodos devuelven error.
    pub should_fail: bool,
}

#[async_trait]
impl SystemNotifier for MockSystemNotifier {
    async fn show_message(&self, _title: &str, _message: &str) -> Result<(), NotifierError> {
        Ok(())
    }

    async fn set_tray_state(&self, _active: bool) -> Result<(), NotifierError> {
        Ok(())
    }
}

/// Mock de ventana de lectura
///
/// Simula el comportamiento de la ventana de lectura
#[derive(Debug, Default)]
pub struct MockReadingWindow {
    pub active: bool,
    pub show_called: bool,
    pub hide_called: bool,
    pub update_config_called: bool,
    pub last_html: Option<String>,
    pub should_fail: bool,
}

#[async_trait]
impl ReadingWindowPort for MockReadingWindow {
    async fn show(
        &mut self,
        _config: &ReadingTherapyConfig,
        html_content: &str,
    ) -> Result<(), ReadingWindowError> {
        self.show_called = true;
        self.last_html = Some(html_content.to_string());
        if self.should_fail {
            Err(ReadingWindowError::CreationError("Error forzaddo".into()))
        } else {
            self.active = true;
            Ok(())
        }
    }

    async fn hide(&mut self) -> Result<(), ReadingWindowError> {
        self.hide_called = true;
        if self.should_fail {
            Err(ReadingWindowError::NotActive)
        } else {
            self.active = false;
            Ok(())
        }
    }

    async fn update_config(
        &mut self,
        _config: &ReadingTherapyConfig,
    ) -> Result<(), ReadingWindowError> {
        self.update_config_called = true;
        if self.should_fail {
            Err(ReadingWindowError::UpdateError("Error forzado".into()))
        } else {
            Ok(())
        }
    }

    fn is_active(&self) -> bool {
        self.active
    }
}

/// Mock de almacenamiento de configuración de lectura.
#[derive(Debug, Default)]
pub struct MockReadingConfigStorage {
    pub config: Option<ReadingTherapyConfig>,
    pub should_fail_load: bool,
    pub should_fail_save: bool,
}

#[async_trait]
impl ConfigStorage<ReadingTherapyConfig> for MockReadingConfigStorage {
    async fn load(&self) -> Result<ReadingTherapyConfig, StorageError> {
        if self.should_fail_load {
            Err(StorageError::NotFound)
        } else {
            self.config.clone().ok_or(StorageError::NotFound)
        }
    }

    async fn save(&self, _config: &ReadingTherapyConfig) -> Result<(), StorageError> {
        if self.should_fail_save {
            Err(StorageError::WriteError("forced error".into()))
        } else {
            Ok(())
        }
    }
}
