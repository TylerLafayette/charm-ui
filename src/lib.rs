mod app;
pub use app::App;

mod window;
pub use window::Window;

pub mod layout;

pub mod component;
pub use component::Component;

mod color;
pub use color::Color;

pub type CharmResult<T> = std::io::Result<T>;
