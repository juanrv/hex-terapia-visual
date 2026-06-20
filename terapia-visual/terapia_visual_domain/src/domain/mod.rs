//! # Módulo de Dominio
//!
//! Este módulo contiene las entidades, value objects y agregados
//! que representan la lógica de negocio de la terapia visual.
//!
//! Los componentes principales son:
//! - [`Color`]: Colores en formato hexadecimal (#RRGGBB)
//! - [`Opacity`]: Opacidad de las zonas (0.0 - 0.8)
//! - [`Layout`]: Patrones de división de la pantalla
//! - [`ZoneRect`]: Rectángulos que definen áreas en la pantalla
//! - [`Zone`]: Zona de color con posición, color y opacidad
//! - [`ZoneConfig`]: Configuración de una zona (color + opacidad)
//! - [`TherapyConfig`]: Configuración completa de la terapia
//! - [`AppSettings`]: Preferencias globales de la aplicación
//!
//! # Ejemplo de uso
//!
//! ```
//! use terapia_visual_domain::domain::{TherapyConfig, TherapyType, Layout, ZoneConfig, Color, Opacity};
//!
//! // Crear una configuración de terapia
//! let config = TherapyConfig::new(
//!     TherapyType::ColorDivision,
//!     Layout::Vertical,
//!     vec![
//!         ZoneConfig {
//!             color: Color::new("#FF0000").unwrap(),
//!             opacity: Opacity::new(0.8).unwrap(),
//!         },
//!         ZoneConfig {
//!             color: Color::new("#00FF00").unwrap(),
//!             opacity: Opacity::new(0.6).unwrap(),
//!         },
//!     ],
//! )
//! .unwrap();
//!
//! // Generar zonas para una pantalla de 1920x1080
//! let zones = config.generate_zones(1920, 1080);
//! assert_eq!(zones.len(), 2);
//! ```

pub mod app_settings;
pub mod color;
pub mod overlay_therapy_config;
pub mod geometry;
pub mod layout;
pub mod opacity;
pub mod zone;

pub use app_settings::{AppSettings, Language};
pub use color::Color;
pub use overlay_therapy_config::{OverlayTherapyConfig, ConfigError, TherapyType, ZoneConfig};
pub use geometry::ZoneRect;
pub use layout::Layout;
pub use opacity::Opacity;
pub use zone::Zone;
