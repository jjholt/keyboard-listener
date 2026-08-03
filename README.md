# Keyboard Listener
Exposes the `Listener` struct and an `InputAction` trait that can be derived, which indicates that enum describes all the actions your software might want to take, e.g.,

```rust
#[derive(InputAction)]
enum Action {
    Play,
    Next,
    Pause,
    None,
}
```

which allows us to create keybinds
```rust
let keybinds = vec![
    Keybind::new(evdev::KeyCode::KEY_A, &[Modifier::Ctrl],                  Action::Start),
    Keybind::new(evdev::KeyCode::KEY_A, &[Modifier::Ctrl, Modifier::Shift], Action::Pause),
];
```

and then build a listener for these bindings. We determine the keyboard using `scan_keyboards()` and `select_keyboard`:

```rust
    let device_info = scan_keyboards()?.select_keyboard()?;

    let (_handler, input_rx) = Listener::builder()
        .device(device_info)
        .binds(keybinds)
        .build().unwrap()
        .spawn();
```

And finally, we can run a loop using `input_rx` that tells us the `Action` received:
```rust
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
```
