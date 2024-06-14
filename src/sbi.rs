// os/src/sbi.rs
// 打印字符串
pub fn console_putchar(c: usize){
    #[allow(deprecated)]
    sbi_rt::legacy::console_putchar(c); 
}

// 关机
pub fn shutdown(failure: bool) -> !{
    use sbi_rt::{system_reset, NoReason, Shutdown, SystemFailure};
    if !failure{
        system_reset(Shutdown, NoReason);
    } else{
        system_reset(Shutdown, SystemFailure);
    }
    unreachable!()
}
