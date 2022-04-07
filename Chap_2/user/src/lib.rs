#![feature(linkage)]     // 支持下面的链接操作
#![no_std]
#![feature(panic_info_message)]

#[macro_use]
pub mod console;
mod lang_items;
mod syscall;

use syscall::*;

#[no_mangle]
#[link_section = ".text.entry"]
pub extern "C" fn _start() -> ! {
    clear_bss();
    exit(main());
    panic!("unreachable after sys_exit!");
}

#[linkage = "weak"]    // 弱连接，会优先执行bin下的程序的main（）
#[no_mangle]
fn main() -> i32 {
    panic!("Cannot find main!");
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

pub fn write(fd: usize, buf:&[u8]) -> isize{
    syswrite(fd, buf)
}

pub fn exit(exit_code: i32) -> isize {
    sys_exit(exit_code)
}
