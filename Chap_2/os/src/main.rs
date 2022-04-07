#![no_std]
#![no_main]
#![feature(panic_info_message)]


#[macro_use]
mod console;
mod sbi;
mod lang_items;
pub mod batch;
mod sync;
pub mod syscall;
pub mod trap;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));    // 用以将应用程序静态链接到内核里

#[no_mangle]
pub fn rust_main() -> !{
    clear_bss();
    println!("[kernel] Hello, world");
    trap::init();
    batch::run_next_app();
}

fn clear_bss() {
    extern "C"{
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each( |a|{
        unsafe { (a as *mut u8).write_volatile(0) }
    });
}
