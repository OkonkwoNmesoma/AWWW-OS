#![no_std]
#![no_main]

mod writer;
use writer::writer::FrameBufferWriter;

use bootloader_api::entry_point;
use bootloader_api::config::Mapping;
use x86_64::instructions::hlt;
use core::fmt::Write;

pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {
    let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024; // 100 KiB
    config
};

entry_point!(my_entry_point, config = &BOOTLOADER_CONFIG);

fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    let frame_buffer_info = boot_info.framebuffer.as_mut().unwrap().info();
    let buffer = boot_info.framebuffer.as_mut().unwrap().buffer_mut();
    
    let mut frame_buffer_writer = FrameBufferWriter::new(buffer, frame_buffer_info);

    writeln!(frame_buffer_writer, "Hello, Rust OS!").expect("Failed to write to framebuffer");
    writeln!(frame_buffer_writer, "Framebuffer initialized.").expect("Failed to write to framebuffer");

    loop {
        hlt();
    }
}

#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {
        hlt();
    }
}
