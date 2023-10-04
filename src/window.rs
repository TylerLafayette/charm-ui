use super::component::RenderCtx;
use super::layout::Bounds;
use super::{CharmResult, Color, Component};

#[derive(Debug)]
pub struct Window {
    pub(crate) size: (usize, usize),
    pub(crate) title: Option<String>,
    pub(crate) background: Option<Color>,
    pub(crate) root: Option<Box<dyn Component>>,
}

impl Window {
    pub fn with_size(size: (usize, usize)) -> Self {
        Self {
            size,
            title: None,
            background: None,
            root: None,
        }
    }

    pub fn background(mut self, fill: impl Into<Color>) -> Self {
        self.background = Some(fill.into());

        self
    }

    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = Some(title.into());

        self
    }

    pub fn set_root_component<C>(&mut self, component: C) -> &mut Self
    where
        C: Component + 'static,
    {
        self.root = Some(Box::new(component));

        self
    }

    fn render(&self, canvas: &mut sdl2::render::WindowCanvas) -> CharmResult<()> {
        let ctx = RenderCtx {
            bounds: Bounds {
                x: 0,
                y: 0,
                width: self.size.0,
                height: self.size.1,
            },
            canvas,
        };

        if let Some(ref root) = self.root {
            root.render(ctx)?;
        }

        Ok(())
    }
}
