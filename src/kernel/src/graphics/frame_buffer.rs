use core::clone::Clone;

pub enum PixelFormat {
  KPixelRGBReserved8BitPerColor,
  KPixelBGRReserved8BitPerColor,
}

pub struct FrameBufferConfig {
  pub frame_buffer: *mut u8,
  pub pixels_per_scan_line: u32,
  pub horizontal_resolution: u32,
  pub vertical_resolution: u32,
  pub pixel_format: PixelFormat,
}

impl Clone for FrameBufferConfig {
  fn clone(&self) -> Self {
    unsafe {
      FrameBufferConfig {
        frame_buffer: &mut (*self.frame_buffer),
        pixels_per_scan_line: self.pixels_per_scan_line,
        horizontal_resolution: self.horizontal_resolution,
        vertical_resolution: self.vertical_resolution,
        pixel_format: match self.pixel_format {
          PixelFormat::KPixelRGBReserved8BitPerColor => PixelFormat::KPixelRGBReserved8BitPerColor,
          PixelFormat::KPixelBGRReserved8BitPerColor => PixelFormat::KPixelBGRReserved8BitPerColor,
        },
      }
    }
  }
}
