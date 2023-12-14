//! Implementation of interrupt handling mechanism.

use crate::println;
use lazy_static::lazy_static;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};
use x86_64::instructions::interrupts;
use hardware_interrupt::{
    HardwareInterruptVectorNumber,
    timer_interrupt_handler,
    keyboard_interrupt_handler,
};

mod hardware_interrupt;

/// Index of IST that double fault handler uses
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

// Initialize IDT
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler);
        idt.page_fault.set_handler_fn(page_fault_handler);
        unsafe {
            idt.double_fault.set_handler_fn(double_fault_handler)
                .set_stack_index(DOUBLE_FAULT_IST_INDEX);
        }
        idt[HardwareInterruptVectorNumber::Timer as usize].set_handler_fn(timer_interrupt_handler);
        idt[HardwareInterruptVectorNumber::Keyboard as usize].set_handler_fn(keyboard_interrupt_handler);
        idt
    };
}

/// Initialize interrupts
pub fn init_interrupts() {
    IDT.load();
    hardware_interrupt::init_8259();
    interrupts::enable();
}

extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION OCCURED: BREAKPOINT");
    println!("{stack_frame:#?}");
}

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    println!("EXCEPTION OCCURED: DOUBLE FAULT");
    println!("{stack_frame:#?}");
    loop {}
}

use x86_64::structures::idt::PageFaultErrorCode;
extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    err_code: PageFaultErrorCode,
)
{
    use x86_64::registers::control::Cr2;
    use crate::hlt_loop::hlt_loop;
    println!("EXCEPTION OCCURED: PAGE FAULT");
    println!("Page fault address: {:?}", Cr2::read());
    println!("Error code: {err_code:?}");
    println!("{stack_frame:#?}");
    hlt_loop();
}

#[test_case]
fn test_int3() {
    use x86_64::instructions::interrupts::int3;
    init_interrupts();
    int3();
}
