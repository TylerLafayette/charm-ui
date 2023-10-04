use std::marker::PhantomData;

use crate::component::Renderer;

use super::component::RenderCtx;
use super::layout::Bounds;
use super::{CharmResult, Color, Component};

pub struct Window<Store = ()> {
    pub(crate) size: (usize, usize),
    pub(crate) title: Option<String>,
    pub(crate) background: Option<Color>,
    pub(crate) root: Option<Box<dyn Component<Store>>>,
}

impl<Store> Window<Store> {
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
        C: Component<Store> + 'static,
    {
        self.root = Some(Box::new(component));

        self
    }

    pub(crate) fn render(
        &self,
        canvas: &mut sdl2::render::WindowCanvas,
        store: &Store,
    ) -> CharmResult<()> {
        let ctx = RenderCtx {
            bounds: Bounds {
                x: 0,
                y: 0,
                width: self.size.0,
                height: self.size.1,
            },
        };

        if let Some(ref root) = self.root {
            let mut renderer = Renderer { canvas };
            root.render(ctx, &mut renderer, &store)?;
        }

        Ok(())
    }
}
