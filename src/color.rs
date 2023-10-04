use sdl2::pixels::Color as SdlColor;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub a: u8,
}

impl From<(u8, u8, u8)> for Color {
    fn from((r, g, b): (u8, u8, u8)) -> Self {
        Self { r, g, b, a: 255 }
    }
}

impl From<(u8, u8, u8, u8)> for Color {
    fn from((r, g, b, a): (u8, u8, u8, u8)) -> Self {
        Self { r, g, b, a }
    }
}

impl From<u32> for Color {
    fn from(input: u32) -> Self {
        let r = ((input >> 3 * 8) & 0xFF) as u8;
        let g = ((input >> 2 * 8) & 0xFF) as u8;
        let b = ((input >> 1 * 8) & 0xFF) as u8;
        let a = ((input >> 0 * 8) & 0xFF) as u8;
        Self { r, g, b, a }
    }
}

impl From<Color> for SdlColor {
    fn from(Color { r, g, b, a }: Color) -> Self {
        SdlColor::RGBA(r, g, b, a)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_color_from_impl() {
        let c = Color::from(0xFF0022EE);
        assert_eq!(c.r, 0xFF);
        assert_eq!(c.g, 0x00);
        assert_eq!(c.b, 0x22);
        assert_eq!(c.a, 0xEE);
    }
}
