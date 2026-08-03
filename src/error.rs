use thiserror::Error;

#[derive(Debug, Error)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("No keyboard devices found")]
    NoKeyboardDevices,

    #[error("Invalid device selection")]
    InvalidDeviceSelection,

    #[error("Input error: {0}")]
    Input(String),

    #[error("No keybinds set to listen")]
    NoKeybindsSet,

    #[error("Listener killed")]
    ListenerKilled,
}
