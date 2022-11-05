#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![feature(abi_x86_interrupt)]  // for interrupt calling convention
#![feature(alloc_error_handler)] 
#![feature(const_mut_refs)]

extern crate alloc;

/// make the required functions available to our integration test, 
/// we need to split off a library from our main.rs, 
/// which can be included by other crates and integration test executables. 

use core::panic::PanicInfo;

#[cfg(test)]
use bootloader::{entry_point, BootInfo};

// make println and serial_println available
// make the modules public to make them usable outside of our library. 
pub mod gdt;
pub mod serial;
pub mod memory;
pub mod vga_buffer;
pub mod interrupts;
pub mod allocator;

#[cfg(test)]
entry_point!(test_kernel_main);

pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T
where
    T: Fn(),
{
    fn run(&self) {
        serial_print!("{}...\t", core::any::type_name::<T>());
        self();
        serial_println!("[ok]");
    }
}

pub fn test_runner(tests: &[&dyn Testable]) {
    serial_println!("Running {} tests", tests.len());
    for test in tests {
        test.run();
    }
    exit_qemu(QemuExitCode::Success);
}

pub fn test_panic_handler(info: &PanicInfo) -> ! {
    serial_println!("[failed]\n");
    serial_println!("Error: {}\n", info);
    exit_qemu(QemuExitCode::Failed);
    // loop {}
    hlt_loop();
}

/// Entry point for `cargo test`
/// cargo test --lib
#[cfg(test)]
#[no_mangle]
// pub extern "C" fn _start() -> ! {
fn test_kernel_main(_boot_info: &'static BootInfo) -> ! {
    init();
    test_main();
    // loop {}
    hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    test_panic_handler(info)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

pub fn init() {
    gdt::init();
    interrupts::init_idt();
    unsafe {interrupts::PICS.lock().initialize()};
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}

#[alloc_error_handler]
fn alloc_error_handler(layout: alloc::alloc::Layout) -> ! {
    panic!("allocation error: {:?}", layout)
}