use crate::{
    component::{IntoParameter, Parameter, RenderCtx, Renderer},
    layout::{Bounds, Padding, Size},
    CharmResult, Color, Component,
};

pub struct VStack<Store> {
    children: Vec<Box<dyn Component<Store>>>,
    size: Option<Parameter<Size, Store>>,
    padding: Option<Parameter<Padding, Store>>,
    background: Option<Parameter<Color, Store>>,
}

impl<Store> VStack<Store> {
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
        C: Component<Store> + 'static,
    {
        self.children.push(Box::new(component));

        self
    }

    pub fn size(mut self, size: Parameter<Size, Store>) -> Self {
        self.size = Some(size);

        self
    }

    pub fn padding(mut self, padding: Parameter<Padding, Store>) -> Self {
        self.padding = Some(padding);

        self
    }

    pub fn background(mut self, fill: Parameter<Color, Store>) -> Self {
        self.background = Some(fill);

        self
    }
}

impl<Store> Component<Store> for VStack<Store> {
    fn render<'a>(&self, ctx: RenderCtx, r: &mut Renderer, store: &Store) -> CharmResult<Bounds> {
        let Bounds { x, y, .. } = ctx.bounds.clone();
        let Size { width, height } = self
            .size
            .as_ref()
            .map(|s| s.resolve(store))
            .unwrap_or(ctx.bounds.into());

        if let Some(ref background_fill) = self.background {
            r.fill_rect((x, y, width, height), background_fill.resolve(&store))?;
        }

        let mut child_ctx = ctx.clone();
        child_ctx.bounds.width = width;
        child_ctx.bounds.height = height;

        if let Some(ref padding) = self.padding {
            let padding = padding.resolve(&store);
            child_ctx.bounds.x += padding.left;
            child_ctx.bounds.y += padding.top;
            child_ctx.bounds.width -= padding.left + padding.right;
            child_ctx.bounds.height -= padding.top + padding.bottom;
        }

        for child in &self.children {
            let bounds = child.render(child_ctx.clone(), r, &store)?;

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
