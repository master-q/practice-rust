#![feature(start)]
#![no_std]

use core::arch::asm;

#[no_mangle]
fn __libc_csu_fini() {}

#[no_mangle]
fn __libc_csu_init() {}

#[no_mangle]
fn __libc_start_main(main: fn() -> isize) {
    let ret = main();
    unsafe {
        asm!(
            "syscall",
            in("rax") 60, // _exit
            in("rdi") ret,
            out("rcx") _, // destroyed in kernel
            out("r11") _, // destroyed in kernel
        );
    }
}

#[panic_handler]    
fn panic(_panic: &core::panic::PanicInfo<'_>) -> ! {
    loop {}
}

#[start]
fn rust_main(_argc: isize, _argv: *const *const u8) -> isize {
    0
}
