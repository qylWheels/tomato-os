#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::tester)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod exit_qemu;
pub mod gdt;
pub mod interrupt;
pub mod serial_print;
pub mod test;
pub mod vga;

/// This entry point is only used when testing
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

/// Only used when testing
#[cfg(test)]
use core::panic;
#[cfg(test)]
#[panic_handler]
fn panic(panic_info: &panic::PanicInfo) -> ! {
    serial_println!("{panic_info}");
    exit_qemu::exit_qemu(exit_qemu::ExitCode::Failed);
    loop {}
}
