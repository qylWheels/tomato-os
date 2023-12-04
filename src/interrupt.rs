use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt;

// Initialize IDT
lazy_static! {
	static ref IDT: idt::InterruptDescriptorTable = {
		let mut tmp = idt::InterruptDescriptorTable::new();
		tmp.breakpoint.set_handler_fn(breakpoint_handler);
		tmp
	};
}

pub fn init_idt() {
	IDT.load();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: idt::InterruptStackFrame) {
	println!("EXCEPTION OCCURED: BREAKPOINT");
	println!("{stack_frame:#?}");
}

#[test_case]
fn test_int3() {
	use x86_64::instructions::interrupts::int3;
	init_idt();
	int3();
}
