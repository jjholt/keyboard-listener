use keyboard_listener::{InputEvent, Keybind, Listener, Modifier, SelectKeyboard, error, scan_keyboards};

#[derive(InputEvent, Clone, Debug)]
enum Actions {
    Start,
    Pause
}

fn main() -> Result<(), error::Error> {
    let keybinds = vec![
        Keybind::new(evdev::KeyCode::KEY_A, &[Modifier::Ctrl], Actions::Start),
        Keybind::new(evdev::KeyCode::KEY_A, &[Modifier::Ctrl, Modifier::Shift], Actions::Pause),
    ];

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
                eprintln!("Listener stopped: {err}");
                break Err(error::Error::ListenerKilled);
            }
        }
    }
}
