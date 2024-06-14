# os/src/entry.asm
  .section .text.entry
  .globl _start
_start:
  la sp, boot_stack_top # 加载这个boot_stack_top 到sp寄存器
  call rust_main
   
  .section .bss.stack
  .globl boot_stack_lower_bound
boot_stack_lower_bound:
  .space 4096*16 # 分配64kb值为0的空间 然后再标记boot_stack_lower_bound向上64kb的位置为栈顶
  .globl boot_stack_top
boot_stack_top:

