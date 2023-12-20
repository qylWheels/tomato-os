use bootloader::bootinfo::{MemoryMap, MemoryRegionType};
use lazy_static::lazy_static;
use spin::Mutex;
use x86_64::structures::paging::mapper::OffsetPageTable;
use x86_64::structures::paging::{self, PageTable, PhysFrame, Size4KiB};
use x86_64::registers::control::Cr3;
use x86_64::{VirtAddr, PhysAddr};

/// Bitmap size in byte for 4GB physical memory.
const BITMAP_SIZE_IN_BYTE: usize = 1024 * 128;

/// Page size, fixed to 4KB
const PAGE_SIZE: usize = 4096;

/// Physical frame allocator.
/// FIXME: We assume that the maximum size of physical memory is 4GB.
#[derive(Debug)]
struct FrameAllocator {
    /// 1 refers to available, 0 refers to unavailable.
    pub bitmap: [u8; BITMAP_SIZE_IN_BYTE],
    /// Next frame to be allocated.
    next: u64,
}

// We will initialize frame allocator in `init()`.
lazy_static! {
    static ref FRAME_ALLOCATOR: Mutex<FrameAllocator> = Mutex::new(FrameAllocator {
        bitmap: [0u8; BITMAP_SIZE_IN_BYTE],
        next: 0,
    });
}

// We will initialize mapper in `init()`.
lazy_static! {
    static ref MAPPER: Mutex<Option<OffsetPageTable<'static>>> = Mutex::new(None);
}

unsafe impl paging::FrameAllocator<Size4KiB> for FrameAllocator {
    // FIXME: return `None` when there is no frame available.
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        let frame = Some(
            PhysFrame::containing_address(PhysAddr::new(self.next * PAGE_SIZE as u64))
        );
        clear_bit_in_byte(&mut self.bitmap[self.next as usize / 8], (self.next % 8) as u8);
        for i in self.next as usize..BITMAP_SIZE_IN_BYTE * 8 {
            if get_bit_in_byte(self.bitmap[i / 8], (i % 8) as u8) == true {
                self.next = i as u64;
                break;
            }
        }
        frame
    }
}

impl paging::FrameDeallocator<Size4KiB> for FrameAllocator {
    unsafe fn deallocate_frame(&mut self, frame: PhysFrame<Size4KiB>) {
        let frame_number = frame.start_address().as_u64() / PAGE_SIZE as u64;
        set_bit_in_byte(&mut self.bitmap[frame_number as usize / 8], (frame_number % 8) as u8);
    }
}

/// Caller must ensure `phys_offset` is a correct value.
#[warn(unsafe_op_in_unsafe_fn)]
unsafe fn get_current_level4_page_table(phys_offset: VirtAddr) -> &'static mut PageTable {
    let level4_page_table_phys_addr = Cr3::read().0.start_address().as_u64();
    let level4_page_table_virt_addr = level4_page_table_phys_addr + phys_offset.as_u64();
    let p_level4_page_table = level4_page_table_virt_addr as *mut PageTable;
    unsafe {
        &mut *p_level4_page_table
    }
}

#[warn(unsafe_op_in_unsafe_fn)]
pub unsafe fn init(mem_map: &MemoryMap, phys_offset: VirtAddr) {
    // Initialize mapper.
    *MAPPER.lock() = unsafe {
        Some(OffsetPageTable::new(get_current_level4_page_table(phys_offset), phys_offset))
    };

    // Initialize frame allocator.
    let usable_frames = mem_map.iter()
        .filter(|region| region.region_type == MemoryRegionType::Usable);
    for frame in usable_frames {
        for i in frame.range.start_frame_number..=frame.range.end_frame_number {
            set_bit_in_byte(&mut FRAME_ALLOCATOR.lock().bitmap[i as usize / 8], (i % 8) as u8);
        }
    }
    for i in 0..BITMAP_SIZE_IN_BYTE * 8 {
        if get_bit_in_byte(FRAME_ALLOCATOR.lock().bitmap[i / 8], (i % 8) as u8) == true {
            FRAME_ALLOCATOR.lock().next = i as u64;
            break;
        }
    }
}

/// MSB on right.
fn set_bit_in_byte(byte: &mut u8, bit: u8) {
    *byte |= 1u8 << (7 - bit);
}

/// MSB on right.
fn clear_bit_in_byte(byte: &mut u8, bit: u8) {
    *byte &= !(1u8 << (7 - bit));
}

/// MSB on right.
fn get_bit_in_byte(byte: u8, bit: u8) -> bool {
    (byte & (1u8 << (7 - bit))) != 0
}
