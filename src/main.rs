#![no_std]
#![no_main]

use core::panic::PanicInfo;

static STR: &[u8] = b"Hello kernel world!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;
    for (i, &c) in STR.iter().enumerate() {
        unsafe {
            *(vga_buffer.add(i * 2)) = c;
            *(vga_buffer.add(i * 2 + 1)) = 0x8c;    // bright red
        }
    }
    loop {}
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    loop {}
}
