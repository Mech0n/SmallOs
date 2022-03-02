### [Write an OS in Rust] Day 1

Today, I finished a minimal rust kernel followed this [psot](https://os.phil-opp.com/minimal-rust-kernel/).And, You can download the source code from this [repo](https://github.com/Mech0n/SmallOs).Tell the truth, I want to lean rust by finishing the project, so this series of blogs will record both this project and some rust usage.

[toc]

### Rust 

##### Attributes

- ***Inner attributes***, written with a bang (`!`) after the hash (`#`), apply to the item that the attribute is declared within. 
- ***Outer attributes***, written without the bang after the hash, apply to the thing that follows the attribute.

##### Example

I did this before:

```rust
[···]`
#![no_mangle] // I mistakenly thought this was a valid uasge for the whole crate.

pub extern "C" fn _start() -> !{
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate(){
        unsafe{
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
[···]
```

I got this:

```zsh
➜  os git:(master) ✗ cargo bootimage                                                                                                                                                                34s
WARNING: `CARGO_MANIFEST_DIR` env variable not set
Building kernel
   Compiling os v0.1.0 (./os)
error: an inner attribute is not permitted in this context
  --> src/main.rs:22:1
   |
22 |   #![no_mangle]
   |   ^^^^^^^^^^^^^
23 | / pub extern "C" fn _start() -> !{
24 | |     loop {}
25 | | }
   | |_- the inner attribute doesn't annotate this function
   |
   = note: inner attributes, like `#![no_std]`, annotate the item enclosing them, and are usually found at the beginning of source files
help: to annotate the function, change the attribute from inner to outer style
   |
22 - #![no_mangle]
22 + #[no_mangle]
   |

error: could not compile `os` due to previous error
Error: Kernel build failed.
Stderr:
```

So, the correct usage is follwed.  The atrribute is only valid to `_start`.

```rust
#[no_mangle]
pub extern "C" fn _start() -> !{
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate(){
        unsafe{
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }
    loop {}
}
```

### OS

OS part is just used to record what I learn about os. See the detailed step to finished the os from [Writing an OS in Rust](https://os.phil-opp.com/) .

##### BIOS

> When you turn on a computer, it loads the BIOS from some special flash memory located on the motherboard. The BIOS runs self test and initialization routines of the hardware, then it looks for bootable disks. If it finds one, the control is transferred to its *bootloader*, which is a 512-byte portion of executable code stored at the disk’s beginning. Most bootloaders are larger than 512 bytes, so bootloaders are commonly split into a small first stage, which fits into 512 bytes, and a second stage, which is subsequently loaded by the first stage.
>
> The bootloader has to determine the location of the kernel image on the disk and load it into memory. It also needs to switch the CPU from the 16-bit [real mode](https://en.wikipedia.org/wiki/Real_mode) first to the 32-bit [protected mode](https://en.wikipedia.org/wiki/Protected_mode), and then to the 64-bit [long mode](https://en.wikipedia.org/wiki/Long_mode), where 64-bit registers and the complete main memory are available. Its third job is to query certain information (such as a memory map) from the BIOS and pass it to the OS kernel.

##### Real mode

> Real mode is characterized by a 20-bit segmented memory address sapce, and unlimited direct software access to all addressable memory, I/O addresses and peripheral hardware. Real mode provides no support for memory protected, multitasking, or code provilege levels.

##### Protected mode

Protected mode is an operational mode of x86 cpu. It allows system software to use features such as virtual memory, paging, and    safe multi-tasking.

For maintain  backward compatibility with earlier x86 processors, when a processor that supports x86 protected mode is power on , it begins executing instructions in real mode, protected mode may only be entered after the system software sets up one descriptor and enables PE bit in the cr0.

##### Long mode

> n the x86-64 computer architecture, **long mode** is the mode where a 64-bit operating system can access 64-bit instructions and registers. 64-bit programs are run in a sub-mode called 64-bit mode, while 32-bit programs and 16-bit [protected mode](https://en.wikipedia.org/wiki/Protected_mode) programs are executed in a sub-mode called compatibility mode. [Real mode](https://en.wikipedia.org/wiki/Real_mode) or virtual 8086 mode programs cannot be natively run in long mode.