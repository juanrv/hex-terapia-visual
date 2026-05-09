use async_trait::async_trait;

use crate::{
    domain::{AppSettings, TherapyConfig},
    ports::{
        ConfigStorage, NotifierError, OverlayError, OverlayPort, StorageError, SystemNotifier,
    },
};

/// Mock de overlay para pruebas
#[derive(Debug, Default)]
pub struct MockOverlay {
    pub active: bool,
    pub show_called: bool,
    pub hide_called: bool,
    pub update_config_called: bool,
    pub last_config: Option<TherapyConfig>,
    pub last_screen_size: Option<(u32, u32)>,
    pub should_fail: bool,
}

#[async_trait]
impl OverlayPort for MockOverlay {
    async fn show(
        &mut self,
        config: &TherapyConfig,
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
        config: &TherapyConfig,
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

/// Mock de TherapyConfigStorage para pruebas
#[derive(Debug, Default)]
pub struct MockTherapyConfigStorage {
    pub config: Option<TherapyConfig>,
    pub load_called: bool,
    pub save_called: bool,
    pub should_fail_load: bool,
    pub should_fail_save: bool,
}

#[async_trait]
impl ConfigStorage<TherapyConfig> for MockTherapyConfigStorage {
    async fn load(&self) -> Result<TherapyConfig, StorageError> {
        if self.should_fail_load {
            Err(StorageError::NotFound)
        } else {
            self.config.clone().ok_or(StorageError::NotFound)
        }
    }

    async fn save(&self, _config: &TherapyConfig) -> Result<(), StorageError> {
        if self.should_fail_save {
            Err(StorageError::WriteError("forced error".into()))
        } else {
            Ok(())
        }
    }
}

/// Mock de AppConfigStorage para pruebas
pub struct MockAppConfigStorage {
    pub app_settings: Option<AppSettings>,
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

/// Mock de SystemNotifier para pruebas
#[derive(Debug, Default)]
pub struct MockSystemNotifier {
    pub last_title: Option<String>,
    pub last_message: Option<String>,
    pub last_state: Option<bool>,
    pub show_message_called: bool,
    pub set_tray_state_called: bool,
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
