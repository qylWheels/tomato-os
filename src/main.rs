#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tomato_os::test::tester)]
#![reexport_test_harness_main = "test_main"]

use core::panic;
use tomato_os::{print, println};

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

#[cfg(not(test))]
#[panic_handler]
fn panic(panic_info: &panic::PanicInfo) -> ! {
    println!("{panic_info}");
    loop {}
}

#[cfg(test)]
use tomato_os::exit_qemu;
#[cfg(test)]
#[panic_handler]
fn panic(panic_info: &panic::PanicInfo) -> ! {
    use tomato_os::serial_println;
    serial_println!("{panic_info}");
    exit_qemu::exit_qemu(exit_qemu::ExitCode::Failed);
    loop {}
}

#[test_case]
fn trivial_test() {
    assert_eq!(3, 3);
}
