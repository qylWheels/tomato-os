#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tomato_os::test::tester)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;
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

    // Tasks!
    use tomato_os::task::Task;
    use tomato_os::task::executor::Executor;

    let mut executor = Executor::new();
    executor.spawn(Task::new(busy_task_2()));
    executor.spawn(Task::new(busy_task_1()));
    executor.run();

    #[cfg(test)]
    test_main();

    println!("Tomato OS is still under development!");

    hlt_loop();
}

async fn loop_until_happy() -> u64 {
    let mut sum: u64 = 0;
    for i in 0..100000000 {
        sum += i;
    }
    sum
}

async fn busy_task_1() {
    let sum = loop_until_happy().await;
    println!("sum = {sum}");
}

async fn busy_task_2() {
    let mut sum: u64 = 0;
    for i in 0..10000 {
        sum += i;
    }
    println!("sum = {sum}");
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
