#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tester)]
#![reexport_test_harness_main = "test_main"]

use core::panic;
use tomato_os::serial_println;
use tomato_os::exit_qemu;

#[no_mangle]
extern "C" fn _start() -> ! {
	test_main();
	loop {}
}

#[panic_handler]
fn panic(panic_info: &panic::PanicInfo) -> ! {
	serial_println!("{panic_info}");
	// exit with success code when panic
	exit_qemu::exit_qemu(exit_qemu::ExitCode::Success);
	loop {}
}

/// We must implement test runner in ourselves
/// because test runner supported by tomato_os::test
/// exits when all tests pass, which is contrary to
/// our goal.
fn tester(tests: &[&dyn Fn()]) {
	for &test in tests {
		test();
		serial_println!("It didn't panic!");
		exit_qemu::exit_qemu(exit_qemu::ExitCode::Failed);
	}
	serial_println!("No tests.");
	exit_qemu::exit_qemu(exit_qemu::ExitCode::Success);
}

#[test_case]
fn it_should_panic() {
	assert_eq!(1, 2);
}
