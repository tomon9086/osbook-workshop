use crate::*;

fn pixel_at(config: &FrameBufferConfig, x: u32, y: u32) -> *mut u8 {
  unsafe {
    config
      .frame_buffer
      .add(4 * (config.pixels_per_scan_line * y + x) as usize)
  }
}

pub trait PixelWriter {
  fn pixel_at(&self, x: u32, y: u32) -> *mut u8;
  fn clear_frame(&self, color: &PixelColor);
  fn write(&self, x: u32, y: u32, color: &PixelColor);
}

pub struct RGBReserved8BitPerColorPixelWriter<'pw> {
  pub config: &'pw mut FrameBufferConfig
}

impl PixelWriter for RGBReserved8BitPerColorPixelWriter<'_> {
  fn pixel_at(&self, x: u32, y: u32) -> *mut u8 {
    pixel_at(self.config, x, y)
  }

  fn clear_frame(&self, color: &PixelColor) {
    unsafe {
      let r = color.r();
      let g = color.g();
      let b = color.b();
      let p_frame_buffer = self.config.frame_buffer as usize;

      let mut y = 0;
      while y < self.config.vertical_resolution {
        let mut x = 0;
        while x < self.config.horizontal_resolution {
          let p = p_frame_buffer + ((self.config.pixels_per_scan_line * y + x) << 2) as usize;

          *((p + 0) as *mut u8) = r;
          *((p + 1) as *mut u8) = g;
          *((p + 2) as *mut u8) = b;
          
          x += 1;
        }
        y += 1;
      }
    }
  }

  fn write(&self, x: u32, y: u32, color: &PixelColor) {
    unsafe {
      let p = self.pixel_at(x, y);
      *p.add(0) = color.r();
      *p.add(1) = color.g();
      *p.add(2) = color.b();
    }
  }
}

pub struct BGRReserved8BitPerColorPixelWriter<'pw> {
  pub config: &'pw mut FrameBufferConfig
}

impl PixelWriter for BGRReserved8BitPerColorPixelWriter<'_> {
  fn pixel_at(&self, x: u32, y: u32) -> *mut u8 {
    pixel_at(self.config, x, y)
  }

  fn clear_frame(&self, color: &PixelColor) {
    unsafe {
      let r = color.r();
      let g = color.g();
      let b = color.b();
      let p_frame_buffer = self.config.frame_buffer as usize;

      let mut y = 0;
      while y < self.config.vertical_resolution {
        let mut x = 0;
        while x < self.config.horizontal_resolution {
          let p = p_frame_buffer + ((self.config.pixels_per_scan_line * y + x) << 2) as usize;

          *((p + 0) as *mut u8) = b;
          *((p + 1) as *mut u8) = g;
          *((p + 2) as *mut u8) = r;

          x += 1;
        }
        y += 1;
      }
    }
  }

  fn write(&self, x: u32, y: u32, color: &PixelColor) {
    unsafe {
      let p = self.pixel_at(x, y);
      *p.add(0) = color.b();
      *p.add(1) = color.g();
      *p.add(2) = color.r();
    }
  }
}
