#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(unused_imports)]
#![no_std]
extern crate rlibc;

/** MODULES */
#[macro_use] 
pub mod serial;

#[allow(dead_code)]
#[macro_use]
pub mod vga_buffer;

/** USINGS */
use core::panic::PanicInfo;

/** Test entry point */
#[cfg(test)]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();
    loop {}
}

#[derive(Debug,Clone,Copy,PartialEq,Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed 	= 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode){
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

/** TEST PANIC HANDLER */
#[cfg(test)]
#[panic_handler]
pub fn panic(info: &PanicInfo) -> ! {
    serial_print!("[X] Error: {}\n",info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}

/** TEST RUNNER */
#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("\nRunning {} tests", tests.len());
    for test in tests {
        test.run();
    }
    serial_println!();
    exit_qemu(QemuExitCode::Success);
}

/** TEST TRAIT */
pub trait Testable{
fn run(&self) -> ();
}

/** TEST LOOP */
impl<T> Testable for T where T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...   \t", core::any::type_name::<T>());
        self();
        serial_println!("[O]");
    }
}



