    .section .text.entry
    .globl _start
_start:
    la sp, boot_stack_top    # 将地址的sp栈指针指向栈顶
    call rust_main           

    .section .bss.stack      # .bss.stack最终会被汇总到.bss段中
    .global boot_stack
boot_stack:                   # 栈底
    .space 4096 * 16          # 栈大小: 单位byte
    .globl boot_stack_top     # 栈顶
boot_stack_top: