use crate::console::print::*;
use crate::graphics::{PixelWriter, PixelColor};

const COLS: u32 = 80;
const ROWS: u32 = 25;

// #[derive(Clone)]
// struct Buffer<'b>(pub [&'b mut [char; COLS as usize]; ROWS as usize]);

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
      cols: COLS,
      rows: ROWS,
      cursor_col: 0,
      cursor_row: 0,
    }
  }

  pub fn put_string(&mut self, s: &str) {
    for c in s.chars() {
      if c == '\n' {
        self.newline();
      } else if self.cursor_col >= self.cols {
        self.put_char(0x02 as char);
      } else {
        self.put_char(c);
        self.cursor_col += 1;
      }
    }
  }

  fn put_char(&mut self, c: char) {
    let char_width = 8;
    let char_height = 16;

    let x = self.cursor_col * char_width;
    let y = self.cursor_row * char_height;

    write_ascii(self.pixel_writer, x, y, c, &PixelColor::new(0, 0, 0));
    // self.buffer.0[y as usize][x as usize] = c;
  }

  fn newline(&mut self) {
    self.cursor_col = 0;
    if self.cursor_row < self.rows - 1 {
      self.cursor_row += 1;
    } else {
      // self.pixel_writer.clear_frame(&PixelColor::new(255, 255, 255));
      // let mut b = [&mut [0 as char; COLS as usize]; ROWS as usize];
      // for i in 1..(ROWS as usize) {
      //   b[i - 1] = self.buffer.0[i];
      //   for c in self.buffer.0[i] {
      //     self.put_char(*c);
      //   }
      // }
      // self.buffer.0 = b;
    }
  }
}
