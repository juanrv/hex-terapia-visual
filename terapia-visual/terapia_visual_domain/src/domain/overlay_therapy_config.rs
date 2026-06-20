//! # Módulo de Configuración de Terapia
//!
//! Define el agregado principal [`OverlayTherapyConfig`], que representa la configuración
//! completa de una terapia visual.
//!
//! Una configuración incluye:
//! - El tipo de terapia ([`TherapyType`]).
//! - El layout de la pantalla ([`Layout`]).
//! - La lista de zonas configuradas ([`ZoneConfig`]), con sus colores y opacidades.
//!
//! Este módulo también proporciona métodos para cambiar el layout, actualizar
//! colores y opacidades, y generar las zonas reales para una resolución de pantalla.
//!
//! # Ejemplos
//!
//! ```
//! use terapia_visual_domain::domain::{OverlayTherapyConfig, TherapyType, Layout, ZoneConfig, Color, Opacity};
//!
//! // Crear una configuración vertical con dos zonas (rojo y verde)
//! let config = OverlayTherapyConfig::new(
//!     TherapyType::ColorDivision,
//!     Layout::Vertical,
//!     vec![
//!         ZoneConfig { color: Color::new("#FF0000").unwrap(), opacity: Opacity::new(0.8).unwrap() },
//!         ZoneConfig { color: Color::new("#00FF00").unwrap(), opacity: Opacity::new(0.6).unwrap() },
//!     ],
//! )
//! .unwrap();
//!
//! assert_eq!(config.layout(), Layout::Vertical);
//! assert_eq!(config.zones_config().len(), 2);
//!
//! // Generar las zonas para una pantalla de 1920x1080
//! let zones = config.generate_zones(1920, 1080);
//! assert_eq!(zones.len(), 2);
//! ```

use serde::{Deserialize, Serialize};

use crate::domain::{Color, Layout, Opacity, Zone};

/// Errores que pueden ocurrir al crear o modificar una configuración de terapia.
#[derive(Debug, thiserror::Error, PartialEq)]
pub enum ConfigError {
    /// El número de zonas no coincide con lo esperado por el layout seleccionado.
    ///
    /// Por ejemplo, un layout vertical requiere exactamente 2 zonas,
    /// pero se proporcionaron 3 o 1.
    #[error("Zone count mismatch: expected {expected}, got {got}")]
    ZoneCountMismatch {
        /// Número de zonas esperado para el layout.
        expected: usize,
        /// Número de zonas proporcionado.
        got: usize,
    },
    /// Se intentó acceder a un índice de zona que no existe.
    #[error("Zone index {0} out of bounds")]
    InvalidZoneIndex(usize),
}

/// Tipo de terapia disponible.
///
/// Actualmente solo se implementa la división de color, pero se pueden agregar
/// más tipos en el futuro sin modificar el resto de la aplicación.
///
/// # Extensibilidad
///
/// Este enum está diseñado para ser extensible. Para añadir un nuevo tipo de terapia:
/// 1. Añadir una nueva variante aquí.
/// 2. Modificar el método [`OverlayTherapyConfig::new`] para manejar la nueva variante.
/// 3. Implementar cualquier lógica específica del nuevo tipo.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum TherapyType {
    /// Terapia de división de color, donde la pantalla se divide en zonas
    /// con diferentes colores y opacidades.
    ColorDivision,
}

/// Configuración completa para una terapia visual.
///
/// Este es el agregado principal del dominio. Contiene toda la información
/// necesaria para configurar y ejecutar una terapia visual.
///
/// # Campos
///
/// * `therapy_type` - El tipo de terapia (actualmente solo `ColorDivision`).
/// * `layout` - El patrón de división de la pantalla (vertical, horizontal, etc.).
/// * `zones_config` - La configuración de cada zona (color y opacidad).
///
/// # Invariantes
///
/// - El número de zonas en `zones_config` debe coincidir con el número esperado
///   por el layout (ver [`Layout::zone_count`]).
/// - Todos los colores y opacidades son válidos (garantizado por los constructores).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct OverlayTherapyConfig {
    therapy_type: TherapyType,
    layout: Layout,
    zones_config: Vec<ZoneConfig>,
}

