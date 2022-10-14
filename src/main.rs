#![no_std]
#![no_main] // We'll make our own entry point

// Anytime you edit linker.ld, rm -rf target/
// 3F20_0008 fsel21 1<<3 turn pin21 into an output
// 3f20_001c gpio1_set 1<<21 turns pin 21 on
// 3f20_0028 gpio1_clear 1<<21 turns pin 21 off

/* 
    How to run the kernel
    1. Build Cargo Project: cargo rustc -- -C link-args=--script=./linker.ld
    2. arm-none-eabi-objdump -D target/armv7a-none-eabi/debug/rpi_os
    2.5. file target/armv7a-none-eabi/debug/rpi_os
    3. Tranform the kernel image into a flat binary: arm-none-eabi-objcopy -O binary target/armv7a-none-eabi/debug/rpi_os ./kernel7.img
    4. cd into mnt and copy kernel7.img cp ~/Document/development/rust/rpi_os/kernel7.img
*/

use core::panic::PanicInfo;
use core::arch::asm;

mod boot { // Make sure that _start() is the first function to run
    use core::arch::global_asm;

    global_asm!(
        ".section .text._start"
    );
}

#[no_mangle] // To make sure that the link environment does not mangle the name (it should be _start())
pub extern "C" fn _start() -> ! {

    unsafe {
        // Turn PIN21 into an output
        core::ptr::write_volatile(0x3F20_0008 as *mut u32, 1 << 3);

        loop {
            // Turn Pin On
            core::ptr::write_volatile(0x3F20_001C as *mut u32, 1<<21);

            for _ in 1..5000 {
                asm!("nop");
            }
            // Turn Pin Off
            core::ptr::write_volatile(0x3F20_0028 as *mut u32, 1<<21);

            for _ in 1..5000 {
                asm!("nop");
            }
        }
    }

    // loop {} // To confirm it will never return anything
}

#[panic_handler] // A function for the processor to execute if something goes wrong
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
