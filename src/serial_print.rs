use lazy_static::lazy_static;
use spin;
use uart_16550;

lazy_static! {
	static ref SERIAL: spin::Mutex<uart_16550::SerialPort> = {
		let mut serial = unsafe { uart_16550::SerialPort::new(0x3f8) };
		serial.init();
		spin::Mutex::new(serial)
	};
}

#[doc(hidden)]
pub fn _print(args: ::core::fmt::Arguments) {
    use core::fmt::Write;
    SERIAL
		.lock()
		.write_fmt(args)
		.expect("Printing to serial failed");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial_print::_print(format_args!($($arg)*));
    };
}

#[macro_export]
macro_rules! serial_println {
    () => ($crate::serial_print!("\n"));
    ($($arg:tt)*) => ($crate::serial_print!("{}\n", format_args!($($arg)*)));
}
