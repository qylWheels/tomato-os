#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tomato_os::test::tester)]
#![reexport_test_harness_main = "test_main"]

use core::panic;
use tomato_os::println;
use tomato_os::interrupt;
use x86_64::instructions::interrupts;

static STARTUP_ASCII_PATTERN: &str = "
,--------.                          ,--.              ,-----.  ,---.   
'--.  .--',---. ,--,--,--. ,--,--.,-'  '-. ,---.     '  .-.  ''   .-'  
   |  |  | .-. ||        |' ,-.  |'-.  .-'| .-. |    |  | |  |`.  `-.  
   |  |  ' '-' '|  |  |  |\\ '-'  |  |  |  ' '-' '    '  '-'  '.-'    | 
   `--'   `---' `--`--`--' `--`--'  `--'   `---'      `-----' `-----'  
";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("{STARTUP_ASCII_PATTERN}");

    // initialization
    interrupt::init_idt();

    #[cfg(test)]
    test_main();

    // toggle a breakpoint exception manually
    interrupts::int3();

    println!("Tomato OS is still under development!");

    loop {}
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
