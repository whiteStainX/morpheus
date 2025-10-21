use crossterm::{cursor, execute, terminal};
use std::io::{stdout, Write};

pub fn clear_screen() {
    let mut out = stdout();
    execute!(out, terminal::Clear(terminal::ClearType::All), cursor::MoveTo(0,0)).unwrap();
}
