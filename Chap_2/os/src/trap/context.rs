use riscv::register::sstatus::{self, Sstatus, SPP};

#[repr(C)]
pub struct TrapContext{     // trap上下文：trap时需要保存的物理资源信息。全部保存下来，并在sret前恢复原样
    // 通用寄存器[0..31]
    pub x: [usize; 32],     // 很难知道32个寄存器究竟哪个需要保存，所以干脆全保存了
    // SSP sstatus
    pub sstatus: Sstatus,
    // CSR sepc -> Trap发生前执行的最后一条指令的地址
    pub sepc: usize,
}

impl TrapContext {
    pub fn set_up(&mut self, sp: usize) {self.x[2] = sp; }
    pub fn app_init_context(entry: usize, sp: usize) -> Self {   // entry: app起始位置， sp: Userstack
        println!("[kernel] Second Step!");
        let mut sstatus = sstatus::read();  // Trap发生前CPU处在哪个特权级
        sstatus.set_spp(SPP::User);
        let mut cx = Self {
            x: [0; 32],
            sstatus,
            sepc: entry,  // entry point of app
        };
        cx.set_up(sp); // app's user stack pointer
        cx   // return initail Trap context of app
    }
}