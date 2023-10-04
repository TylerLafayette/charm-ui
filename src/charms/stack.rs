use crate::{
    component::{RenderCtx, Renderer},
    layout::{Bounds, Padding, Size},
    CharmResult, Color, Component,
};

#[derive(Debug)]
pub struct VStack {
    children: Vec<Box<dyn Component>>,
    size: Option<Size>,
    padding: Option<Padding>,
    background: Option<Color>,
}

impl VStack {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            size: None,
            padding: None,
            background: None,
        }
    }

    pub fn add_child<C>(mut self, component: C) -> Self
    where
        C: Component + 'static,
    {
        self.children.push(Box::new(component));

        self
    }

    pub fn size(mut self, size: impl Into<Size>) -> Self {
        self.size = Some(size.into());

        self
    }

    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = Some(padding.into());

        self
    }

    pub fn background(mut self, fill: impl Into<Color>) -> Self {
        self.background = Some(fill.into());

        self
    }
}

impl Component for VStack {
    fn render<'a>(&self, ctx: RenderCtx, r: &mut Renderer) -> CharmResult<Bounds> {
        let Bounds { x, y, .. } = ctx.bounds.clone();
        let Size { width, height } = self.size.clone().unwrap_or(ctx.bounds.into());

        if let Some(background_fill) = self.background {
            r.fill_rect((x, y, width, height), background_fill)?;
        }

        let mut child_ctx = ctx.clone();
        child_ctx.bounds.width = width;
        child_ctx.bounds.height = height;

        if let Some(padding) = self.padding {
            child_ctx.bounds.x += padding.left;
            child_ctx.bounds.y += padding.top;
            child_ctx.bounds.width -= padding.left + padding.right;
            child_ctx.bounds.height -= padding.top + padding.bottom;
        }

        for child in &self.children {
            let bounds = child.render(child_ctx.clone(), r)?;

            child_ctx.bounds.y += bounds.height;
            child_ctx.bounds.height -= bounds.height;
        }

        Ok(Bounds {
            x,
            y,
            width,
            height,
        })
    }
}
