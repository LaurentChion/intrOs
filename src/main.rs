#![no_std]
#![no_main]

use core::panic::PanicInfo;

// This function is called on panic.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// Hello world in binary an array of char
static HELLO: &[u8] = b"Hello World!";

#[no_mangle]
pub extern "C" fn _start() -> ! {
    // adress of the vga buffer
    // cast integer into pointer
    let vga_buffer = 0xb8000 as *mut u8;

    // write each bytes in buffer 
    for (i, &byte) in HELLO.iter().enumerate() {
        // vga_buffer pointer is a raw pointer
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
