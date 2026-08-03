use std::sync::mpsc;
use std::thread;

use evdev::KeyCode;
use evdev::{Device, EventSummary};

use crate::InputEvent;
use crate::device::DeviceInfo;
use crate::error::Error;
use crate::keys::{IntoModifier, Keybind, ModifierState};


/// Owns the evdev device handle and translates raw key events into
/// `InputEvent`s, which are sent over a std mpsc channel.
pub struct Listener<T: InputEvent> {
    device: Device,
    keybinds: Vec<Keybind<T>>,
}

impl<T: InputEvent + Send + 'static + Clone> Listener<T> {
    pub fn builder() -> ListenerBuilder<T> {
        ListenerBuilder::new()
    }

    /// Spawns the listener on a background thread.
    /// Returns the `JoinHandle` and the receiving end of the channel.
    /// The thread runs until the receiver is dropped or an error occurs.
    pub fn spawn(self) -> (thread::JoinHandle<Result<(), Error>>, mpsc::Receiver<T>) {
        let (tx, rx) = mpsc::channel::<T>();
        let handle = thread::spawn(move || self.run(tx));
        (handle, rx)
    }

    /// Blocking event loop. Runs on its own thread.
    /// Sends `InputEvent`s over `tx` whenever a configured keybind fires.
    fn run(mut self, tx: mpsc::Sender<T>) -> Result<(), Error> {
        let mut held_modifiers = ModifierState::empty();
        loop {
            let events = self
                .device
                .fetch_events()
                .map_err(|e| Error::Input(e.to_string()))?;
            for event in events {
                match event.destructure() {
                    EventSummary::Key(_, key, value) => {
                        if let Some(modifier) = key.into_modifier() {
                            match value {
                                1 => {
                                    held_modifiers |= ModifierState::from(modifier);
                                }
                                0 => {
                                    held_modifiers.remove(modifier.into());
                                }
                                _ => {}
                            }
                            continue;
                        }

                        if value != 1 {
                            continue;
                        }

                        if let Some(input_event) =
                            Self::translate(key, held_modifiers, &self.keybinds)
                        {
                            if tx.send(input_event).is_err() {
                                return Ok(());
                            }
                        }
                    }
                    _ => {}
                }
            }
        }
    }
    /// Translates a raw key + modifier state into an `InputEvent`,
    /// returning `None` if the combination doesn't match any bind.
    fn translate<'a>(key: KeyCode, held: ModifierState, keybinds: &'a [Keybind<T>]) -> Option<T> {
        keybinds
            .iter()
            .find(|&bind| bind.key == key && bind.modifiers == held)
            .map(|b| b.action.clone())
    }
}

#[derive(Debug, Default)]
pub struct ListenerBuilder<T: InputEvent> {
    device_info: Option<DeviceInfo>,
    keybinds: Vec<Keybind<T>>,
}

impl <T: InputEvent> ListenerBuilder<T> {
    pub fn new() -> Self {
        Self {
            device_info: None,
            keybinds: Vec::new(),
        }
    }

    pub fn device(mut self, device_info: DeviceInfo) -> Self {
        self.device_info = Some(device_info);
        self
    }

    pub fn bind(mut self, keybind: Keybind<T>) -> Self {
        self.keybinds.push(keybind);
        self
    }
    pub fn binds(mut self, keybinds: impl IntoIterator<Item = Keybind<T>>) -> Self {
        self.keybinds.extend(keybinds);
        self
    }

    pub fn build(self) -> Result<Listener<T>, Error> {
        let device_info = self
            .device_info
            .ok_or_else(|| Error::Config("Listener requires a device".to_string()))?;
        let keybinds = self.keybinds;
        if keybinds.is_empty() {
            return Err(Error::NoKeybindsSet)
        }

        let device = Device::open(&device_info.path)?;

        Ok(Listener { device, keybinds})
    }
}
