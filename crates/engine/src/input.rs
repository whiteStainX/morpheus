use std::collections::HashSet;

use crossterm::event::KeyCode;

/// Tracks the keyboard state for the current frame.
#[derive(Default, Debug)]
pub struct InputState {
    pressed: HashSet<KeyCode>,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn is_key_pressed(&self, key: KeyCode) -> bool {
        self.pressed.contains(&key)
    }

    pub fn set_key_pressed(&mut self, key: KeyCode, pressed: bool) {
        if pressed {
            self.pressed.insert(key);
        } else {
            self.pressed.remove(&key);
        }
    }

    pub fn clear(&mut self) {
        self.pressed.clear();
    }
}
