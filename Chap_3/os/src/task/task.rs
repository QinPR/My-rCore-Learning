use crate::task::context::TaskContext;

// 为这个类型提供一些Trait的默认实现
#[derive(Copy, Clone, PartialEq)]     
pub enum TaskStatus {
    UnInit,
    Ready,
    Running,
    Exited,
}

#[derive(Copy, Clone)]
pub struct TaskControlBlock {
    pub task_status: TaskStatus,
    pub task_cx: TaskContext,           // 维护任务上下文
}