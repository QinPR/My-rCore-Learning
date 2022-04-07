mod context;

use crate::batch::run_next_app;
use crate::syscall::syscall;
use core::arch::global_asm;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap},
    stval, stvec,
};

global_asm!(include_str!("trap.S"));

// 通过 __alltraps将Trap上下文保存在内核栈上， 
// 跳转到trap_handler函数完成Trap分发与处理
// trap_handler返回后，用__restore从保存在内核栈上的Trap上下文恢复寄存器
// 最后通过sret回到应用程序

pub fn init(){
    extern "C" {fn __alltraps(); }     
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);    // 在初始化时就将__alltraps设置为trap的入口！！前两位设置为Direct
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext{
    let scause = scause::read();
    let stval = stval::read();

    println!("[kernel] Fourth Step");

    match scause.cause() {    // 对trap的原因进行分发处理
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }

        Trap::Exception(Exception::StoreFault) | 
        Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            run_next_app();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            run_next_app();
        }
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
    println!("[kernel] Fifth Step");
    cx
}

pub use context::TrapContext;