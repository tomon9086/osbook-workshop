use crate::*;

pub trait PixelWriter {
  fn pixel_at(&self, config: &mut FrameBufferConfig, x: u32, y: u32) -> *mut u8 {
    unsafe {
      config
        .frame_buffer
        .add(4 * (config.pixels_per_scan_line * y + x) as usize)
    }
  }

  fn write(&self, config: &mut FrameBufferConfig, x: u32, y: u32, color: &PixelColor);
}

pub struct RGBReserved8BitPerColorPixelWriter {}

impl PixelWriter for RGBReserved8BitPerColorPixelWriter {
  fn write(&self, config: &mut FrameBufferConfig, x: u32, y: u32, color: &PixelColor) {
    unsafe {
      let p = self.pixel_at(config, x, y);
      *p.add(0) = color.r();
      *p.add(1) = color.g();
      *p.add(2) = color.b();
    }
  }
}

pub struct BGRReserved8BitPerColorPixelWriter {}

impl PixelWriter for BGRReserved8BitPerColorPixelWriter {
  fn write(&self, config: &mut FrameBufferConfig, x: u32, y: u32, color: &PixelColor) {
    unsafe {
      let p = self.pixel_at(config, x, y);
      *p.add(0) = color.b();
      *p.add(1) = color.g();
      *p.add(2) = color.r();
    }
  }
}
