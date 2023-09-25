#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(storm_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use vga_buffer::Color;

mod serial;
mod vga_buffer;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_buffer::WRITER.lock().set_foreground(Color::LightRed);
    println!("StormOS");
    vga_buffer::WRITER.lock().set_foreground(Color::LightBlue);
    println!("Hello World!");

    #[cfg(test)]
    test_main();

    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    storm_os::test_panic_handler(info)
}
