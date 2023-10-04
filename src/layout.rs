#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Size {
    pub width: usize,
    pub height: usize,
}

impl From<(usize, usize)> for Size {
    fn from((width, height): (usize, usize)) -> Self {
        Self { width, height }
    }
}

impl From<Bounds> for Size {
    fn from(Bounds { width, height, .. }: Bounds) -> Self {
        Self { width, height }
    }
}

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Padding {
    pub top: usize,
    pub right: usize,
    pub bottom: usize,
    pub left: usize,
}

impl From<usize> for Padding {
    fn from(padding: usize) -> Self {
        Self {
            top: padding,
            right: padding,
            bottom: padding,
            left: padding,
        }
    }
}

impl From<(usize, usize)> for Padding {
    fn from((vert, horiz): (usize, usize)) -> Self {
        Self {
            top: vert,
            right: horiz,
            bottom: vert,
            left: horiz,
        }
    }
}

impl From<(usize, usize, usize, usize)> for Padding {
    fn from((top, right, bottom, left): (usize, usize, usize, usize)) -> Self {
        Self {
            top,
            right,
            bottom,
            left,
        }
    }
}
