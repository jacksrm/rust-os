#![no_std]
#![no_main]

use core::panic::PanicInfo;

static _HELLO: &[u8] = b"Hello World";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;
