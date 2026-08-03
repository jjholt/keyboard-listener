# Keyboard Listener
Exposes the `Listener` struct and an `InputAction` trait that can be derived, which indicates that enum describes all the actions your software might want to take, e.g.,

```rust
#[derive(InputAction)]
enum Actions {
    Play,
    Next,
    Pause,
    None,
}
```
