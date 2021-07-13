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
    
    for c in s.chars() {
      let x = self.cursor_col * char_width;
      let y = self.cursor_row * char_height;

      if c == '\n' {
        self.newline();
      } else if self.cursor_col >= self.cols {
        write_ascii(self.pixel_writer, x, y, 0x02 as char, &PixelColor::new(0, 0, 0));
      } else {
        write_ascii(self.pixel_writer, x, y, c, &PixelColor::new(0, 0, 0));
        self.cursor_col += 1;
      }
    }
  }

  fn newline(&mut self) {
    self.cursor_col = 0;
    if self.cursor_row < self.rows - 1 {
      self.cursor_row += 1;
    } else {
      self.pixel_writer.clear_frame(&PixelColor::new(255, 255, 255));
    }
  }
}
