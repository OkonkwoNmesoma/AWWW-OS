use std::path::PathBuf;

fn main() {
    // Get the output directory set by Cargo
    let out_dir = PathBuf::from(std::env::var_os("OUT_DIR").unwrap());

    // Print OUT_DIR for debugging (Optional)
    println!("std::env::var_os('OUT_DIR') = {:?}", std::env::var_os("OUT_DIR").unwrap());

    // Get the kernel binary path (set by Cargo's artifact dependency feature)
    let kernel = PathBuf::from(std::env::var_os("CARGO_BIN_FILE_KERNEL_WITH_BOOTLOADER").unwrap());

    // Create a UEFI disk image
    let uefi_path = out_dir.join("uefi.img");
    bootloader::UefiBoot::new(&kernel)
        .create_disk_image(&uefi_path)
        .expect("Failed to create UEFI disk image");

    // Create a BIOS disk image
    let bios_path = out_dir.join("bios.img");
    bootloader::BiosBoot::new(&kernel)
        .create_disk_image(&bios_path)
        .expect("Failed to create BIOS disk image");

    // Pass the disk image paths as environment variables to `main.rs`
    println!("cargo:rustc-env=UEFI_PATH={}", uefi_path.display());
    println!("cargo:rustc-env=BIOS_PATH={}", bios_path.display());
}
