#![no_std]
#![no_main]
#![feature(asm)]

mod frame_buffer;

use frame_buffer::*;
use panic_halt;

fn halt() -> ! {
  loop {
    unsafe {
      asm!("hlt");
    }
  }
}

#[no_mangle]
pub extern "C" fn KernelMain(frame_buffer_config: &mut FrameBufferConfig) {
  let w = 300;
  let h = 200;

  for x in 0..w {
    for y in 0..h {
      unsafe {
        let p = frame_buffer_config
          .frame_buffer
          .add(4 * (frame_buffer_config.pixels_per_scan_line * y + x) as usize);
        *p.add(0) = (255 * x / w) as u8;
        *p.add(1) = 180;
        *p.add(2) = (255 * y / h) as u8;
      }
    }
  }

  halt();
}
