use evdev::KeyCode;
use serde::{Deserialize, Serialize};

use crate::InputEvent;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Modifier {
    Ctrl,
    Shift,
    Alt,
    Super,
}

#[derive(Debug)]
pub struct Keybind<T: InputEvent> {
    pub key: KeyCode,
    pub modifiers: ModifierState,
    pub action: T,
}

impl<T: InputEvent> Keybind<T> {
    pub fn new(key: KeyCode, modifiers: &[Modifier], action: T) -> Self {
        Self {
            key,
            modifiers: modifiers.into(),
            action,
        }
    }
}

bitflags::bitflags! {
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct ModifierState: u8 {
        const CTRL  = 1 << 0;
        const SHIFT = 1 << 1;
        const ALT   = 1 << 2;
        const SUPER = 1 << 3;
    }
}

impl From<Modifier> for ModifierState {
    fn from(value: Modifier) -> Self {
        match value {
            Modifier::Ctrl => ModifierState::CTRL,
            Modifier::Shift => ModifierState::SHIFT,
            Modifier::Alt => ModifierState::ALT,
            Modifier::Super => ModifierState::SUPER,
        }
    }
}

impl From<&[Modifier]> for ModifierState {
    fn from(value: &[Modifier]) -> Self {
        value
            .iter()
            .copied()
            .map(ModifierState::from)
            .fold(ModifierState::empty(), |state, modifier| state | modifier)
    }
}

/// Parses an evdev key name string (e.g. "KEY_ENTER") into a `Key`.
/// Returns `None` if the name is not recognised.
pub fn key_from_str(name: &str) -> Option<KeyCode> {
    name.parse::<KeyCode>().ok()
}

pub trait IntoModifier {
    fn into_modifier(self) -> Option<Modifier>;
}

impl IntoModifier for KeyCode {
    fn into_modifier(self) -> Option<Modifier> {
        match self {
            KeyCode::KEY_LEFTCTRL | KeyCode::KEY_RIGHTCTRL => Some(Modifier::Ctrl),
            KeyCode::KEY_LEFTSHIFT | KeyCode::KEY_RIGHTSHIFT => Some(Modifier::Shift),
            KeyCode::KEY_LEFTALT | KeyCode::KEY_RIGHTALT => Some(Modifier::Alt),
            KeyCode::KEY_LEFTMETA | KeyCode::KEY_RIGHTMETA => Some(Modifier::Super),
            _ => None,
        }
    }
}
