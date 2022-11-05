#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(os::test_runner)]
#![reexport_test_harness_main = "test_main"]

extern crate alloc;

use core::panic::PanicInfo;
use os::println;
use bootloader::{BootInfo, entry_point};

entry_point!(kernel_main);  // the macro defines the real lower level _start entry point for us.  no longer need to use extern "C" or no_mangle for our entry point

fn kernel_main(boot_info: &'static BootInfo) -> !{
    use os::memory;
    use os::allocator;
    // use x86_64::structures::paging::Page;
    use x86_64::VirtAddr;

    // use core::fmt::Write;
    // vga_buffer::WRITER.lock().write_str("Hello again!").unwrap();
    // write!(vga_buffer::WRITER.lock(), ", some numbers {} {}", 42, 1.337).unwrap();

    println!("Hellow World{}", "!");

    os::init();

    // // trigger a breakpoint
    // x86_64::instructions::interrupts::int3();

    // // trigger a page fault
    // unsafe {
    //     *(0xdeadbeef as *mut u64) = 42;
    // };

    // let ptr = 0x204bd6 as *mut u32;
    // // read from a code page
    // unsafe {let x = *ptr;}
    // println!("read worked");
    // // write to a code page
    // unsafe { *ptr = 42; }
    // println!("write worked");

    // // take a look at the page tables that define how our kernel is mapped
    // use x86_64::registers::control::Cr3;
    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());


    // // trigger a reboot. XD
    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    // // traverse the page table start
    // use os::memory::active_level_4_table;
    // use x86_64::VirtAddr;

    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let l4_table = unsafe { active_level_4_table(phys_mem_offset) };

    // // traverse the page table
    // for (i, entry) in l4_table.iter().enumerate() {
    //     if !entry.is_unused() {
    //         println!("L4 Entry {}: {:?}", i, entry);

    //         // // get the physical address from the entry and convert it
    //         // let phys = entry.frame().unwrap().start_address();
    //         // let virt = phys.as_u64() + boot_info.physical_memory_offset;
    //         // let ptr = VirtAddr::new(virt).as_mut_ptr();
    //         // let l3_table: &PageTable = unsafe { &*ptr };
            
    //         // // print non-empty entries of the level 3 table
    //         // for (i, entry) in l3_table.iter().enumerate() {
    //         //     if !entry.is_unused() {
    //         //         println!("  L3 Entry {}: {:?}", i, entry);
    //         //     }
    //         // }
    //     }
    // }

    // // traverse the page table anothor
    // use os::memory;
    // use x86_64::structures::paging::Translate;
    
    // let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    // let mapper = unsafe {memory::init(phys_mem_offset)};
    // let addresses = [
    //     // the identity-mapped vga buffer page
    //     0xb8000,
    //     // some code page
    //     0x201008,
    //     // some stack page
    //     0x0100_0020_1a10,
    //     // virtual address mapped to physical address 0
    //     boot_info.physical_memory_offset,
    // ];
    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    // // map a new virt addr to a usable phys addr

    let mut frame_allocator = unsafe {
        memory::BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };

    // // map an unused page
    // let page = Page::containing_address(VirtAddr::new(0xdeadbeaf000));
    // memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    // let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    // unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e)};

    // init heap
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap initialization failed");


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