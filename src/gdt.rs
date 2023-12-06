use crate::interrupt;
use lazy_static::lazy_static;
use x86_64;
use x86_64::instructions::tables;
use x86_64::structures::{gdt, tss};
use x86_64::registers::segmentation::{self, Segment};

lazy_static! {
	static ref TSS: tss::TaskStateSegment = {
		let mut tss = tss::TaskStateSegment::new();
		const STACK_SIZE: usize = 4096 * 16;
		static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];
		tss.interrupt_stack_table[interrupt::DOUBLE_FAULT_IST_INDEX as usize] = x86_64::VirtAddr::from_ptr(
			unsafe { (&STACK as *const u8).add(STACK_SIZE) }
		);
		tss
	};
}

#[derive(Debug)]
struct Selectors {
	kernel_code_selector: segmentation::SegmentSelector,
	tss_selector: segmentation::SegmentSelector,
}

lazy_static! {
	static ref GDT: (gdt::GlobalDescriptorTable, Selectors) = {
		let mut gdt = gdt::GlobalDescriptorTable::new();
		let kernel_code_selector = gdt.add_entry(gdt::Descriptor::kernel_code_segment());
		let tss_selector = gdt.add_entry(gdt::Descriptor::tss_segment(&TSS));
		(gdt, Selectors { kernel_code_selector, tss_selector })
	};
}

// Initialize GDT
pub fn init_gdt() {
	GDT.0.load();
	unsafe {
		segmentation::CS::set_reg(GDT.1.kernel_code_selector);
		tables::load_tss(GDT.1.tss_selector);
	}
}
