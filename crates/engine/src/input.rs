use std::{collections::HashSet, time::Duration};

use anyhow::Result;
use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};

/// Tracks the keyboard state for the current frame.
#[derive(Default, Debug)]
pub struct InputState {
    pressed: HashSet<KeyCode>,
}

impl InputState {
    pub fn new() -> Self {
        Self::default()
    }

    /// Polls the terminal for pending input events and updates the state.
    ///
    /// Returns the list of key events that were pressed during this poll so
    /// callers can react to discrete input actions while the pressed set keeps
    /// track of the current state of each key.
    pub fn poll_events(&mut self) -> Result<Vec<KeyEvent>> {
        let mut events = Vec::new();

        while event::poll(Duration::from_millis(0))? {
            match event::read()? {
                Event::Key(key_event) => match key_event.kind {
                    KeyEventKind::Press => {
                        self.set_key_pressed(key_event.code, true);
                        events.push(key_event);
                    }
                    KeyEventKind::Repeat => {
                        self.set_key_pressed(key_event.code, true);
                    }
                    KeyEventKind::Release => {
                        self.set_key_pressed(key_event.code, false);
                    }
                },
                _ => {}
            }
        }

        Ok(events)
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
