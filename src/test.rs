/// Due to the fact that "test_runner" attribute in main.rs
/// need to take effect whatever in non-test case or test
/// case, we have to make this module absolutely visible.
/// As a result, we removed #![cfg(test)] attribute.
use crate::{serial_print, serial_println};
use crate::exit_qemu;
use core::panic;

pub trait Testable {
    fn run(&self);
}

impl<F: Fn()> Testable for F {
    fn run(&self) {
        use core::any;
        serial_print!("testing {}()......", any::type_name::<F>());
        self();
        serial_println!("[OK]");
    }
}

pub fn tester(tests: &[&dyn Testable]) {
    for &test in tests {
        test.run();
    }
    exit_qemu::exit_qemu(exit_qemu::ExitCode::Success);
}

pub fn panic_used_in_tests(panic_info: &panic::PanicInfo) {
    serial_println!("{panic_info}");
}
