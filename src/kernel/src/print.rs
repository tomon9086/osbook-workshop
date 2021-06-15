use crate::*;

pub fn write_ascii(frame_buffer_config: &mut FrameBufferConfig, pixel_writer: &dyn PixelWriter, x: u32, y: u32, c: char, color: &PixelColor) {
  let w = 8;
  let h = 16;

  for px in 0..w {
    for py in 0..h {
      let font = get_font(c).get_pixel();
      if (font[py] << px) & 0x80 != 0 {
        (*pixel_writer).write(frame_buffer_config, x + px as u32, y + py as u32, color);
      }
    }
  }
}

pub fn write_string(frame_buffer_config: &mut FrameBufferConfig, pixel_writer: &dyn PixelWriter, x: u32, y: u32, s: &str, color: &PixelColor) {
  let char_width = 8;

  for (i, c) in s.chars().enumerate() {
    write_ascii(frame_buffer_config, pixel_writer, x + (i * char_width) as u32, y, c, color);
  }
}
