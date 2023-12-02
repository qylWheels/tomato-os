#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tomato_os::test::tester)]
#![reexport_test_harness_main = "test_main"]

use core::panic;
use tomato_os::test;
use tomato_os::exit_qemu;

#[no_mangle]
extern "C" fn _start() -> ! {
	test_main();
	loop {}
}

#[panic_handler]
fn panic(panic_info: &panic::PanicInfo) -> ! {
	test::panic_used_in_tests(panic_info);
	exit_qemu::exit_qemu(exit_qemu::ExitCode::Failed);
	loop {}
}

#[test_case]
fn fly() {
	assert!(114 == 514 - 400);
}
