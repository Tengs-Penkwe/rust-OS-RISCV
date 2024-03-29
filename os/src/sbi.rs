

const SBI_SHUTDOWN: usize = 8;

#[inline(always)]
fn sbi_call(which: usize, arg0: usize, arg1: usize, arg2: usize) -> usize {
    let mut ret;
    unsafe {
        llvm_asm!("ecall"
                  : "={x10}" (ret)
                  : "{x10}" (arg0),"{x11}" (arg1),"{x12}" (arg2),"{x17}" (which)
                  : "memory"
                  : "volatile"
                  );
    }
    ret
}



pub fn shutdown() -> ! {
    sbi_call(SBI_SHUTDOWN, 0, 0, 0);
    panic!("It should shutdown!");
}
