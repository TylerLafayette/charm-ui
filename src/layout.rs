#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bounds {
    pub x: usize,
    pub y: usize,
    pub width: usize,
    pub height: usize,
}

impl From<(usize, usize, usize, usize)> for Bounds {
    fn from((x, y, width, height): (usize, usize, usize, usize)) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }
}
