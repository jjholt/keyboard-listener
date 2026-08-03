use keyboard_listener::{InputEvent, Keybind, Listener, Modifier, SelectKeyboard, error, scan_keyboards};
use serde::{Deserialize, Serialize};
use toml;

#[derive(InputEvent, Clone, Debug, Deserialize, Serialize)]
#[serde(rename_all = "lowercase")]
enum Action {
    Start,
    Pause
}

#[derive(Deserialize, Serialize)]
struct Config {
    keybinds: Vec<Keybind<Action>>
}

fn main() -> Result<(), error::Error> {
    // let keybinds = vec![
    //     Keybind::new(evdev::KeyCode::KEY_A, &[Modifier::Ctrl], Action::Start),
    //     Keybind::new(evdev::KeyCode::KEY_A, &[Modifier::Ctrl, Modifier::Shift], Action::Pause),
    // ];

    // let config = Config {
    //     keybinds,
    // };

    // println!("{}", toml::to_string_pretty(&config)?);

    let contents = std::fs::read_to_string("examples/keybinds.toml")?;
    let config: Config = toml::from_str(&contents)?;
    let keybinds = config.keybinds;

    let device_info = scan_keyboards().unwrap().select_keyboard().unwrap();

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
                break Err(error::Error::ListenerKilled);
            }
        }
    }
}
