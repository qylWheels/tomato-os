#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(test::tester)]
#![reexport_test_harness_main = "test_main"]

mod exit_qemu;
mod serial_print;
mod test;
mod vga;

use core::panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    println!("Hello {}", "world");
    println!("2 + 3 = {}", 2 + 3);
    println!("2 > 3 ? {}", 2 > 3);
    for _i in 0..160 {
        print!("a");
    }
    panic!("Normal exit");
}

// used in normal case
#[cfg(not(test))]
#[panic_handler]
fn panic(panic_info: &panic::PanicInfo) -> ! {
    println!("{panic_info}");
    loop {}
}

#[test_case]
fn trivial_test() {
    assert_eq!(3, 3);
}
