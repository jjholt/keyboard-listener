use std::fmt;

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

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq, Eq, Hash)]
pub struct Keybind<T: InputEvent> {
    #[serde(deserialize_with = "deserialise_keycode", serialize_with = "serialise_keycode")]
    pub key: KeyCode,
    pub modifiers: ModifierState,
    pub action: T,
}

impl <T: InputEvent> fmt::Display for Keybind<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}+{}", self.action, self.modifiers, self.key)
    }
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

impl fmt::Display for Modifier {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:?}", self)
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

impl From<ModifierState> for Vec<Modifier> {
    fn from(value: ModifierState) -> Self {
        [
            (ModifierState::CTRL, Modifier::Ctrl),
            (ModifierState::SHIFT, Modifier::Shift),
            (ModifierState::ALT, Modifier::Alt),
            (ModifierState::SUPER, Modifier::Super),
        ]
        .into_iter()
        .filter_map(|(flag, modifier)| value.contains(flag).then_some(modifier))
        .collect()
    }
}

impl fmt::Display for ModifierState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let modifiers: Vec<Modifier> = ModifierState.into();
        let text = modifiers.iter().map(ToString::to_string).collect::<Vec<_>>().join("+");
        f.write_str(&text)
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

impl<'de> Deserialize<'de> for ModifierState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let modifiers = Vec::<Modifier>::deserialize(deserializer)?;
        Ok(modifiers.as_slice().into())
    }
}

impl Serialize for ModifierState {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let modifiers: Vec<Modifier> = (*self).into();
        modifiers.serialize(serializer)
    }
}

fn deserialise_keycode<'de, D>(deserializer: D) -> Result<KeyCode, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let value = String::deserialize(deserializer)?;
    let keycode = if value.starts_with("KEY_") {
        value.parse::<KeyCode>()
    } else {
        format!("KEY_{}", value.to_uppercase()).parse::<KeyCode>()
    };
    keycode.map_err(|_| serde::de::Error::custom("Unknown key: {name}"))
}

fn serialise_keycode<S>(keycode: &KeyCode, serializer: S) -> Result<S::Ok, S::Error> where S: serde::Serializer {
    let name = format!("{:?}", keycode);
    let name = name.strip_prefix("KEY_").unwrap_or(&name);
    serializer.serialize_str(name)
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
