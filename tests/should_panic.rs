#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use rust_os::{
    colors::{Green, Red},
    exit_qemu, serial_print, serial_println, QemuExitCode,
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("{},", Red("TEST DID NOT PANIC"));
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("{},", Green("ok"));
    exit_qemu(QemuExitCode::Success);
    loop {}
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(1, 1);
}
