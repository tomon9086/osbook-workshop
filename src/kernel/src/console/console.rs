use crate::console::print::*;
use crate::graphics::{PixelWriter, PixelColor};

pub struct Console<'c> {
  pixel_writer: &'c dyn PixelWriter,
  cols: u32,
  rows: u32,
  cursor_col: u32,
  cursor_row: u32,
}

impl Console<'_> {
  pub fn new(pixel_writer: &dyn PixelWriter) -> Console {
    Console {
      pixel_writer: pixel_writer,
      cols: 80,
      rows: 25,
      cursor_col: 0,
      cursor_row: 0,
    }
  }

  pub fn put_string(&mut self, s: &str) {
    let char_width = 8;
    let char_height = 16;
    
    for (i, c) in s.chars().enumerate() {
      let x = self.cursor_col * char_width;
      let y = self.cursor_row * char_height;

      write_ascii(self.pixel_writer, x, y, c, &PixelColor::new(0, 0, 0));
      self.cursor_col += 1;
    }
  }
}
