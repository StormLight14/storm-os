#![no_std]
#![no_main]

use core::panic::PanicInfo;
use vga_buffer::{ColorCode, Color};
use core::fmt::Write;

mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    write!(vga_buffer::WRITER.lock(), "Hello World; 你好地球!").unwrap();
    loop {}
}