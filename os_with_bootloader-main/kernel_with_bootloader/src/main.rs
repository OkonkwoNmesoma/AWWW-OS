#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)] //application binary interface

// use core::fmt::{Arguments, Write};
mod writer;
use writer::{set_frame_buffer_writer, FrameBufferWriter};

mod interrupts;

use bootloader_api::config::Mapping;
use x86_64::instructions::hlt;

extern crate alloc;

use good_memory_allocator::SpinLockedAllocator;

use crate::writer::move_writer_position;

#[global_allocator]
static ALLOCATOR: SpinLockedAllocator = SpinLockedAllocator::empty();

//Use the entry_point macro to register the entry point function: bootloader_api::entry_point!(kernel_main)
//my_entry_point can be any name.
//optionally pass a custom config
pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};
bootloader_api::entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    //boot_info.framebuffer is our target for display
    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();

    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();

    let frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info);

    set_frame_buffer_writer(frame_buffer_writer);

    interrupts::init();

    let name = "SOLIS THE GOD";
    //CA QUESTION -- print & println macro
    print!("Hello  {}", name);
    println!("Good to see you again Master");
    println!("Welcome");

    //CA QUESTION -- set position
    move_writer_position(30usize, 30usize);

    println!("Position has been changed");

    println!("Please enter a character: ");

    let mut a_char = input_char!();

    println!("The character you inputed was: {}", a_char);

    print!("Please enter a character: ");
    a_char = input_char!();

    println!("The character you inputed was: {}", a_char);

    // x86_64::instructions::interrupts::enable();

    //uncommenting the loop below would cause a deadlock
    // loop {
    //     print!("_");
    // }

    //Change X & Y position
    // if let Some(frame_buffer_writer) = FRAME_BUFFER_WRITER.lock().as_mut(){
    //     frame_buffer_writer.set_position(x_position, y_position);
    // }

    // print!("Testing {} and {}", 1000, 4000);

    // if let Some(frame_buffer_writer) = FRAME_BUFFER_WRITERBUFFER_WRITER.lock().as_mut(){
    //     frame_buffer_writer.set_position(20usize, 30usize);
    // }

    /*let mut write_str = |s:&str| {
        frame_buffer_writer
        .write_str(s)
        .unwrap() ;
    };*/

    //use core::fmt::Write;

    // check later
    // writeln!(
    //     frame_buffer_writer,
    //     "Testing testing {} and {}",
    //     1,
    //     4.0 / 2.0
    // )
    // .unwrap();

    //let memory_regions_count = boot_info.memory_regions.iter().size_hint();
    //println!("{}", memory_regions_count.0);

    // //Let's get the usable memory
    // let last_memory_region = boot_info.memory_regions.last().unwrap();
    // //println!("{:X}", last_memory_region.end);

    // //get the first bootload memory
    // let mut boot_loader_memory_region = MemoryRegion::empty();

    // for memory_region in boot_info.memory_regions.iter() {
    //     match memory_region.kind {
    //         MemoryRegionKind::Bootloader => {
    //             boot_loader_memory_region = *memory_region;
    //             break;
    //         }
    //         _ => continue,
    //     }
    // }
    // //println!("{:X} {:X} {:?}", boot_loader_memory_region.start, boot_loader_memory_region.end, boot_loader_memory_region.kind);

    // let physical_memory_offset = boot_info.physical_memory_offset.into_option().unwrap();
    // //let heap_start = 0x3E000 + physical_memory_offset;
    // //let heap_size = 0x7FC2000;
    // let heap_start = boot_loader_memory_region.end + 0x1 + physical_memory_offset;
    // let heap_size = last_memory_region.end - (boot_loader_memory_region.end + 0x1);

    // //println!("{:X} {:X}", heap_start as usize, heap_size as usize);

    // unsafe {
    //     ALLOCATOR.init(heap_start as usize, heap_size as usize);
    // }

    // use alloc::boxed::Box;

    // let x = Box::new(33);

    // check later
    // writeln!(frame_buffer_writer, "Value in X is {}", x).unwrap();

    //Let's examine our memory
    //Go through memory regions passed and add usable ones to our global allocator
    /*let mut counter = 0 as u8;
    for memory_region in boot_info.memory_regions.iter() {
        counter += 1;
        frame_buffer_writer
            .write_fmt(format_args!("{}. ", counter)) //All other formatting macros (format!, write, println!, etc) are proxied through this one. format_args!, unlike its derived macros, avoids heap allocations.
            .unwrap();
        //print!("{}. ", counter);
        frame_buffer_writer
            .write_fmt(format_args!("{:X} ", memory_region.start)) //All other formatting macros (format!, write, println!, etc) are proxied through this one. format_args!, unlike its derived macros, avoids heap allocations.
            .unwrap();
        //print!("{:X}. ", memory_region.start);
        frame_buffer_writer
            .write_fmt(format_args!("{:X}, ", memory_region.end))
            .unwrap();
        //print!("{:X}. ", memory_region.end);
        frame_buffer_writer
            .write_fmt(format_args!(
                "size = {:X}, ",
                memory_region.end - memory_region.start
            ))
            .unwrap();
        //print!("size = {:X}, ", memory_region.end - memory_region.start);
        match memory_region.kind {
            MemoryRegionKind::Usable => write!(frame_buffer_writer, "Usable;  ").unwrap(),
            MemoryRegionKind::Bootloader => write!(frame_buffer_writer, "Bootload;").unwrap(),
            MemoryRegionKind::UnknownUefi(_) => {
                write!(frame_buffer_writer, "UnknownUefi;").unwrap();
            }
            MemoryRegionKind::UnknownBios(_) => {
                write!(frame_buffer_writer, "UnknownBios;").unwrap();
            }
            _ => write!(frame_buffer_writer, "Unknown;").unwrap(),
        }
    }*/

    loop {
        hlt(); //stop x86_64 from being unnecessarily busy while looping
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}
