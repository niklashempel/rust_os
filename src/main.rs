#![no_std]
#![no_main]

mod vga_buffer;

use core::{fmt::write, panic::PanicInfo};

static HELLO: &[u8] = b"Hello World!";

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[no_mangle]
pub extern "C" fn _start() -> ! {
    use core::fmt::Write;
    vga_buffer::WRITER.lock().write_str("Hello again").unwrap();
    write!(
        vga_buffer::WRITER.lock(),
        ", some numbers: {} {}",
        42,
        1.337
    )
    .unwrap();
    println!("\nHello World{}", "!");
    panic!("Some panic message");
    loop {}
}
