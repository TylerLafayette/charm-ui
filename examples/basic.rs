// use charm_ui::charms::{Button, Text, VStack};
use charm_ui::{App, Window};

pub struct Store {
    count: usize,
}

pub fn main() {
    let store = Store { count: 0 };
    let mut app = App::with_store(store);
    let window = Window::with_size((1200, 600))
        .title("My first window")
        .background((255, 255, 127));

    // window.set_root_component(
    //     VStack::new()
    //         .add_child(
    //             Text::new(|store| format!("clicked {} times", store.count)).fill((255, 255, 255)),
    //         )
    //         .add_child(Button::new("+ 1").on_click(|mut store| {
    //             store.count += 1;
    //         }))
    //         .size(400, 300)
    //         .background((255, 0, 0)),
    // );

    app.add_window(window);
    app.run().unwrap();
}
