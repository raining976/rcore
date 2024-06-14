// os/src/main.ts
#![no_main]
#![no_std]
#![feature(panic_info_message)]

mod lang_items;
mod sbi;
mod logging;
#[macro_use]
mod console;

use core::arch::global_asm;
use log::*;
global_asm!(include_str!("entry.asm")); // 将entry.asm中的汇编代码转为字符串并嵌入到该位置
                            
#[no_mangle]
pub fn rust_main() -> ! {
  extern "C" {
        fn stext(); // begin addr of text segment
        fn etext(); // end addr of text segment
        fn srodata(); // start addr of Read-Only data segment
        fn erodata(); // end addr of Read-Only data ssegment
        fn sdata(); // start addr of data segment
        fn edata(); // end addr of data segment
        fn sbss(); // start addr of BSS segment
        fn ebss(); // end addr of BSS segment
        fn boot_stack_lower_bound(); // stack lower bound
        fn boot_stack_top(); // stack top
    }
    clear_bss();
    logging::init();
    println!("[kernel] Hello, world!");
    trace!(
        "[kernel] .text [{:#x}, {:#x})",
        stext as usize,
        etext as usize
    );
    debug!(
        "[kernel] .rodata [{:#x}, {:#x})",
        srodata as usize, erodata as usize
    );
    info!(
        "[kernel] .data [{:#x}, {:#x})",
        sdata as usize, edata as usize
    );
    warn!(
        "[kernel] boot_stack top=bottom={:#x}, lower_bound={:#x}",
        boot_stack_top as usize, boot_stack_lower_bound as usize
    );
    error!("[kernel] .bss [{:#x}, {:#x})", sbss as usize, ebss as usize);   panic!("shutdown machine!");

}

fn clear_bss() {
    extern "C"{
        fn sbss();
        fn ebss();
    }
    (sbss as usize..ebss as usize).for_each(|a| {
        unsafe { (a as *mut u8).write_volatile(0)} // 操作裸指针为不安全操作，不能通过rust的类型检查
                                                   // 因此需要放到unsafe域中
    });
}
