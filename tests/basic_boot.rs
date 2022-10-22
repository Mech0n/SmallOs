#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
// `_start` and `no_main` atrribute can ignore the `main` function which is generated by custom_test_frameworks, so we need to et the name of the test framework entry function to `test_main` and call it from our `_start` entry point. 
#![reexport_test_harness_main = "test_main"]

/// an example integration test named basic_boot 
/// We don’t need any cfg(test) attributes because integration test executables are never built in non-test mode.

use core::panic::PanicInfo;
use os::println;

#[no_mangle]    // means that telling compiler not to mangle this function name
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

/// the `test_main` function which is generated by custom_test_frameworks calls this function.
// fn test_runner(test: &[&dyn Fn()]) {
//     unimplemented!();
// }
// in this file, test_runner is in lib.rs, refer to Line 4.

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    os::test_panic_handler(info)
}

#[test_case]
fn test_println() {
    println!("test_println output");
}