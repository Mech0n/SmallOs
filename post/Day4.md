# [Write an OS in Rust] Day 4

> Today's task is implement the first exception handler followed by [post](https://os.phil-opp.com/cpu-exceptions/)

[toc]

### Rust

##### Foreign calling conventions

The following is an excerpt from [FFI](https://doc.rust-lang.org/nomicon/ffi.html#foreign-calling-conventions)

Most foreign code exposes a C ABI, and Rust uses the platform's C calling convention by default when calling foreign functions. Some foreign functions, most notably the Windows API, use other calling conventions. Rust provides a way to tell the compiler which convention to use.

```rust
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}
```

The list of supported ABI constraints are:
- `stdcall`
- `aapcs`
- `cdecl`
- `fastcall`
- `vectorcall` This is currently hidden behind the abi_vectorcall gate and is subject to change.
- `Rust`
- `rust-intrinsic`
- `system`
- `C`
- `win64`
- `sysv64`

### OS

[a detailed description about stack in kernel](https://www.cnblogs.com/sky-heaven/p/12788807.html)