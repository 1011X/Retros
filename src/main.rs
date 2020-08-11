#![no_std]  									// No Standard lib
#![no_main] 									// No Main Entry Point
#![feature(custom_test_frameworks)]				// Enable Custom Framework
#![test_runner(test_runner)]				// Locate Test Runner
#![reexport_test_harness_main = "test_main"]	// Export New Test Main
#![allow(unused_imports)]						// Shuts cargo up about testing module

/* USINGS */
use core::panic::PanicInfo;
#[macro_use]
use ::retros::println;

/* REG PANIC HANDLER */
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	setColourcode!(retros::vga_buffer::Colour::LightRed,retros::vga_buffer::Colour::Black);
	println!("\nPANIC: {}",info);
	loop {}
}
/* TEST PANIC HANDLER */
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
	panic(info);
}

/* MAIN LOOP */
#[no_mangle]
pub extern "C" fn _start() -> ! {
	setColourcode!(retros::vga_buffer::Colour::White,retros::vga_buffer::Colour::DarkGray);
	clear!();
	setCursor!(0,1);

	print!(" Hello ");
	setColourcode!(retros::vga_buffer::Colour::LightRed,retros::vga_buffer::Colour::Black);
	println!("Retros!");
	setColourcode!(retros::vga_buffer::Colour::White,retros::vga_buffer::Colour::DarkGray);

	setCursor!(0,23);
	println!(" Now we're cooking with gas!");

	loop{}
}

