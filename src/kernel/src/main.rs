#![no_std]
#![no_main]
#![feature(asm)]

mod console;
mod graphics;

use core::panic::PanicInfo;
use console::*;
use graphics::*;

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

  let mut console = Console::new(pixel_writer);

  (*pixel_writer).clear_frame(&PixelColor::new(255, 255, 255));

  let w = 300;
  let h = 200;
  for y in 0..h {
    for x in 0..w {
      let color = PixelColor::new((255 * x / w) as u8, 180, (255 * y / h) as u8);
      (*pixel_writer).write(100 + x, 100 + y, &color);
    }
  }

  console.put_string("ABC abc ｱｲｳ ±²³ _!?");

  halt();
}
