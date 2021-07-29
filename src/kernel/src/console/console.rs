use heapless::Vec;
use crate::console::print::*;
use crate::graphics::{PixelWriter, PixelColor};

const COLS: u32 = 80;
const ROWS: u32 = 25;
const CHAR_WIDTH: u32 = 8;
const CHAR_HEIGHT: u32 = 16;

type Buffer = Vec<Vec<char, {COLS as usize}>, {ROWS as usize}>;

pub struct Console<'c> {
  pixel_writer: &'c dyn PixelWriter,
  cursor_col: u32,
  cursor_row: u32,
  buffer: Buffer,
}

impl Console<'_> {
  pub fn new(pixel_writer: &dyn PixelWriter) -> Console {
    let mut buffer: Buffer = Vec::new();
    for _ in 0..ROWS {
      buffer.push(Vec::from_slice(&[0 as char; COLS as usize]).unwrap()).unwrap();
    }

    Console {
      pixel_writer: pixel_writer,
      cursor_col: 0,
      cursor_row: 0,
      buffer: buffer,
    }
  }
  
  fn set_buffer(&mut self, x: u32, y: u32, c: char) {
    if y < ROWS && x < COLS {
      self.buffer[y as usize][x as usize] = c;
    }
  }

  pub fn put_string(&mut self, s: &str) {
    for c in s.chars() {
      self.put_char(c);
    }
  }

  fn put_char(&mut self, c: char) {
    let x = self.cursor_col * CHAR_WIDTH;
    let y = self.cursor_row * CHAR_HEIGHT;

    if c == '\n' {
      self.newline();
      return;
    } else if c == 0 as char {
      let marker = ' ';
      write_ascii(self.pixel_writer, x, y, marker, &PixelColor::new(0, 0, 0));
      self.set_buffer(self.cursor_col, self.cursor_row, marker);
      self.cursor_col += 1;
    } else if self.cursor_col >= COLS {
      let marker = 0x02 as char;
      write_ascii(self.pixel_writer, x, y, marker, &PixelColor::new(0, 0, 0));
      self.set_buffer(self.cursor_col, self.cursor_row, marker);
    } else {
      write_ascii(self.pixel_writer, x, y, c, &PixelColor::new(0, 0, 0));
      self.set_buffer(self.cursor_col, self.cursor_row, c);
      self.cursor_col += 1;
    }
  }

  fn newline(&mut self) {
    self.cursor_col = 0;
    if self.cursor_row < ROWS - 1 {
      self.cursor_row += 1;
    } else {
      let mut b: Buffer = Vec::new();
      for y in 1..ROWS as usize {
        b.push(Vec::from_slice(&[0 as char; COLS as usize]).unwrap()).unwrap();
        for x in 0..COLS as usize {
          b[y - 1][x] = self.buffer[y][x];
        }
      }
      for y in 0..ROWS as usize {
        for x in 0..COLS as usize {
          self.buffer[y][x] = 0 as char;
        }
      }

      self.pixel_writer.clear_frame(&PixelColor::new(255, 255, 255));

      self.cursor_row = 0;
      for row in b.into_iter() {
        for c in row {
          self.put_char(c);
        }
        self.put_char('\n');
      }
    }
  }
}
