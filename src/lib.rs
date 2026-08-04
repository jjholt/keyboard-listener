pub mod device;
pub mod listener;
pub mod error;
pub mod keys;
pub mod prelude;

pub use keys::{Modifier, ModifierState, Keybind};

extern crate self as keyboard_listener;

pub trait InputEvent: Send + 'static + Clone + PartialEq + std::fmt::Debug {}
