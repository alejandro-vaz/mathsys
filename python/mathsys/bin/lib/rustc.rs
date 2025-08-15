//
//  RUSTC
//

// RUSTC -> PERSONALITY
#[no_mangle]
pub fn rust_eh_personality() -> () {}

// RUSTC -> PANIC HANDLER
#[panic_handler]
unsafe fn panic(info: &crate::PanicInfo) -> ! {
    crate::stdout::issue("Panic!");
    crate::stdout::crash();
}
// CORE -> PANIC HANDLER
