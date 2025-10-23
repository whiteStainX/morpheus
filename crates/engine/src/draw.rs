use crate::render::Cell;
use crossterm::style::Color;
use serde::Deserialize;

#[derive(Debug, Clone, Copy, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum PixelMode {
    Ascii,
    #[serde(alias = "halfblock")]
    HalfBlock,
    Braille,
}

impl Default for PixelMode {
    fn default() -> Self {
        PixelMode::Ascii
    }
}

impl PixelMode {
    pub fn next(self) -> Self {
        match self {
            PixelMode::Ascii => PixelMode::HalfBlock,
            PixelMode::HalfBlock => PixelMode::Braille,
            PixelMode::Braille => PixelMode::Ascii,
        }
    }
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

    pub fn draw_point(&mut self, x: u16, y: u16) {
        if x < self.width && y < self.height {
            let index = (y * self.width + x) as usize;
            self.buffer[index] = Cell {
                symbol: self.current_symbol,
                fg: self.current_fg,
                bg: self.current_bg,
            };
        }
    }

    pub fn draw_line(&mut self, x0: i32, y0: i32, x1: i32, y1: i32) {
        let mut x0 = x0;
        let mut y0 = y0;
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy; // error value e_xy

        loop {
            self.draw_point(x0 as u16, y0 as u16);
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                // e_xy + e_x > 0
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                // e_xy + e_y < 0
                err += dx;
                y0 += sy;
            }
        }
    }

    pub fn draw_rect(&mut self, x: u16, y: u16, w: u16, h: u16, filled: bool) {
        if filled {
            for current_y in y..(y + h) {
                for current_x in x..(x + w) {
                    self.draw_point(current_x, current_y);
                }
            }
        } else {
            // Top line
            self.draw_line(x as i32, y as i32, (x + w - 1) as i32, y as i32);
            // Bottom line
            self.draw_line(
                x as i32,
                (y + h - 1) as i32,
                (x + w - 1) as i32,
                (y + h - 1) as i32,
            );
            // Left line
            self.draw_line(x as i32, y as i32, x as i32, (y + h - 1) as i32);
            // Right line
            self.draw_line(
                (x + w - 1) as i32,
                y as i32,
                (x + w - 1) as i32,
                (y + h - 1) as i32,
            );
        }
    }

    pub fn draw_circle(&mut self, cx: i32, cy: i32, r: i32, filled: bool) {
        if filled {
            // Filled circle (draw horizontal lines)
            let mut x = r;
            let mut y = 0;
            let mut err = 0;

            while x >= y {
                self.draw_line(cx - x, cy + y, cx + x, cy + y);
                self.draw_line(cx - y, cy + x, cx + y, cy + x);
                self.draw_line(cx - x, cy - y, cx + x, cy - y);
                self.draw_line(cx - y, cy - x, cx + y, cy - x);

                y += 1;
                err += 1 + 2 * y;
                if 2 * (err - x) + 1 > 0 {
                    x -= 1;
                    err += 1 - 2 * x;
                }
            }
        } else {
            // Midpoint Circle Algorithm (outline)
            let mut x = r;
            let mut y = 0;
            let mut err = 0;

            while x >= y {
                self.draw_point((cx + x) as u16, (cy + y) as u16);
                self.draw_point((cx + y) as u16, (cy + x) as u16);
                self.draw_point((cx - y) as u16, (cy + x) as u16);
                self.draw_point((cx - x) as u16, (cy + y) as u16);
                self.draw_point((cx - x) as u16, (cy - y) as u16);
                self.draw_point((cx - y) as u16, (cy - x) as u16);
                self.draw_point((cx + y) as u16, (cy - x) as u16);
                self.draw_point((cx + x) as u16, (cy - y) as u16);

                y += 1;
                err += 1 + 2 * y;
                if 2 * (err - x) + 1 > 0 {
                    x -= 1;
                    err += 1 - 2 * x;
                }
            }
        }
    }

    pub fn set_symbol(&mut self, symbol: char) {
        self.current_symbol = symbol;
    }

    pub fn set_foreground_color(&mut self, color: Color) {
        self.current_fg = color;
    }

    pub fn set_background_color(&mut self, color: Color) {
        self.current_bg = color;
    }
}
