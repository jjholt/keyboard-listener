use evdev::Device;
use evdev::KeyCode as Key;
use std::path::PathBuf;

use crate::error::Error;

/// Lightweight descriptor of a discovered keyboard device.
#[derive(Debug, Clone)]
pub struct DeviceInfo {
    /// Path to the device file, e.g. /dev/input/event3
    pub path: PathBuf,
    /// Human-readable name reported by the device
    pub name: String,
}

impl DeviceInfo {
    fn new(path: PathBuf, name: String) -> Self {
        Self { path, name }
    }
}

/// Returns all input devices that look like keyboards.
/// A device is considered a keyboard if it reports support for
/// standard letter keys (KEY_A through KEY_Z).
pub fn scan_keyboards() -> Result<Vec<DeviceInfo>, Error> {
    let devices = evdev::enumerate()
        .filter_map(|(path, device)| {
            if is_keyboard(&device) {
                let name = device.name().unwrap_or("Unknown Device").to_string();
                Some(DeviceInfo::new(path, name))
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    Ok(devices)
}

/// Heuristic: a device is a keyboard if it supports KEY_A..=KEY_Z.
fn is_keyboard(device: &Device) -> bool {
    let Some(keys) = device.supported_keys() else {
        return false;
    };
    // Check that a representative set of letter keys are all supported
    [Key::KEY_A, Key::KEY_S, Key::KEY_D, Key::KEY_ENTER]
        .iter()
        .all(|k| keys.contains(*k))
}

/// Presents the list of keyboards to the user on stdout and reads
/// their choice from stdin. Returns the selected `DeviceInfo`.
///
/// If exactly one keyboard is found it is selected automatically
/// without prompting.
pub trait SelectKeyboard<T> {
    fn select_keyboard(self) -> Result<T, Error>;
}

impl SelectKeyboard<DeviceInfo> for Vec<DeviceInfo> {
    fn select_keyboard(self) -> Result<DeviceInfo, Error> {
        if self.is_empty() {
            return Err(Error::NoKeyboardDevices);
        }

        if self.len() == 1 {
            println!("Using keyboard: {}", self[0].name);
            return Ok(self.into_iter().next().unwrap());
        }

        println!("\nMultiple keyboard self found. Please select one:\n");
        for (i, dev) in self.iter().enumerate() {
            println!("  [{}] {} ({})", i + 1, dev.name, dev.path.display());
        }
        println!();

        loop {
            print!("Enter number: ");

            // Flush stdout so the prompt appears before we block on stdin
            use std::io::Write;
            std::io::stdout().flush()?;

            let mut input = String::new();
            std::io::stdin().read_line(&mut input)?;

            match input.trim().parse::<usize>() {
                Ok(n) if n >= 1 && n <= self.len() => {
                    // Safe: bounds checked above
                    let selected = self.into_iter().nth(n - 1).unwrap();
                    println!("Selected: {}\n", selected.name);
                    return Ok(selected);
                }
                _ => println!(
                    "Invalid selection, please enter a number between 1 and {}.",
                    self.len()
                ),
            }
        }
    }
}
