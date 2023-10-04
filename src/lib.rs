use std::fmt::Debug;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color as SdlColor;
use std::time::Duration;

#[derive(Debug)]
pub struct App<Store = ()> {
    store: Store,
    windows: Vec<Window>,
}

impl App<()> {
    pub fn new() -> Self {
        Self {
            store: (),
            windows: Vec::new(),
        }
    }
}

impl<Store> App<Store> {
    pub fn with_store(store: Store) -> Self {
        Self {
            store,
            windows: Vec::new(),
        }
    }

    pub fn add_window(&mut self, window: Window) -> &mut Self {
        self.windows.push(window);

        self
    }

    pub fn run(self) -> std::io::Result<()> {
        let sdl_context = sdl2::init().unwrap();
        let video_subsystem = sdl_context.video().unwrap();

        let mut windows = Vec::new();

        for window in self.windows {
            let sdl_window = video_subsystem
                .window(
                    window.title.as_ref().unwrap_or(&String::new()),
                    window.size.0 as u32,
                    window.size.1 as u32,
                )
                .position_centered()
                .build()
                .unwrap();

            let canvas = sdl_window.into_canvas().build().unwrap();

            windows.push((window, canvas));
        }

        let mut event_pump = sdl_context.event_pump().unwrap();

        'running: loop {
            for event in event_pump.poll_iter() {
                match event {
                    Event::Quit { .. }
                    | Event::KeyDown {
                        keycode: Some(Keycode::Escape),
                        ..
                    } => {
                        break 'running;
                    }
                    _ => {}
                }
            }

            for (window, canvas) in &mut windows {
                canvas.set_draw_color(window.background.unwrap_or(Color::from((0, 0, 0))));
                canvas.clear();

                canvas.present();
            }
        }

        Ok(())
    }
}

#[derive(Debug)]
pub struct Window {
    size: (usize, usize),
    title: Option<String>,
    background: Option<Color>,
    root: Option<Box<dyn Component>>,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Bounds {
    x: usize,
    y: usize,
    width: usize,
    height: usize,
}

#[derive(Debug)]
pub struct RenderCtx {
    bounds: Bounds,
}

pub trait Component: Debug {
    fn render(&self, ctx: RenderCtx) -> std::io::Result<()>;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Color {
    r: u8,
    g: u8,
    b: u8,
    a: u8,
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
