use charm_ui::charms::VStack;
use charm_ui::{App, Window};

pub struct Store {
    count: usize,
}

pub fn main() {
    let store = Store { count: 0 };
    let mut app = App::with_store(store);
    let mut window = Window::with_size((1200, 600))
        .title("My first window")
        .background((255, 255, 127));

    window.set_root_component(
        VStack::new()
            .size((600, 400))
            .padding((64, 16))
            .background((255, 0, 0))
            .add_child(VStack::new().background((0, 255, 0))),
    );

    app.add_window(window);
    app.run().unwrap();
}