/// Configuración de una zona dentro de la terapia.
///
/// Define el color y la opacidad de una zona. La posición y el tamaño
/// de la zona son calculados por el layout.
///
/// # Campos
///
/// * `color` - El color que se muestra en la zona.
/// * `opacity` - La opacidad de la zona (0.0 a 0.8).
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ZoneConfig {
    pub color: Color,
    pub opacity: Opacity,
}

impl OverlayTherapyConfig {
    /// Crea una nueva configuración de terapia, validando que la cantidad de zonas
    /// coincida con lo esperado por el layout.
    ///
    /// # Argumentos
    ///
    /// * `therapy_type` - El tipo de terapia.
    /// * `layout` - El layout de la pantalla.
    /// * `zones_config` - La configuración de cada zona (color y opacidad).
    ///
    /// # Errores
    ///
    /// Devuelve [`ConfigError::ZoneCountMismatch`] si el número de zonas no coincide
    /// con lo esperado por el layout.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::{OverlayTherapyConfig, TherapyType, Layout, ZoneConfig, Color, Opacity};
    ///
    /// // Configuración válida
    /// let config = OverlayTherapyConfig::new(
    ///     TherapyType::ColorDivision,
    ///     Layout::Vertical,
    ///     vec![
    ///         ZoneConfig { color: Color::new("#FF0000").unwrap(), opacity: Opacity::new(0.8).unwrap() },
    ///         ZoneConfig { color: Color::new("#0000FF").unwrap(), opacity: Opacity::new(0.6).unwrap() },
    ///     ],
    /// );
    /// assert!(config.is_ok());
    ///
    /// // Configuración inválida (número incorrecto de zonas)
    /// let invalid = OverlayTherapyConfig::new(
    ///     TherapyType::ColorDivision,
    ///     Layout::Vertical,
    ///     vec![
    ///         ZoneConfig { color: Color::default(), opacity: Opacity::default() },
    ///     ],
    /// );
    /// assert!(invalid.is_err());
    /// ```
    pub fn new(
        therapy_type: TherapyType,
        layout: Layout,
        zones_config: Vec<ZoneConfig>,
    ) -> Result<Self, ConfigError> {
        match therapy_type {
            TherapyType::ColorDivision => {
                let expected_zones = layout.zone_count();
                let got_zones = zones_config.len();
                if zones_config.len() != expected_zones {
                    return Err(ConfigError::ZoneCountMismatch {
                        expected: expected_zones,
                        got: got_zones,
                    });
                }
                Ok(Self {
                    therapy_type,
                    layout,
                    zones_config,
                })
            }
        }
    }

    /// Cambia el layout actual, adaptando automáticamente la cantidad de zonas.
    ///
    /// Si el nuevo layout requiere más zonas, se clonan las zonas existentes
    /// de forma cíclica hasta alcanzar el número necesario.
    /// Si requiere menos, se truncan las zonas sobrantes.
    ///
    /// **Nota**: Este método también aplica las reglas de sincronización específicas
    /// de cada layout (por ejemplo, en `Checkerboard`, las zonas 0 y 3 se sincronizan,
    /// y las zonas 1 y 2 se sincronizan).
    ///
    /// # Argumentos
    ///
    /// * `new_layout` - El nuevo layout que se quiere aplicar.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::{OverlayTherapyConfig, Layout};
    ///
    /// let mut config = OverlayTherapyConfig::default(); // Layout::Vertical (2 zonas)
    ///
    /// // Cambiar a Checkerboard (4 zonas)
    /// config.change_layout(Layout::Checkerboard);
    /// assert_eq!(config.layout(), Layout::Checkerboard);
    /// assert_eq!(config.zones_config().len(), 4);
    /// ```
    pub fn change_layout(&mut self, new_layout: Layout) {
        let expected_zones = new_layout.zone_count();
        let current_zones = self.zones_config.len();

        if expected_zones > current_zones {
            let mut new_zones = self.zones_config.clone();
            while new_zones.len() < expected_zones {
                // Toma el indice de la zona que toca clonar (ciclicamente)
                let source_index = new_zones.len() % current_zones;
                new_zones.push(new_zones[source_index].clone());
            }
            self.zones_config = new_zones;
        } else if expected_zones < current_zones {
            self.zones_config.truncate(expected_zones);
        }

        self.layout = new_layout;

        // Aplicar reglas de sincronizacion espeecificas de cada layout.
        self.apply_sync_rules();
    }

