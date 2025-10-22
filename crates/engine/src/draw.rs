use crate::render::Cell;
use crossterm::style::Color;

#[derive(Debug)]
pub enum PixelMode {
    Ascii,
    HalfBlock,
    Braille,
}

#[allow(dead_code)] // Fields will be used in future steps
pub struct Canvas<'a> {
    pub width: u16,
    pub height: u16,
    buffer: &'a mut Vec<Cell>,
    current_fg: Color,
    current_bg: Color,
    current_symbol: char,
    pub current_pixel_mode: PixelMode,
}

impl<'a> Canvas<'a> {
    pub fn new(width: u16, height: u16, buffer: &'a mut Vec<Cell>) -> Self {
        Self {
            width,
            height,
            buffer,
            current_fg: Color::Reset,
            current_bg: Color::Reset,
            current_symbol: ' ',
            current_pixel_mode: PixelMode::Ascii,
        }
    }

    pub fn draw_text(&mut self, x: u16, y: u16, text: &str) {
        if y >= self.height {
            return;
        }
        for (i, c) in text.chars().enumerate() {
            let x = x + i as u16;
            if x >= self.width {
                break;
            }
            let index = (y * self.width + x) as usize;
            self.buffer[index] = Cell {
                symbol: c,
                fg: self.current_fg,
                bg: self.current_bg,
            };
        }
    }

    // Future drawing primitives will go here
}
