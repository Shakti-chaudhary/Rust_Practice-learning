enum KeyPress {
    Down,
    Up,
}
struct KeyEvent {
    keycode: u16,
    state: KeyPress,
}

enum InputEvent {
    key(u16, KeyPress),
    Mouse,
}
impl From<KeyEvent> for InputEvent {
    fn from(ev: KeyEvent) -> Self {
        InputEvent::key(ev.keycode, ev.state)
    }
}

fn main() {
    let key_ev = KeyEvent {
        keycode: 5,
        state: KeyPress::Down,
    };
    // let input_ev = InputEvent::from(key_ev);
    let input_ev: InputEvent = key_ev.into();
}
