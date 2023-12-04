use core::{fmt::{self, Write}, ptr, slice};
use lazy_static::lazy_static;
use spin;
use volatile;

const MAX_LINE: usize = 25;
const MAX_COL: usize = 80;

struct VGABuffer {
    buf: &'static mut [volatile::Volatile<u8>],
    cur_line: usize,
    cur_col: usize,
}

impl VGABuffer {
    fn set_pos(&mut self, line: usize, col: usize) {
        assert!(line < MAX_LINE);
        assert!(col < MAX_COL);
        self.cur_line = line;
        self.cur_col = col;
    }

    fn new_blank_line(&mut self) {
        if self.cur_line + 1 >= MAX_LINE {
            self.shift_up()
        }
        self.set_pos(self.cur_line + 1, 0);
        for _i in 0..MAX_COL {
            self.write_byte(b' ', 0);
        }
    }

    // shift up a line without clearing the bottom line
    fn shift_up(&mut self) {
        unsafe {
            ptr::copy(
                (self.buf.as_ptr().add(1 * MAX_COL * 2)) as *const u8,
                (self.buf.as_ptr()) as *mut u8,
                (MAX_LINE - 1) * MAX_COL,
            );
        }
        if self.cur_line > 0 {
            self.set_pos(self.cur_line - 1, self.cur_col);
        }
    }

    fn write_byte(&mut self, byte: u8, color: u8) {
        match byte {
            b'\n' => self.new_blank_line(),
            _ => {
                self.buf[(self.cur_line * MAX_COL + self.cur_col) * 2].write(byte);
                self.buf[(self.cur_line * MAX_COL + self.cur_col) * 2 + 1].write(color);
            },
        }
    }
}

impl fmt::Write for VGABuffer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for index_and_byte in s.bytes().enumerate() {
            self.write_byte(index_and_byte.1, 0x0e);
            // We've already moved the pointer if current character is '\n'
            // so we don't need to move it again.
            if index_and_byte.1 != b'\n' {      
                let mut line = self.cur_line;
                let mut col = self.cur_col;
                col += 1;
                if col >= MAX_COL {
                    col = 0;
                    line += 1;
                    if line >= MAX_LINE {
                        self.shift_up();
                        self.new_blank_line();
                        continue;
                    }
                }
                self.set_pos(line, col);
            }
        }
        Ok(())
    }
}

lazy_static! {
    static ref VGA_BUFFER_HANDLE: spin::Mutex<VGABuffer> = spin::Mutex::new(VGABuffer {
        buf: unsafe {
            let ptr = 0xb8000 as *mut u8 as *mut volatile::Volatile<u8>;
            slice::from_raw_parts_mut(ptr, MAX_LINE * MAX_COL * 2)
        },
        cur_line: 0,
        cur_col: 0,
    });
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    VGA_BUFFER_HANDLE
        .lock()
        .write_fmt(args)
        .unwrap();
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::vga::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}
