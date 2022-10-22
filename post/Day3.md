# [Write an OS in Rust] Day 3

> Today's task is implement the custom test framework followed by [post](https://os.phil-opp.com/testing/)
> After the national post-graduate entrance examination, I finally squeeze in some time to go on my rust learning. XD

[toc]

### Rust

##### `$crate` in macro

Writting the test framwork, we use serial port to interact with qemu, and we need to implement some macro to write sth to qemu and vga buffer.

look at this example in `serial.rs`:
```rust
/// Use to print msg about serial port.
#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;
    SERIAL1.lock().write_fmt(args).expect("Printing to serial failed");
}

#[macro_export]
macro_rules! serial_print {
    ($($arg:tt)*) => {
        $crate::serial::_print(format_args!($($arg)*)); // [Line 1]
    };
}

/// Prints to the host through the serial interface, appending a newline.
#[macro_export]
macro_rules!  serial_println {
    () => ($crate::serial_print!("\n"));    // [Line 2]
    ($fmt:expr) => ($crate::serial_print!(concat!($fmt, "\n")));
    ($fmt:expr, $($arg:tt)*) => ($crate::serial_print!(concat!($fmt, "\n"), $($arg)*));
}
```
We use `$crate` to point out the functions' path, and `Line 1` is difference from `Line 2`, `Line 1` point out that `_print` is implemented in `serial`, but in `Line 2`, `serial_print` not. The reason is that `macro_export` moves the function name to root crate namespace.

### OS

Refer to the post and comment in code file.

Now we can use `cargo run` to compile the project and launch the qemu.
`cargo test --test [arg]` to test a imgle test.
`cargo test` to test all test.