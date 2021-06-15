#![no_std]
#![no_main]
#![feature(asm)]

mod frame_buffer;
mod font;
mod pixel_color;
mod pixel_writer;
mod print;

use core::panic::PanicInfo;
use frame_buffer::*;
use font::*;
use pixel_color::*;
use pixel_writer::*;
use print::*;

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

  (*pixel_writer).clear_frame(frame_buffer_config, &PixelColor::new(255, 255, 255));

  let w = 300;
  let h = 200;
  for y in 0..h {
    for x in 0..w {
      let color = PixelColor::new((255 * x / w) as u8, 180, (255 * y / h) as u8);
      (*pixel_writer).write(frame_buffer_config, 100 + x, 100 + y, &color);
    }
  }

  write_string(frame_buffer_config, pixel_writer, 50, 30, "ABC abc ｱｲｳ ±²³ _!?", &PixelColor::new(0, 0, 0));

  for ascii in 0..=0xff {
    write_ascii(frame_buffer_config, pixel_writer, 50 + (ascii * 8) % 300, 50 + 16 * ((ascii * 8) / 300) as u32, ascii as u8 as char, &PixelColor::new(0, 0, 0));
  }

  halt();
}
