# charm-ui

Charm is a (WIP) lightweight immediate-mode GUI framework with a friendly API
designed to be familiar.

## Example

```rust
use charm_ui::{App, Color, Window};
use charm_ui::charms::VStack;
use charm_ui::component::Parameter;

pub struct Store {
    count: usize,
}

pub fn main() {
    let store = Store { count: 127 };
    let mut app = App::with_store(store);
    let mut window: Window<Store> = Window::with_size((1200, 600))
        .title("My first window")
        .background((255, 255, 127));

    window.set_root_component(
        VStack::new()
            .size(Parameter::constant((600, 400)))
            .padding(Parameter::constant((64, 16)))
            .background(Parameter::closure(|store: &Store| {
                (store.count as u8, 0, 0)
            }))
            .add_child(VStack::new().background(Parameter::constant((0, 255, 0)))),
    );

    app.add_window(window);
    app.run().unwrap();
}
```

## Backends

Charm currently only supports SDL2, but the graphics layer is abstracted so more
backends will be available shortly!
