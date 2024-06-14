// os/src/main.ts
#![no_main]
#![no_std]
#![feature(panic_info_message)]

mod lang_items;
mod sbi;
#[macro_use]
mod console;

use core::arch::global_asm;
global_asm!(include_str!("entry.asm")); // 将entry.asm中的汇编代码转为字符串并嵌入到该位置
                            
#[no_mangle]
pub fn rust_main() -> ! {
   clear_bss();
   println!("hello macro!");
   panic!("shutdown machine!");

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