    /// Aplica las reglas de sincronización de colores y opacidades según el layout actual.
    ///
    /// # Reglas implementadas
    ///
    /// - **Checkerboard**: Sincroniza colores y opacidades en pares diagonales:
    ///   - Zona 0 ↔ Zona 3
    ///   - Zona 1 ↔ Zona 2
    ///
    /// - **Vertical4Columns**: Sincroniza colores y opacidades en pares reflejados:
    ///   - Zona 0 ↔ Zona 2
    ///   - Zona 1 ↔ Zona 3
    fn apply_sync_rules(&mut self) {
        match (self.layout, self.zones_config.len()) {
            (Layout::Checkerboard, 4) => {
                self.zones_config[3] = self.zones_config[0].clone();
                self.zones_config[2] = self.zones_config[1].clone();
            }
            (Layout::Vertical4Columns, 4) => {
                self.zones_config[2] = self.zones_config[0].clone();
                self.zones_config[3] = self.zones_config[1].clone();
            }
            _ => {}
        }
    }

    /// Actualiza el color de una zona específica.
    ///
    /// Si el layout actual tiene reglas de sincronización (Checkerboard o Vertical4Columns),
    /// los colores se sincronizan automáticamente entre las zonas emparejadas.
    ///
    /// # Argumentos
    ///
    /// * `zone_index` - Índice de la zona a actualizar (base 0).
    /// * `new_color` - El nuevo color para la zona.
    ///
    /// # Errores
    ///
    /// Devuelve [`ConfigError::InvalidZoneIndex`] si el índice está fuera de rango.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::{OverlayTherapyConfig, Color};
    ///
    /// let mut config = OverlayTherapyConfig::default();
    /// let new_color = Color::new("#0000FF").unwrap();
    ///
    /// // Cambiar el color de la primera zona a azul
    /// config.update_zone_color(0, new_color).unwrap();
    /// assert_eq!(config.zones_config()[0].color.as_str(), "#0000FF");
    /// ```
    pub fn update_zone_color(
        &mut self,
        zone_index: usize,
        new_color: Color,
    ) -> Result<(), ConfigError> {
        if zone_index >= self.zones_config.len() {
            return Err(ConfigError::InvalidZoneIndex(zone_index));
        }

        self.zones_config[zone_index].color = new_color.clone();

        if let Some(pair) = self.get_sync_pair(zone_index) {
            self.zones_config[pair].color = new_color;
        }

        Ok(())
    }

    /// Actualiza la opacidad de una zona específica.
    ///
    /// Si el layout actual tiene reglas de sincronización (Checkerboard o Vertical4Columns),
    /// las opacidades se sincronizan automáticamente entre las zonas emparejadas.
    ///
    /// # Argumentos
    ///
    /// * `zone_index` - Índice de la zona a actualizar (base 0).
    /// * `new_opacity` - La nueva opacidad para la zona.
    ///
    /// # Errores
    ///
    /// Devuelve [`ConfigError::InvalidZoneIndex`] si el índice está fuera de rango.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::{OverlayTherapyConfig, Opacity};
    ///
    /// let mut config = OverlayTherapyConfig::default();
    /// let new_opacity = Opacity::new(0.3).unwrap();
    ///
    /// // Cambiar la opacidad de la primera zona a 0.3
    /// config.update_zone_opacity(0, new_opacity).unwrap();
    /// assert_eq!(config.zones_config()[0].opacity.value(), 0.3);
    /// ```
    pub fn update_zone_opacity(
        &mut self,
        zone_index: usize,
        new_opacity: Opacity,
    ) -> Result<(), ConfigError> {
        if zone_index >= self.zones_config.len() {
            return Err(ConfigError::InvalidZoneIndex(zone_index));
        }

        self.zones_config[zone_index].opacity = new_opacity;

        if let Some(pair) = self.get_sync_pair(zone_index) {
            self.zones_config[pair].opacity = new_opacity;
        }

        Ok(())
    }

