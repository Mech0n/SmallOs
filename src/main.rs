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

    os::init();
    // x86_64::instructions::interrupts::int3();

    // // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    // // trigger a reboot. XD
    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    #[cfg(test)]
    test_main();

    // panic!("Trigger a panic!");
    println!("It did not crash!");
    
    // loop {
    //     // // trigger a deadlock
    //     // use os::print;
    //     // for _ in 0..10000 {}
    //     // print!("-");
    // }

    os::hlt_loop();
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    // loop {}
    os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> !{
    os::test_panic_handler(info);
}