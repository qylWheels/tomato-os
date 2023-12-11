#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tomato_os::test::tester)]
#![reexport_test_harness_main = "test_main"]

use core::panic;
use tomato_os::{interrupt, gdt};
use tomato_os::hlt_loop::hlt_loop;
use tomato_os::println;

const STARTUP_ASCII_PATTERN: &str = "
,--------.                          ,--.              ,-----.  ,---.   
'--.  .--',---. ,--,--,--. ,--,--.,-'  '-. ,---.     '  .-.  ''   .-'  
   |  |  | .-. ||        |' ,-.  |'-.  .-'| .-. |    |  | |  |`.  `-.  
   |  |  ' '-' '|  |  |  |\\ '-'  |  |  |  ' '-' '    '  '-'  '.-'    | 
   `--'   `---' `--`--`--' `--`--'  `--'   `---'      `-----' `-----'  
";

/// Entry point of the kernel.
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{STARTUP_ASCII_PATTERN}");

    // initialization
    interrupt::init_interrupts();
    gdt::init_gdt();

    #[cfg(test)]
    test_main();

    println!("Tomato OS is still under development!");

    hlt_loop();
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
