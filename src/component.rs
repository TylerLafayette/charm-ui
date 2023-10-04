use std::fmt::Debug;

use super::layout::Bounds;
use super::{CharmResult, Color};

#[derive(Clone, Copy)]
pub struct RenderCtx {
    pub(crate) bounds: Bounds,
}

pub struct Renderer<'a> {
    pub(crate) canvas: &'a mut sdl2::render::WindowCanvas,
}

impl<'a> Renderer<'a> {
    pub fn fill_rect(
        &mut self,
        bounds: impl Into<Bounds>,
        color: impl Into<Color>,
    ) -> CharmResult<()> {
        let bounds = bounds.into();
        let color = color.into();

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
    fn render<'a>(&self, ctx: RenderCtx, renderer: &mut Renderer<'a>) -> CharmResult<Bounds>;
}
