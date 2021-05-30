#![no_std]
#![no_main]
#![feature(asm)]

fn halt() -> ! {
  loop {
    unsafe {
      asm!("hlt");
    }
  }
}

#[no_mangle]
pub extern "C" fn KernelMain() {
  halt();
}
