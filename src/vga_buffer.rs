
pub const BUFFER_HEIGHT: usize = 25;
pub const BUFFER_WIDTH:  usize = 80;

#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Colour{
    Black 		= 0,
    Blue 		= 1,
    Green 		= 2,
    Cyan 		= 3,
    Red 		= 4,
    Magenta 	= 5,
    Brown 		= 6,
    LightGray 	= 7,
    DarkGray 	= 8,
    LightBlue	= 9,
    LightGreen 	= 10,
    LightCyan 	= 11,
    LightRed 	= 12,
    Pink 		= 13,
    Yellow 		= 14,
    White 		= 15,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(transparent)]
pub struct ColourCode(u8);
impl ColourCode {
	fn new(foreground: Colour, background: Colour) -> ColourCode{
		ColourCode((background as u8) << 4 | (foreground as u8))
	}
}
	
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]
pub struct ScreenChar {
	pub ascii_char:  u8,
	pub colour_code: ColourCode,
}

use volatile::Volatile;	
#[repr(transparent)]
pub struct Buffer {
	pub chars: 		 [[Volatile<ScreenChar>; BUFFER_WIDTH]; BUFFER_HEIGHT],
}
	
pub struct Writer {
	pub row: 		 usize,
	pub column: 	 usize,
	pub colour_code: ColourCode,
	pub buffer: 	 &'static mut Buffer,
}

impl Writer {
	pub fn write_byte(&mut self, byte: u8) {
		match byte {
			b'\n' => self.new_line(),
			byte => {
				if self.column >= BUFFER_WIDTH {
					self.new_line();
				}
				
				let row = self.row;
				let col = self.column;
				let colour_code = self.colour_code;
				
				self.buffer.chars[row][col].write(ScreenChar {
					ascii_char: byte,
					colour_code,
				});

				self.column += 1;
			}
		}
	}
		
	pub fn write_string(&mut self, s: &str){
		for byte in s.bytes(){
			match byte{
				0x20..=0x7e | b'\n' => self.write_byte(byte),
				_ => self.write_byte(0xfe),
			}
		}
	}
		
	pub fn new_line(&mut self) {
		// Shift text up if buffer full
		if self.row == (BUFFER_HEIGHT-1){
			for row in 0..BUFFER_HEIGHT-1{
				for col in 0..BUFFER_WIDTH{
					let char = self.buffer.chars[row][col].read();
					if row > 0{	self.buffer.chars[row -1][col].write(char); } 	// Stops top row from overflowing
				}
			}
		}else{
			self.row += 1;
		}
		self.column = 0;
	}

	pub fn clear_row(&mut self, row: usize) {
		let blank = ScreenChar { ascii_char: b' ', colour_code: self.colour_code};
		
		for col in 0..BUFFER_WIDTH {
			self.buffer.chars[row][col].write(blank);
		}
	}
}

use core::fmt;
impl fmt::Write for Writer {
	fn write_str(&mut self, s: &str) -> fmt::Result {
		self.write_string(s);
		Ok(())
	}
}

use lazy_static::lazy_static;
use spin::Mutex;
lazy_static! {
	pub static ref WRITER: Mutex<Writer> = Mutex::new(Writer {
		row: 0,
		column: 0,
		colour_code: ColourCode::new(Colour::White, Colour::Black),
		buffer: unsafe { &mut *(0xb8000 as *mut Buffer) },
	});
}

/* MACROS */
#[macro_export]
macro_rules! print {
	($($arg:tt)*) => ($crate::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
	() => ($crate::print!("\n"));
	($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! clear {
	() => { $crate::_clear();};
}

#[macro_export]
macro_rules! setCursor {
	($col:tt,$row:tt) => { crate::vga_buffer::_set_cursor($row, $col);	};
}

#[macro_export]
macro_rules! setColourcode {
	($foreground:path,$background:path) => { $crate::vga_buffer::_set_colourcode($foreground, $background);	};
}

#[doc(hidden)]
pub fn _clear(){
	for _i in 0..crate::vga_buffer::BUFFER_HEIGHT{
		WRITER.lock().clear_row(_i);
	}
	setCursor!(0,0);
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
	use core::fmt::Write;
	WRITER.lock().write_fmt(args).unwrap();
}

#[doc(hidden)]
pub fn _set_cursor(row: usize, col: usize){
	WRITER.lock().row = row;
	WRITER.lock().column = col;	
}

#[doc(hidden)]
pub fn _set_colourcode(foreground: Colour, background: Colour){
	WRITER.lock().colour_code = ColourCode::new(foreground, background);
}