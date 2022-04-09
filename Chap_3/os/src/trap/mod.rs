mod context;

use crate::syscall::syscall;
use core::arch::global_asm;
use crate::task::{exit_current_and_run_next, suspend_current_and_run_next};
use crate::timer::set_next_trigger;
use riscv::register::{
    mtvec::TrapMode,
    scause::{self, Exception, Trap, Interrupt},
    sie, stval, stvec,
};

global_asm!(include_str!("trap.S"));

// 通过 __alltraps将Trap上下文保存在内核栈上， 
// 跳转到trap_handler函数完成Trap分发与处理
// trap_handler返回后，用__restore从保存在内核栈上的Trap上下文恢复寄存器
// 最后通过sret回到应用程序

pub fn init(){
    extern "C" {fn __alltraps(); }     // 使用alltrap将Trap上下文保存在内核栈上
    unsafe {
        stvec::write(__alltraps as usize, TrapMode::Direct);    // 在初始化时就将__alltraps设置为trap的入口！！
    }
}

/// timer interrupt enabled
pub fn enable_timer_interrupt() {
    unsafe {
        sie::set_stimer();
    }
}

#[no_mangle]
pub fn trap_handler(cx: &mut TrapContext) -> &mut TrapContext{
    let scause = scause::read();
    let stval = stval::read();


    match scause.cause() {    // 对trap的原因进行分发处理
        Trap::Exception(Exception::UserEnvCall) => {
            cx.sepc += 4;
            cx.x[10] = syscall(cx.x[17], [cx.x[10], cx.x[11], cx.x[12]]) as usize;
        }
        Trap::Exception(Exception::StoreFault) | 
        Trap::Exception(Exception::StorePageFault) => {
            println!("[kernel] PageFault in application, kernel killed it.");
            exit_current_and_run_next();
        }
        Trap::Exception(Exception::IllegalInstruction) => {
            println!("[kernel] IllegalInstruction in application, kernel killed it.");
            exit_current_and_run_next();
        }
        Trap::Interrupt(Interrupt::SupervisorTimer) => {
            set_next_trigger();
            suspend_current_and_run_next();
        }
        _ => {
            panic!("Unsupported trap {:?}, stval = {:#x}!", scause.cause(), stval);
        }
    }
    cx
}

pub use context::TrapContext;