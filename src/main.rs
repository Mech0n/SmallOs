#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use os::println;

#[no_mangle]
pub extern "C" fn _start() -> !{
    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again!").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers {} {}", 42, 1.337).unwrap();

    println!("Hellow World{}", "!");

    #[cfg(test)]
    test_main();

    // panic!("Trigger a panic!");
    
    loop {}
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    os::test_panic_handler(info);
}