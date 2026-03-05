#![no_std]
#![no_main]

use core::panic::PanicInfo;

static _HELLO: &[u8] = b"Hello World";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER
        .lock()
        .write_str("Hello Again\n")
        .unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        "Here are some cool numbers: \n{} \n{}",
        42,
        1.254
    )
    .unwrap();

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;
