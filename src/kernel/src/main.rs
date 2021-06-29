#![no_std]
#![no_main]
#![feature(asm)]

mod graphics;
mod font;
mod print;

use core::panic::PanicInfo;
use graphics::*;
use font::*;
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

  // ビミョい？
  let rgb_pw = RGBReserved8BitPerColorPixelWriter {
    config: &mut (*frame_buffer_config).clone()
  };
  let bgr_pw = BGRReserved8BitPerColorPixelWriter {
    config: &mut (*frame_buffer_config).clone()
  };

  match frame_buffer_config.pixel_format {
    PixelFormat::KPixelRGBReserved8BitPerColor => {
      pixel_writer = &rgb_pw;
    }
    PixelFormat::KPixelBGRReserved8BitPerColor => {
      pixel_writer = &bgr_pw;
    }
  }

  (*pixel_writer).clear_frame(&PixelColor::new(255, 255, 255));

  let w = 300;
  let h = 200;
  for y in 0..h {
    for x in 0..w {
      let color = PixelColor::new((255 * x / w) as u8, 180, (255 * y / h) as u8);
      (*pixel_writer).write(100 + x, 100 + y, &color);
    }
  }

  write_string(pixel_writer, 50, 30, "ABC abc ｱｲｳ ±²³ _!?", &PixelColor::new(0, 0, 0));

  for ascii in 0..=0xff {
    write_ascii(pixel_writer, 50 + (ascii * 8) % 300, 50 + 16 * ((ascii * 8) / 300) as u32, ascii as u8 as char, &PixelColor::new(0, 0, 0));
  }

  halt();
}
