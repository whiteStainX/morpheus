use crate::draw::Canvas;
use anyhow::Result;
use crossterm::{
    cursor,
    event::{KeyboardEnhancementFlags, PopKeyboardEnhancementFlags, PushKeyboardEnhancementFlags},
    execute,
    style::{Color, Print, SetBackgroundColor, SetForegroundColor},
    terminal,
};
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

#[allow(dead_code)] // Fields will be used in future steps
pub struct TerminalRenderer {
    width: u16,
    height: u16,
    front_buffer: Vec<Cell>,
    back_buffer: Vec<Cell>,
    overlay_buffer: Vec<Cell>,
    stdout: std::io::Stdout,
    keyboard_enhancement_enabled: bool,
}

impl TerminalRenderer {
    pub fn new(width: u16, height: u16) -> Result<Self> {
        let size = (width * height) as usize;
        Ok(Self {
            width,
            height,
            front_buffer: vec![Cell::default(); size],
            back_buffer: vec![Cell::default(); size],
            overlay_buffer: vec![Cell::default(); size],
            stdout: stdout(),
            keyboard_enhancement_enabled: false,
        })
    }

    pub fn init(&mut self) -> Result<()> {
        terminal::enable_raw_mode()?;
        execute!(self.stdout, terminal::EnterAlternateScreen, cursor::Hide)?;

        if matches!(terminal::supports_keyboard_enhancement(), Ok(true)) {
            execute!(
                self.stdout,
                PushKeyboardEnhancementFlags(KeyboardEnhancementFlags::REPORT_EVENT_TYPES)
            )?;
            self.keyboard_enhancement_enabled = true;
        }
        Ok(())
    }

    pub fn shutdown(&mut self) -> Result<()> {
        if self.keyboard_enhancement_enabled {
            execute!(self.stdout, PopKeyboardEnhancementFlags)?;
            self.keyboard_enhancement_enabled = false;
        }
        execute!(self.stdout, terminal::LeaveAlternateScreen, cursor::Show)?;
        terminal::disable_raw_mode()?;
        Ok(())
    }

    pub fn clear_screen(&mut self) {
        for cell in self.back_buffer.iter_mut() {
            *cell = Cell::default();
        }
        self.clear_overlay();
    }

    pub fn canvas(&mut self) -> Canvas<'_> {
        Canvas::new(self.width, self.height, &mut self.back_buffer)
    }

    pub fn overlay_canvas(&mut self) -> Canvas<'_> {
        Canvas::new(self.width, self.height, &mut self.overlay_buffer)
    }

    pub fn clear_overlay(&mut self) {
        for cell in self.overlay_buffer.iter_mut() {
            *cell = Cell::default();
        }
    }

    pub fn flush(&mut self) -> Result<()> {
        for (i, (front_cell, back_cell)) in self
            .front_buffer
            .iter()
            .zip(self.back_buffer.iter())
            .enumerate()
        {
            if front_cell != back_cell {
                let x = (i % self.width as usize) as u16;
                let y = (i / self.width as usize) as u16;
                execute!(
                    self.stdout,
                    cursor::MoveTo(x, y),
                    SetForegroundColor(back_cell.fg),
                    SetBackgroundColor(back_cell.bg),
                    Print(back_cell.symbol)
                )?;
            }
        }
        self.front_buffer.copy_from_slice(&self.back_buffer);

        for (i, overlay_cell) in self.overlay_buffer.iter().enumerate() {
            if overlay_cell.symbol != ' '
                || overlay_cell.fg != Color::Reset
                || overlay_cell.bg != Color::Reset
            {
                let x = (i % self.width as usize) as u16;
                let y = (i / self.width as usize) as u16;
                execute!(
                    self.stdout,
                    cursor::MoveTo(x, y),
                    SetForegroundColor(overlay_cell.fg),
                    SetBackgroundColor(overlay_cell.bg),
                    Print(overlay_cell.symbol)
                )?;
                self.front_buffer[i] = *overlay_cell;
            }
        }

        self.stdout.flush()?;
        self.clear_overlay();
        Ok(())
    }
}

impl Drop for TerminalRenderer {
    fn drop(&mut self) {
        self.shutdown().unwrap();
    }
}
