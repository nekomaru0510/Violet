//extern crate cc;

use std::error::Error;
//use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    // assemble the `asm.s` file
    /*
    Build::new()
        .file("../../driver/arch/rv64/boot/boot.s")
        //.flag("-mabi=lp64")
        .compile("asm");

    #[cfg(target_arch = "riscv32")]
    Build::new()
        .file("../../driver/arch/rv32/boot/boot.s")
        .flag("-mabi=ilp32")
        .compile("asm");
    */
    Ok(())
}
