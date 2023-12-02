#![cfg(test)]

use core::panic;
use crate::{serial_print, serial_println};
use crate::exit_qemu;

// use in testing
#[panic_handler]
fn panic(panic_info: &panic::PanicInfo) -> ! {
    serial_println!("{panic_info}");
    exit_qemu::exit_qemu(exit_qemu::ExitCode::Failed);
    loop {}
}

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
