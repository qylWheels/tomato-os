#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::tester)]
#![reexport_test_harness_main = "test_main"]

pub mod exit_qemu;
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

#[test_case]
fn test_in_lib1() {
	assert!("tomato" == concat!("to", "mato"));
}

#[test_case]
fn test_in_lib2() {
	assert!("potato" == concat!("po", "tato"));
}

#[test_case]
fn test_in_lib3() {
	assert!("114514" == concat!(114, 514));
}
