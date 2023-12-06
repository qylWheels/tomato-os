use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt;

/// Index of IST that double fault handler uses
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// Initialize IDT
lazy_static! {
	static ref IDT: idt::InterruptDescriptorTable = {
		let mut idt = idt::InterruptDescriptorTable::new();
		idt.breakpoint.set_handler_fn(breakpoint_handler);
		unsafe {
			idt.double_fault.set_handler_fn(double_fault_handler)
				.set_stack_index(DOUBLE_FAULT_IST_INDEX);
		}
		idt
	};
}

/// Initialize IDT
pub fn init_idt() {
	IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: idt::InterruptStackFrame) {
	println!("EXCEPTION OCCURED: BREAKPOINT");
	println!("{stack_frame:#?}");
}

extern "x86-interrupt" fn double_fault_handler(
	stack_frame: idt::InterruptStackFrame,
	_error_code: u64,
) -> ! {
	println!("EXCEPTION OCCURED: DOUBLE FAULT");
	println!("{stack_frame:#?}");
	loop {}
}

#[test_case]
fn test_int3() {
	use x86_64::instructions::interrupts::int3;
	init_idt();
	int3();
}
