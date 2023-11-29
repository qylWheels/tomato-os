#![no_std]
#![no_main]

mod vga;

use core::panic;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello {}", "world");
    println!("2 + 3 = {}", 2 + 3);
    println!("2 > 3 ? {}", 2 > 3);
    for _i in 0..160 {
        print!("a");
    }
    panic!("Normal exit");
}

#[panic_handler]
fn panic(panic_info: &panic::PanicInfo) -> ! {
    println!("{panic_info}");
    loop {}
}
