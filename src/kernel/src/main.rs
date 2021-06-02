#![no_std]
#![no_main]
#![feature(asm)]

mod frame_buffer;
mod pixel_color;
mod pixel_writer;

use core::panic::PanicInfo;
use frame_buffer::*;
use pixel_color::*;
use pixel_writer::*;

fn halt() -> ! {
  loop {
    unsafe {
      asm!("hlt");
    }
  }
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
  halt()
}

fn clear_frame(frame_buffer_config: &mut FrameBufferConfig, pixel_writer: &dyn PixelWriter) {
  for x in 0..frame_buffer_config.horizontal_resolution {
    for y in 0..frame_buffer_config.vertical_resolution {
      let color = PixelColor::new(255, 255, 255);
      (*pixel_writer).write(frame_buffer_config, x, y, &color);
    }
  }
}

#[no_mangle]
pub extern "C" fn KernelMain(frame_buffer_config: &mut FrameBufferConfig) {
  let pixel_writer: &dyn PixelWriter;

  match &frame_buffer_config.pixel_format {
    PixelFormat::KPixelRGBReserved8BitPerColor => {
      pixel_writer = &RGBReserved8BitPerColorPixelWriter {}
    }
    PixelFormat::KPixelBGRReserved8BitPerColor => {
      pixel_writer = &BGRReserved8BitPerColorPixelWriter {}
    }
  }

  clear_frame(frame_buffer_config, pixel_writer);

  let w = 300;
  let h = 200;
  for x in 0..w {
    for y in 0..h {
      let color = PixelColor::new((255 * x / w) as u8, 180, (255 * y / h) as u8);
      (*pixel_writer).write(frame_buffer_config, 100 + x, 100 + y, &color);
    }
  }

  halt();
}
