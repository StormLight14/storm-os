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
    vga_buffer::WRITER.lock().set_foreground(Color::LightRed);
    println!("StormOS");
    vga_buffer::WRITER.lock().set_foreground(Color::LightBlue);
    println!("Hello World!");
    loop {}
}