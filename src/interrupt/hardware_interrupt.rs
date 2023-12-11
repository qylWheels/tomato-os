use crate::print;
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

/// Hardware interrupt vector number
#[derive(Debug)]
#[repr(u8)]
pub(super) enum HardwareInterruptVectorNumber {
    Timer = PIC8259_PRIMARY_OFFSET,
    Keyboard,
}

// Assign interrupt vector numbers for interrupts come from 8259
const PIC8259_PRIMARY_OFFSET: u8 = 32;
const PIC8259_SECONDARY_OFFSET: u8 = 32 + 8;
// Initialize 8259
lazy_static! {
    pub(super) static ref PIC8259: Mutex<ChainedPics> = {
        unsafe {
            let pic8259 = ChainedPics::new(PIC8259_PRIMARY_OFFSET, PIC8259_SECONDARY_OFFSET);
            Mutex::new(pic8259)
        }
    };
}

// Initialize 8259
pub(super) fn init_8259() {
    unsafe { PIC8259.lock().initialize(); }
}

/// Timer interrupt handler
pub(super) extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    unsafe {
        PIC8259.lock().notify_end_of_interrupt(HardwareInterruptVectorNumber::Timer as u8);
    }
}

/// Keyboard interrupt handler
pub(super) extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use pc_keyboard::{layouts, Keyboard, HandleControl, DecodedKey, ScancodeSet1};
    use x86_64::instructions::port::Port;

    // Initialize keyboard
    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> = {
            let keyboard = Keyboard::new(ScancodeSet1::new(), layouts::Us104Key, HandleControl::Ignore);
            Mutex::new(keyboard)
        };
    }

    // Read a byte from data port of the PS/2 controller
    const PORT_NUMBER_OF_DATA_PORT: u16 = 0x60;
    let mut port = Port::new(PORT_NUMBER_OF_DATA_PORT);
    let scancode: u8 = unsafe { port.read() };

    // Interpret scancode
    let mut keyboard_lock = KEYBOARD.lock();
    if let Some(key_event) = keyboard_lock.add_byte(scancode).unwrap() {
        if let Some(DecodedKey::Unicode(c)) = keyboard_lock.process_keyevent(key_event) {
            print!("{c}");
        }
    }

    // EOI
    unsafe {
        PIC8259.lock().notify_end_of_interrupt(HardwareInterruptVectorNumber::Keyboard as u8);
    }
}
