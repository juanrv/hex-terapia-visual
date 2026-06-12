use std::sync::atomic::AtomicBool;

use terapia_visual_adapter::config_storage::TomlStorage;
use terapia_visual_adapter::notifier::TauriSystemNotifier;
use terapia_visual_adapter::overlay::TauriOverlay;
use terapia_visual_domain::domain::TherapyConfig;
use tokio::sync::{Mutex, RwLock};

pub struct AppState {
    pub therapy_storage: TomlStorage,
    pub app_storage: TomlStorage,
    pub overlay: Mutex<TauriOverlay>,
    pub notifier: TauriSystemNotifier,
    pub current_config: RwLock<TherapyConfig>,
    pub is_toggling: AtomicBool,
}
