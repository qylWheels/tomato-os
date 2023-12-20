#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tomato_os::test::tester)]
#![reexport_test_harness_main = "test_main"]

use bootloader::{BootInfo, entry_point};
use core::panic;
use tomato_os::{interrupt, gdt};
use tomato_os::hlt_loop::hlt_loop;
use tomato_os::memory;
use tomato_os::println;
use x86_64::VirtAddr;

const STARTUP_ASCII_PATTERN: &str = "
,--------.                          ,--.              ,-----.  ,---.   
'--.  .--',---. ,--,--,--. ,--,--.,-'  '-. ,---.     '  .-.  ''   .-'  
   |  |  | .-. ||        |' ,-.  |'-.  .-'| .-. |    |  | |  |`.  `-.  
   |  |  ' '-' '|  |  |  |\\ '-'  |  |  |  ' '-' '    '  '-'  '.-'    | 
   `--'   `---' `--`--`--' `--`--'  `--'   `---'      `-----' `-----'  
";

// Use "kernel_main()" as kernel entry point.
entry_point!(kernel_main);

/// Entry point of the kernel.
fn kernel_main(boot_info: &'static BootInfo) -> ! {
    println!("{STARTUP_ASCII_PATTERN}");

    // Initialization.
    interrupt::init();
    gdt::init();
    unsafe {
        memory::init(&boot_info.memory_map, VirtAddr::new(boot_info.physical_memory_offset));
    }

    #[cfg(test)]
    test_main();

    println!("Tomato OS is still under development!");

    hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(panic_info: &panic::PanicInfo) -> ! {
    println!("{panic_info}");
    hlt_loop();
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
