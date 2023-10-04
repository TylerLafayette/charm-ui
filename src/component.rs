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

pub trait Component<Store> {
    fn render<'a>(
        &self,
        ctx: RenderCtx,
        renderer: &mut Renderer<'a>,
        store: &Store,
    ) -> CharmResult<Bounds>;
}

pub struct Parameter<T, Store> {
    thunk: Box<dyn Fn(&Store) -> T>,
}

impl<T, Store> Parameter<T, Store> {
    pub fn constant(constant: impl Into<T>) -> Self
    where
        T: 'static + Clone,
    {
        let constant = constant.into();
        let thunk = move |_: &Store| constant.clone();

        Self {
            thunk: Box::new(thunk),
        }
    }

    pub fn closure<O>(thunk: impl Fn(&Store) -> O + 'static) -> Self
    where
        O: Into<T>,
    {
        Self {
            thunk: Box::new(move |store| thunk(store).into()),
        }
    }

    pub(crate) fn resolve(&self, store: &Store) -> T {
        (self.thunk)(store)
    }
}

pub trait IntoParameter<P, Store> {
    fn into_parameter(self) -> Parameter<P, Store>;
}

impl<P, Store> IntoParameter<P, Store> for P
where
    P: Clone + 'static,
{
    fn into_parameter(self) -> Parameter<P, Store> {
        Parameter::constant(self)
    }
}

impl<P, Store> IntoParameter<P, Store> for Parameter<P, Store> {
    fn into_parameter(self) -> Parameter<P, Store> {
        self
    }
}
