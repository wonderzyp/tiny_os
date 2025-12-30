#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(tiny_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use tiny_os::println;
extern crate alloc;
use alloc::{boxed::Box, vec, vec::Vec, rc::Rc};

use bootloader::{BootInfo, entry_point};
entry_point!(kernel_main);

#[unsafe(no_mangle)]
fn kernel_main(boot_info: &'static BootInfo) -> !{
    use x86_64::{VirtAddr, structures::paging::Translate, structures::paging::Page};
    use tiny_os::allocator;
    use tiny_os::memory::{self, BootInfoFrameAllocator};
    println!("Hello World{}", "!");
    tiny_os::init();

    #[cfg(test)]
    test_main();

    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
  
    let mut mapper = unsafe {
        memory::init(phys_mem_offset)
    };
    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };
    
    allocator::init_heap(&mut mapper, &mut frame_allocator).expect("heap init failed...");

    
    let mut vec:Vec<char> = Vec::new();
    vec.push('a');
    println!("vec at {:p}", vec.as_slice());

    let x = Box::new(88);
    println!("x at {:p}", x);

    println!("Not Crashed!");
    let reference_counted = Rc::new(vec![1, 2, 3]);
    let cloned_reference = reference_counted.clone();
    println!("current reference count is {}",Rc::strong_count(&cloned_reference));
    core::mem::drop(reference_counted);
    println!("reference count is {} now", Rc::strong_count(&cloned_reference));


    loop {
        tiny_os::hlt_loop();
    }
}






// --------- Test Cases ---------


#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("{}", _info);
    loop {
        tiny_os::hlt_loop();
    }
}

#[cfg(test)]
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    tiny_os::test_panic_handler(_info)
}