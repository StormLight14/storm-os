#![no_std]
#![no_main]

use core::panic::PanicInfo;
use vga_buffer::{ColorCode, Color};
use core::fmt::Write;

mod vga_buffer;

/// This function is called on panic.
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World!");
    panic!("test panic!!!");
    loop {}
}