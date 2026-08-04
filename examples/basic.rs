use keyboard_listener::prelude::*;
use serde::{Deserialize, Serialize};
use toml;

#[derive(PartialEq, InputEvent, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Action {
    Start,
    Pause
}

fn main() -> Result<(), Error> {
    // let keybinds = vec![
    //     Keybind::new(evdev::KeyCode::KEY_A, &[Modifier::Ctrl], Action::Start),
    //     Keybind::new(evdev::KeyCode::KEY_A, &[Modifier::Ctrl, Modifier::Shift], Action::Pause),
    // ];

    // let config = Config {
    //     keybinds,
    // };

    // println!("{}", toml::to_string_pretty(&config)?);

    let contents = std::fs::read_to_string("examples/keybinds.toml")?;
    let keybinds: Keybinds<Action> = toml::from_str(&contents)?;

    let device_info = scan_keyboards()?.select_keyboard()?;

    let (_handler, input_rx) = Listener::builder()
        .device(device_info)
        .binds(keybinds)
        .build().unwrap()
        .spawn();

    loop {
        match input_rx.recv() {
            Ok(action) => {
                println!("{:?}", action);
            },
            Err(err) => {
                eprintln!("Listener stopped: {err:?}");
                break Err(Error::ListenerKilled);
            }
        }
    }
}
