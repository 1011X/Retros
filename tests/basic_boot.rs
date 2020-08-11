#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![reexport_test_harness_main = "test_main"]
#![test_runner(crate::test_runner)]

use core::panic::PanicInfo;

#[macro_use]
use ::retros::println;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

/* TESTS */
#[test_case]
fn test_println_simple(){
    ::retros::println!("Simple output");
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}

#[test_case]
fn test_println_runaway() {
    for _ in 0..200 {
        ::retros::println!("Runaway output");
    }
}

#[test_case]
fn test_println_output() {
    let s = "Print test output";
    ::retros::println!("{}",s);
    
    for (i, c) in s.chars().enumerate() {
        let writer = retros::vga_buffer::WRITER.lock();
        let screen_chara = &writer.buffer.chars[writer.row][i];
        assert_eq!(char::from(screen_chara.read().ascii_char), c);
    }
}

#[test_case]
fn test_clear_buffer(){
    retros::clear!();
    let writer = &retros::vga_buffer::WRITER.lock();
    for row in 0..vga_buffer::BUFFER_HEIGHT{
        for col in 0..vga_buffer::BUFFER_WIDTH{
            let chara:vga_buffer::ScreenChar = writer.buffer.chars[row][col].read(); 
            assert_eq!(chara.ascii_char,b' ');
        }
        assert_eq!(writer.row,0);
        assert_eq!(writer.column,0);
    }
}