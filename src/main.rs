#![no_std]
#![no_main]
mod vga_buffer;

static HELLO: &[u8] = b"\nHello World!\n";

use core::panic::PanicInfo;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");

    loop {}
}