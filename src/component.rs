use std::fmt::Debug;

use super::layout::Bounds;
use super::{CharmResult, Color};

pub struct RenderCtx<'a> {
    pub(crate) bounds: Bounds,
    pub(crate) canvas: &'a mut sdl2::render::WindowCanvas,
}

impl<'a> RenderCtx<'a> {
    pub fn fill_rect(&mut self, bounds: Bounds, color: Color) -> CharmResult<()> {
        self.canvas.set_draw_color(color);
        self.canvas
            .fill_rect(sdl2::rect::Rect::new(
                bounds.x as i32,
                bounds.y as i32,
                bounds.width as u32,
                bounds.height as u32,
            ))
            .unwrap();

        Ok(())
    }
}

pub trait Component: Debug {
    fn render<'a>(&self, ctx: RenderCtx<'a>) -> CharmResult<()>;
}
