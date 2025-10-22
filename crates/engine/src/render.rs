use anyhow::Result;
use crossterm::{cursor, execute, style::Color, terminal};
use std::io::{stdout, Write};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Cell {
    pub symbol: char,
    pub fg: Color,
    pub bg: Color,
}

impl Default for Cell {
    fn default() -> Self {
        Self {
            symbol: ' ',
            fg: Color::Reset,
            bg: Color::Reset,
        }
    }
}

pub struct TerminalRenderer {
    width: u16,
    height: u16,
    front_buffer: Vec<Cell>,
    back_buffer: Vec<Cell>,
    stdout: std::io::Stdout,
}

impl TerminalRenderer {
    pub fn new(width: u16, height: u16) -> Result<Self> {
        let size = (width * height) as usize;
        Ok(Self {
            width,
            height,
            front_buffer: vec![Cell::default(); size],
            back_buffer: vec![Cell::default(); size],
            stdout: stdout(),
        })
    }

    pub fn init(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(self.stdout, terminal::EnterAlternateScreen, cursor::Hide)?;
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<()> {
        execute!(self.stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen(&mut self) {
        for cell in self.back_buffer.iter_mut() {
            *cell = Cell::default();
        }
    }

    pub fn flush(&mut self) -> Result<()> {
        // Placeholder for diff-based flushing logic
        Ok(())
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        self.shutdown().unwrap();
    }
}
