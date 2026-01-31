//
//  RUSTC
//

// RUSTC -> PERSONALITY
#[no_mangle]
pub fn rust_eh_personality() -> () {}

// RUSTC -> UNWIND RESUME
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

// RUSTC -> PANIC HANDLER
#[panic_handler]
fn panic(info: &crate::PanicInfo) -> ! {
    crate::stack::system::exit(255);
}