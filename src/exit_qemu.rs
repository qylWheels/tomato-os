#[repr(u32)]
#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub enum ExitCode {
    Success = 0x20,
    Failed = 0x21,
}

#[allow(dead_code)]
pub fn exit_qemu(exit_code: ExitCode) {
	use x86_64::instructions::port::Port;

	unsafe {
		let mut port = Port::new(0xf4);
		port.write(exit_code as u32);
	}
}
