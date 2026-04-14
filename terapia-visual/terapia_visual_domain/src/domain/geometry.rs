/// Representa un rectangulo que ocupa una zona en la pantalla.
/// Las coordenadas se representan como (x, y, width, height).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ZoneRect {
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
}

impl ZoneRect {
    pub fn new(x: u32, y: u32, width: u32, height: u32) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
