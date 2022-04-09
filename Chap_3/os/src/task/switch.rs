use core::arch::global_asm;

global_asm!(include_str!("switch.S"));

use super::TaskContext;

extern "C" {
    pub fn __switch(
        current_task_cx_ptr: *mut TaskContext,        // 对应switch.S中的a0
        next_task_cx_ptr: *const TaskContext          // 对应switch.S中的a1
    );
}