#![no_std]
#![no_main]
#![feature(panic_info_message)]
#![feature(register_tool)]
#![register_tool(clipper)]
#![feature(alloc_error_handler)]

#[macro_use]
mod console;
mod sbi;
mod lang_items;
mod sync;
mod loader;
mod timer;
pub mod config;
pub mod syscall;
pub mod trap;
pub mod task;

extern crate alloc;
extern crate bitflags;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm"));
global_asm!(include_str!("link_app.S"));    // 用以将应用程序静态链接到内核里

#[no_mangle]
pub fn rust_main() -> !{
    clear_bss();
    println!("[kernel] Hello, world");
    trap::init();                         // 将trap上下文保存在内核栈上， 所有程序共享一个trap上下文
    loader::load_apps();
    trap::enable_timer_interrupt();    // 设置sie.stie使得S特权级时钟中断不会被屏蔽
    timer::set_next_trigger();
    task::run_first_task();
    panic!("[kernel] Finish!");
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
