# [Write an OS in Rust] Day 5

> Today's task is implemented the double fault handler/GDT/TSS and learned GDT/TSS followed by [post](https://os.phil-opp.com/double-fault-exceptions/#a-stack-overflow-test)

[toc]

### OS

##### [GDT](https://web.archive.org/web/20190217233448/https://www.flingos.co.uk/docs/reference/Global-Descriptor-Table/)

While segmentation is no longer supported in 64-bit mode, the GDT still exists. It is mostly used for two things: Switching between kernel space and user space, and loading a TSS structure.

A **selector** is the number of bytes from the start of the table to the start of the descriptor (/table entry). For a standard GDT, this means the selector for the second entry (the first valid one after the NULL descriptor) has selector 0x10 (16).

```asm
; This is the GDT table pre-filled with the entries required to make the entire address space accessible 
;   from user and kernel mode for both data and code.
GDT_Contents:
  db 0, 0, 0, 0, 0, 0, 0, 0            ; Offset: 0  - Null selector - required 
  db 255, 255, 0, 0, 0, 0x9A, 0xCF, 0  ; Offset: 8  - KM Code selector - covers the entire 4GiB address range
  db 255, 255, 0, 0, 0, 0x92, 0xCF, 0  ; Offset: 16 - KM Data selector - covers the entire 4GiB address range
  db 255, 255, 0, 0, 0, 0xFA, 0xCF, 0  ; Offset: 24 - UM Code selector - covers the entire 4GiB address range
  db 255, 255, 0, 0, 0, 0xF2, 0xCF, 0  ; Offset: 32 - UM Data selector - covers the entire 4GiB address range
  db 0x67,  0, 0, 0, 0, 0xE9, 0x00, 0  ; Offset: 40 - TSS Selector - Pointer to the TSS 

TSS:
  TIMES 104 db 0
TSS_POINTER equ (TSS - KERNEL_VIRTUAL_BASE)  ; Physical address of TSS
```

##### TSS

The x86-64 architecture does not support hardware task switches. However the TSS can still be used in a machine running in the 64 bit extended modes. In these modes the TSS is still useful as it stores:

- The stack pointer addresses for each privilege level.
- Pointer Addresses for the Interrupt Stack Table (The inner-level stack pointer section above, discusses the need for this).
- Offset Address of the IO permission bitmap.

Also, the task register is expanded in these modes to be able to hold a 64-bit base address.

The 64-bit TSS has the following format:

| Field	| Type |
|  ----  | ----  |
| (reserved)	| u32 |
| Privilege Stack Table	| [u64; 3] |
| (reserved)	| u64 |
| Interrupt Stack Table	| [u64; 7] |
| (reserved)	| u64 |
| (reserved)	| u16 |
| I/O Map Base Address	| u16 |

See more info: [Task_State_Segment](https://wiki.osdev.org/Task_State_Segment)

### Rust

##### Array


A fixed-size array, denoted `[T; N]`, for the element type, `T`, and the non-negative compile-time constant size, `N`.

There are two syntactic forms for creating an array:

- A list with each element, i.e., `[x, y, z]`.
- A repeat expression `[x; N]`, which produces an array with `N` copies of `x`. The type of `x` must be Copy.

```rust
let mut array: [i32; 3] = [0; 3];

array[1] = 1;
array[2] = 2;

let array_: [i32; 5] = [1, 2, 3, 4, 5];
```
