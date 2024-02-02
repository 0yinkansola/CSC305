

#![no_std]
#![no_main]
pub mod smart_pointer_examples;
mod std;
#[warn(unused_imports)]
mod writer;
mod r#macro;


use bootloader_api::config::Mapping;
use writer::FrameBufferWriter;
use x86_64::instructions::hlt;

// Get heap memory allocation going
extern crate alloc;
use good_memory_allocator::SpinLockedAllocator;

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

// Initialize FRAME_BUFFER_WRITER on kernel entry
pub static mut FRAME_BUFFER_WRITER: Option<FrameBufferWriter> = None;

use bootloader_api::{
    //config::Mapping,
    info::{MemoryRegion, MemoryRegionKind},
};
use crate::smart_pointer_examples::box_vs_rc;

// Use the entry_point macro to register the entry point function: bootloader_api::entry_point!(kernel_main)
// Optionally pass a custom config
pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();

    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

    // Initialize FRAME_BUFFER_WRITER here
    unsafe {
        FRAME_BUFFER_WRITER = Some(FrameBufferWriter::new(buffer, frame_buffer_info));
    }

    let frame_buffer_writer = unsafe { FRAME_BUFFER_WRITER.as_mut().unwrap() };

    use core::fmt::Write; // below requires this
    writeln!(
        frame_buffer_writer,
        "Testing testing {} and {}",
        1,
        4.0 / 2.0
    )
    .unwrap();

    println!("annyeong ;)");

    frame_buffer_writer.set_x_pos(500);
    frame_buffer_writer.set_y_pos(200);

    println!(":)");

    let last_memory_region = boot_info.memory_regions.last().unwrap();

    let mut boot_loader_memory_region = MemoryRegion::empty();

    for memory_region in boot_info.memory_regions.iter() {
        println!("\nMemory Regions are: {:?}", memory_region);
        match memory_region.kind {
            MemoryRegionKind::Bootloader => {
                boot_loader_memory_region = *memory_region;
                break;
            }
            _ => continue,
        }
    }
    let physical_memory_offset = boot_info.physical_memory_offset.into_option().unwrap();

    let heap_start = boot_loader_memory_region.end + 0x1 + physical_memory_offset;
    let heap_size = last_memory_region.end - (boot_loader_memory_region.end + 0x1);

    unsafe {
        ALLOCATOR.init(heap_start as usize, heap_size as usize);
    }

    use alloc::boxed::Box;

    let x = Box::new(33);
    println!("\n value in heap is {}", &x);

    let y = 33;
    println!("\n Value in stack is {}", &y);
    box_vs_rc();
    
    // use smart

    loop {
        hlt(); // stop x86_64 from being unnecessarily busy while looping
    }
}
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
      hlt();
}
}