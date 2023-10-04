use super::{CharmResult, Color, Window};

use sdl2::event::Event;
use sdl2::keyboard::Keycode;

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

    pub fn run(self) -> CharmResult<()> {
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
