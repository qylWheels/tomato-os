#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::tester)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]

pub mod exit_qemu;
pub mod gdt;
pub mod hlt_loop;
pub mod interrupt;
pub mod memory;
pub mod serial_print;
pub mod test;
pub mod vga;

#[cfg(test)]
use bootloader::{BootInfo, entry_point};
#[cfg(test)]
use hlt_loop::hlt_loop;

#[cfg(test)]
entry_point!(test_kernel_main);

/// This entry point is only used when testing
#[cfg(test)]
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    test_main();
    hlt_loop();
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
