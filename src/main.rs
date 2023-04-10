#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(blog_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use blog_os::{println, init, hlt_loop};
use x86_64::{VirtAddr, structures::paging::PageTable};
use core::panic::PanicInfo;
use bootloader::{BootInfo, entry_point, bootinfo};

entry_point!(kernel_main);

fn kernel_main(boot_info: &'static BootInfo) -> ! {
    use blog_os::memory;
    use blog_os::memory::BootInfoFrameAllocator;
    use x86_64::{structures::paging::Page, VirtAddr}; // 新しいインポート

    println!("Hello World{}", "!");
    blog_os::init();

    // invoke a breakpoint exception
    // x86_64::instructions::interrupts::int3(); // new

    // loop {
    //     use blog_os::print;
    //     print!("-")
    // }

    let mut frame_allocator = unsafe {
        BootInfoFrameAllocator::init(&boot_info.memory_map)
    };

    // use x86_64::registers::control::Cr3;
    let phys_mem_offset = VirtAddr::new(boot_info.physical_memory_offset);
    let mut mapper = unsafe { memory::init(phys_mem_offset) };
    let mut frame_allocator = memory::EmptyFrameAllocator;

    let page = Page::containing_address(VirtAddr::new(0));
    memory::create_example_mapping(page, &mut mapper, &mut frame_allocator);

    let page_ptr: *mut u64 = page.start_address().as_mut_ptr();
    unsafe { page_ptr.offset(400).write_volatile(0x_f021_f077_f065_f04e) };

    // let addresses = [
    //     // 恒等対応しているVGAバッファのページ
    //     0xb8000,
    //     // コードページのどこか
    //     0x201008,
    //     // スタックページのどこか
    //     0x0100_0020_1a10,
    //     // 物理アドレス "0" にマップされている仮想アドレス
    //     boot_info.physical_memory_offset,
    // ];

    // for &address in &addresses {
    //     let virt = VirtAddr::new(address);
    //     // 追加：`mapper.translate_addr`メソッドを使う
    //     let phys = mapper.translate_addr(virt);
    //     println!("{:?} -> {:?}", virt, phys);
    // }

    // let (level_4_page_table, _) = Cr3::read();
    // println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

    #[cfg(test)]
    test_main();
    
    println!("It dit not crash!");
    hlt_loop();
}

// #[no_mangle]
// pub extern "C" fn _start(boot_info: &'static BootInfo) -> ! {
//     println!("Hello World{}", "!");
//     blog_os::init();

//     // invoke a breakpoint exception
//     // x86_64::instructions::interrupts::int3(); // new

//     // loop {
//     //     use blog_os::print;
//     //     print!("-")
//     // }
//     println!("It dit not crash!");

//     use x86_64::registers::control::Cr3;

//     let (level_4_page_table, _) = Cr3::read();
//     println!("Level 4 page table at: {:?}", level_4_page_table.start_address());

//     #[cfg(test)]
//     test_main();

//     hlt_loop();
// }

/// This function is called on panic.
#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    blog_os::hlt_loop();
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    blog_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
