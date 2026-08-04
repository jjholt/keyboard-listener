pub mod device;
pub mod listener;
pub mod error;
pub mod keys;

pub use keys::{Modifier, Keybind};
pub use listener::Listener;
pub use device::{scan_keyboards, SelectKeyboard};

pub use error::Error;

extern crate self as keyboard_listener;

pub trait InputEvent: Send + 'static + Clone + PartialEq + std::fmt::Debug {}

pub use keyboard_listener_derive::InputEvent;