    /// Obtiene el índice de la zona emparejada para sincronización.
    ///
    /// # Retorno
    ///
    /// * `Some(pair_index)` - Si el layout actual tiene reglas de sincronización.
    /// * `None` - Si el layout no tiene sincronización o el índice no está emparejado.
    fn get_sync_pair(&self, zone_index: usize) -> Option<usize> {
        match self.layout {
            Layout::Checkerboard => match zone_index {
                0 => Some(3),
                3 => Some(0),
                1 => Some(2),
                2 => Some(1),
                _ => None,
            },
            Layout::Vertical4Columns => match zone_index {
                0 => Some(2),
                2 => Some(0),
                1 => Some(3),
                3 => Some(1),
                _ => None,
            },
            _ => None,
        }
    }

    /// Devuelve el tipo de terapia.
    pub fn therapy_type(&self) -> TherapyType {
        self.therapy_type
    }

    /// Devuelve el layout de la terapia.
    pub fn layout(&self) -> Layout {
        self.layout
    }

    /// Devuelve la configuración de las zonas.
    pub fn zones_config(&self) -> &[ZoneConfig] {
        &self.zones_config
    }

    /// Genera las zonas reales (con rectángulos calculados) para una resolución de pantalla dada.
    ///
    /// # Argumentos
    ///
    /// * `screen_width` - Ancho de la pantalla en píxeles.
    /// * `screen_height` - Alto de la pantalla en píxeles.
    ///
    /// # Retorno
    ///
    /// Un vector de [`Zone`] con los rectángulos calculados según el layout actual
    /// y las configuraciones de color y opacidad.
    ///
    /// # Ejemplos
    ///
    /// ```
    /// use terapia_visual_domain::domain::OverlayTherapyConfig;
    ///
    /// let config = OverlayTherapyConfig::default(); // Layout::Vertical
    /// let zones = config.generate_zones(1920, 1080);
    ///
    /// assert_eq!(zones.len(), 2);
    /// assert_eq!(zones[0].rect().width, 960);
    /// assert_eq!(zones[1].rect().x, 960);
    /// ```
    pub fn generate_zones(&self, screen_width: u32, screen_height: u32) -> Vec<Zone> {
        let zone_rects = self.layout.calculate_zones(screen_width, screen_height);
        zone_rects
            .into_iter()
            .zip(self.zones_config.iter())
            .map(|(rect, config)| Zone::new(rect, config.color.clone(), config.opacity))
            .collect()
    }
}

