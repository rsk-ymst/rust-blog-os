#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crate::vga_buffer::WRITER;
mod vga_buffer;

// 実行順番
// 1. cargo build
// 2. cargo bootimage
// 3. qemu-system-x86_64 -drive format=raw,file=target/x86_64-blog_os/debug/bootimage-blog_os.bin

// エントリポイントを上書きしている
#[no_mangle]
pub extern "C" fn _start() -> ! {
    // let vga_buffer = 0xb8000 as *mut u8;

    // for (i, &byte) in HELLO.iter().enumerate() {
    //     unsafe {
    //         *vga_buffer.offset(i as isize * 2) = byte;
    //         *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
    //     }
    // }
    // vga_buffer::print_something();
    // vga_buffer::print_something_v2();

    panic!("Some panic message");

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
