use volatile::Volatile;
use core::fmt;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_string(s);
        Ok(())
    }
}

#[allow(dead_code)] // allow unused struct
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)] // data layout
pub enum Color{
    Black,
    Blue,
    Green,
    Cyan,
    Red,
    Megenta,
    Brown,
    LightGray,
    DarkGray,
    LightBlue,
    LightGreen,
    LightCyan,
    LightRed,
    Pink,
    Yello,
    White,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
struct ColorCode(u8);

impl ColorCode {
    fn new(foreground: Color, background: Color) -> ColorCode {
        ColorCode((background as u8) << 4 | (foreground as u8))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]  // data layout like C
struct ScreenChar {
    ascii_character: u8,
    color_code: ColorCode,
}

const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

pub struct Writer {
    column_position: usize,
    color_code: ColorCode,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >=  BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;    // write on the last line.
                let col = self.column_position;

                let color_code = self.color_code;
                self.buffer.chars[row][col].write(ScreenChar{
                    ascii_character: byte,
                    color_code,
                });
                
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {/* TODO */}

    pub fn write_string(&mut self, s: &str) {
        for byte in s.bytes() {
            match byte {
                0x20..=0x7e | b'\n' => self.write_byte(byte),
                _ => self.write_byte(0xfe),
            }
        }
    }
}


/// test the struct : Writer 
pub fn vga_buffer_test() {
    use core::fmt::Write;   // for the last line code which use write!/writeln! marcos in the block

    let mut writer = Writer {   // must be mutable
        column_position : 0,
        color_code: ColorCode::new(Color::Yello, Color::Black),
        buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
        // we just can’t dereference raw pointers outside an unsafe block, as you’ll see in a bit.
    };

    writer.write_string("Hello ");
    writer.write_string("Wörld! ");
    write!(writer, "The numbers are {} and {}", 42, 1.0/3.0).unwrap();
}

