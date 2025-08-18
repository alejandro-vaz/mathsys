//
//  RUSTC
//

// RUSTC -> PERSONALITY
#[no_mangle]
pub fn rust_eh_personality() -> () {}

// CORE -> UNWIND RESUME
#[no_mangle]
pub unsafe extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

// RUSTC -> PANIC HANDLER
#[panic_handler]
unsafe fn panic(info: &crate::PanicInfo) -> ! {
    crate::stack::system::exit(255);
}