/// Proporciona una configuración predeterminada para la terapia.
///
/// La configuración predeterminada es:
/// - Tipo: `ColorDivision`
/// - Layout: `Vertical`
/// - Dos zonas: roja (#FF0000) y verde (#00FF00), ambas con opacidad predeterminada (0.5)
///
/// # Ejemplos
///
/// ```
/// use terapia_visual_domain::domain::{OverlayTherapyConfig, Layout};
///
/// let config = OverlayTherapyConfig::default();
/// assert_eq!(config.layout(), Layout::Vertical);
/// assert_eq!(config.zones_config().len(), 2);
/// ```
impl Default for OverlayTherapyConfig {
    fn default() -> Self {
        let default_zones = vec![
            ZoneConfig {
                color: Color::new("#FF0000").unwrap(),
                opacity: Opacity::default(),
            },
            ZoneConfig {
                color: Color::new("#00FF00").unwrap(),
                opacity: Opacity::default(),
            },
        ];
        Self::new(TherapyType::ColorDivision, Layout::Vertical, default_zones)
            .expect("Default config")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn two_zones_config() -> Vec<ZoneConfig> {
        vec![
            ZoneConfig {
                color: Color::new("#FF0000").unwrap(),
                opacity: Opacity::new(0.8).unwrap(),
            },
            ZoneConfig {
                color: Color::new("#00FF00").unwrap(),
                opacity: Opacity::new(0.5).unwrap(),
            },
        ]
    }

    // ==========================================
    // CREACION Y GENERACION BASICA
    // ==========================================

    #[test]
    fn test_new_config_valid() {
        let config = OverlayTherapyConfig::new(
            TherapyType::ColorDivision,
            Layout::Vertical,
            two_zones_config(),
        );
        assert!(config.is_ok());
        let config = config.unwrap();
        assert_eq!(config.therapy_type(), TherapyType::ColorDivision);
        assert_eq!(config.layout(), Layout::Vertical);
        assert_eq!(config.zones_config().len(), 2);
    }

    #[test]
    fn test_new_config_invalid_zone_count() {
        let zones = vec![ZoneConfig {
            color: Color::default(),
            opacity: Opacity::default(),
        }];
        let result = OverlayTherapyConfig::new(TherapyType::ColorDivision, Layout::Vertical, zones);
        assert_eq!(
            result,
            Err(ConfigError::ZoneCountMismatch {
                expected: 2,
                got: 1
            })
        );
    }

    #[test]
    fn test_generate_zones() {
        let config = OverlayTherapyConfig::new(
            TherapyType::ColorDivision,
            Layout::Vertical,
            two_zones_config(),
        )
        .unwrap();
        let zones = config.generate_zones(1920, 1080);
        assert_eq!(zones.len(), 2);
        assert_eq!(zones[0].rect().x, 0);
        assert_eq!(zones[0].rect().width, 960);
        assert_eq!(zones[1].rect().x, 960);
        assert_eq!(zones[1].rect().width, 960);
        assert_eq!(zones[0].color().as_str(), "#FF0000");
        assert_eq!(zones[1].color().as_str(), "#00FF00");
    }

    // ==========================================
    // CAMBIOS DE LAYOUT
    // ==========================================

    #[test]
    fn test_change_layout_same_zone_count() {
        let mut config = OverlayTherapyConfig::default();
        config.change_layout(Layout::Horizontal);
        assert_eq!(config.layout(), Layout::Horizontal);
        assert_eq!(config.zones_config().len(), 2);
    }

    #[test]
    fn test_change_layout_truncates_when_reducing_zones() {
        let mut config = OverlayTherapyConfig::default();
        config.change_layout(Layout::Checkerboard); // Sube a 4
        config.change_layout(Layout::Horizontal); // Baja a 2
        assert_eq!(config.layout(), Layout::Horizontal);
        assert_eq!(config.zones_config().len(), 2);
    }

    #[test]
    fn test_change_layout_to_checkboard_expands_and_syncs() {
        let mut config = OverlayTherapyConfig::default();
        config.change_layout(Layout::Checkerboard);
        assert_eq!(config.layout(), Layout::Checkerboard);
        assert_eq!(config.zones_config().len(), 4);
        assert_eq!(
            config.zones_config()[0].color,
            config.zones_config()[3].color
        );
        assert_eq!(
            config.zones_config()[1].color,
            config.zones_config()[2].color
        );
    }

    #[test]
    fn test_change_layout_to_vertical4_expands_and_syncs() {
        let mut config = OverlayTherapyConfig::default();
        config.change_layout(Layout::Vertical4Columns);
        assert_eq!(config.layout(), Layout::Vertical4Columns);
        assert_eq!(config.zones_config().len(), 4);
        assert_eq!(
            config.zones_config()[0].color,
            config.zones_config()[2].color
        );
        assert_eq!(
            config.zones_config()[1].color,
            config.zones_config()[3].color
        );
    }

    // ==========================================
    // 3ACTUALIZACIONES BASICAS Y ERRORES
    // ==========================================

    #[test]
    fn test_update_zone_color_success() {
        let mut config = OverlayTherapyConfig::default();
        let new_color = Color::new("#0000FF").unwrap();
        assert!(config.update_zone_color(0, new_color.clone()).is_ok());
        assert_eq!(config.zones_config()[0].color, new_color);
    }

    #[test]
    fn test_update_zone_color_out_of_bounds() {
        let mut config = OverlayTherapyConfig::default();
        let new_color = Color::new("#0000FF").unwrap();
        assert_eq!(
            config.update_zone_color(99, new_color),
            Err(ConfigError::InvalidZoneIndex(99))
        );
    }

    #[test]
    fn test_update_zone_color_no_sync_for_vertical_layout() {
        let mut config = OverlayTherapyConfig::default(); // Inicia Vertical
        let new_color = Color::new("#FFFFFF").unwrap();
        let _ = config.update_zone_color(0, new_color.clone());
        assert_eq!(config.zones_config()[0].color, new_color);
        assert_ne!(config.zones_config()[1].color, new_color); // No debe sincronizar
    }

    #[test]
    fn test_update_zone_opacity_success() {
        let mut config = OverlayTherapyConfig::default();
        let new_opacity = Opacity::new(0.2).unwrap();
        assert!(config.update_zone_opacity(1, new_opacity).is_ok());
        assert_eq!(config.zones_config()[1].opacity, new_opacity);
    }

    #[test]
    fn test_update_zone_opacity_out_of_bounds() {
        let mut config = OverlayTherapyConfig::default();
        let new_opacity = Opacity::new(0.5).unwrap();
        // Intentamos actualizar una zona inexistente
        assert_eq!(
            config.update_zone_opacity(99, new_opacity),
            Err(ConfigError::InvalidZoneIndex(99))
        );
    }

    #[test]
    fn test_update_zone_opacity_no_sync_for_vertical_layout() {
        let mut config = OverlayTherapyConfig::default();
        let new_opacity = Opacity::new(0.3).unwrap();
        let _ = config.update_zone_opacity(0, new_opacity);
        assert_eq!(config.zones_config()[0].opacity, new_opacity);
        assert_ne!(config.zones_config()[1].opacity, new_opacity); // No debe sincronizar
    }

    // ==========================================
    // REGLAS DE SINCRONIZACIIN (AJEDREZ)
    // ==========================================

    #[test]
    fn test_checkerboard_syncs_color_updates() {
        let mut config = OverlayTherapyConfig::default();
        config.change_layout(Layout::Checkerboard);

        let color_a = Color::new("#111111").unwrap();
        let color_b = Color::new("#222222").unwrap();
        let color_c = Color::new("#333333").unwrap();
        let color_d = Color::new("#444444").unwrap();

        // Direcciones normales
        let _ = config.update_zone_color(0, color_a.clone());
        assert_eq!(config.zones_config()[3].color, color_a); // 0 actualiza 3

        let _ = config.update_zone_color(1, color_b.clone());
        assert_eq!(config.zones_config()[2].color, color_b); // 1 actualiza 2

        // Direcciones inversas
        let _ = config.update_zone_color(3, color_c.clone());
        assert_eq!(config.zones_config()[0].color, color_c); // 3 actualiza 0

        let _ = config.update_zone_color(2, color_d.clone());
        assert_eq!(config.zones_config()[1].color, color_d); // 2 actualiza 1
    }

    #[test]
    fn test_checkerboard_syncs_opacity_updates() {
        let mut config = OverlayTherapyConfig::default();
        config.change_layout(Layout::Checkerboard);

        let op_a = Opacity::new(0.1).unwrap();
        let op_b = Opacity::new(0.2).unwrap();
        let op_c = Opacity::new(0.3).unwrap();
        let op_d = Opacity::new(0.4).unwrap();

        // Direcciones normales
        let _ = config.update_zone_opacity(0, op_a);
        assert_eq!(config.zones_config()[3].opacity, op_a); // 0 actualiza 3

        let _ = config.update_zone_opacity(1, op_b);
        assert_eq!(config.zones_config()[2].opacity, op_b); // 1 actualiza 2

        // Direcciones inversas
        let _ = config.update_zone_opacity(3, op_c);
        assert_eq!(config.zones_config()[0].opacity, op_c); // 3 actualiza 0

        let _ = config.update_zone_opacity(2, op_d);
        assert_eq!(config.zones_config()[1].opacity, op_d); // 2 actualiza 1
    }

    // ==========================================
    // REGLAS DE SINCRONIZACION (4 COLUMNAS)
    // ==========================================

    #[test]
    fn test_vertical4_syncs_color_updates() {
        let mut config = OverlayTherapyConfig::default();
        config.change_layout(Layout::Vertical4Columns);

        let color_a = Color::new("#111111").unwrap();
        let color_b = Color::new("#222222").unwrap();
        let color_c = Color::new("#333333").unwrap();
        let color_d = Color::new("#444444").unwrap();

        // Direcciones normales
        let _ = config.update_zone_color(0, color_a.clone());
        assert_eq!(config.zones_config()[2].color, color_a); // 0 actualiza 2

        let _ = config.update_zone_color(1, color_b.clone());
        assert_eq!(config.zones_config()[3].color, color_b); // 1 actualiza 3

        // Direcciones inversas
        let _ = config.update_zone_color(2, color_c.clone());
        assert_eq!(config.zones_config()[0].color, color_c); // 2 actualiza 0

        let _ = config.update_zone_color(3, color_d.clone());
        assert_eq!(config.zones_config()[1].color, color_d); // 3 actualiza 1
    }

    #[test]
    fn test_vertical4_syncs_opacity_updates() {
        let mut config = OverlayTherapyConfig::default();
        config.change_layout(Layout::Vertical4Columns);

        let op_a = Opacity::new(0.1).unwrap();
        let op_b = Opacity::new(0.2).unwrap();
        let op_c = Opacity::new(0.3).unwrap();
        let op_d = Opacity::new(0.4).unwrap();

        // Direcciones normales
        let _ = config.update_zone_opacity(0, op_a);
        assert_eq!(config.zones_config()[2].opacity, op_a); // 0 actualiza 2

        let _ = config.update_zone_opacity(1, op_b);
        assert_eq!(config.zones_config()[3].opacity, op_b); // 1 actualiza 3

        // Direcciones inversas
        let _ = config.update_zone_opacity(2, op_c);
        assert_eq!(config.zones_config()[0].opacity, op_c); // 2 actualiza 0

        let _ = config.update_zone_opacity(3, op_d);
        assert_eq!(config.zones_config()[1].opacity, op_d); // 3 actualiza 1
    }
    // ==========================================
    // REGLAS DE SINCRONIZACIÓN - CASOS NONE
    // ==========================================

    #[test]
    fn test_get_sync_pair_returns_none_for_vertical_layout() {
        let config = OverlayTherapyConfig::default(); // Layout::Vertical
        // Verificar que para cualquier índice en Vertical, get_sync_pair retorna None
        for i in 0..config.zones_config().len() {
            assert_eq!(config.get_sync_pair(i), None);
        }
    }

    #[test]
    fn test_get_sync_pair_returns_none_for_horizontal_layout() {
        let mut config = OverlayTherapyConfig::default();
        config.change_layout(Layout::Horizontal);
        for i in 0..config.zones_config().len() {
            assert_eq!(config.get_sync_pair(i), None);
        }
    }

    #[test]
    fn test_get_sync_pair_returns_none_for_out_of_range_index() {
        let config = OverlayTherapyConfig::default();
        // Intentar con índices fuera del rango de zonas
        // (aunque el método no valida el índice, el match lo maneja con _)
        assert_eq!(config.get_sync_pair(99), None);
        assert_eq!(config.get_sync_pair(100), None);
    }

    #[test]
    fn test_get_sync_pair_returns_none_for_unpaired_index_in_checkerboard() {
        let mut config = OverlayTherapyConfig::default();
        config.change_layout(Layout::Checkerboard);
        // En Checkerboard, los índices válidos son 0,1,2,3
        // Los índices fuera de ese rango retornan None
        assert_eq!(config.get_sync_pair(4), None);
        assert_eq!(config.get_sync_pair(5), None);
    }

    #[test]
    fn test_get_sync_pair_returns_none_for_unpaired_index_in_vertical4() {
        let mut config = OverlayTherapyConfig::default();
        config.change_layout(Layout::Vertical4Columns);
        // En Vertical4Columns, los índices válidos son 0,1,2,3
        // Los índices fuera de ese rango retornan None
        assert_eq!(config.get_sync_pair(4), None);
        assert_eq!(config.get_sync_pair(5), None);
    }
}
