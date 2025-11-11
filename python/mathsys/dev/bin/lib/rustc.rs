//^
//^ MEMORY
//^

//> MEMORY -> COPY
#[no_mangle]
pub fn memcpy(destination: *mut u8, source: *const u8, size: usize) -> *mut u8 {
    for index in 0..size {unsafe {
        *destination.add(index) = *source.add(index);
    }}
    return destination;
}

//> MEMORY -> SET
#[no_mangle]
pub fn memset(destination: *mut u8, set: u8, size: usize) -> *mut u8 {
    for index in 0..size {unsafe {
        *destination.add(index) = set as u8;
    }}
    return destination;
}

//> MEMORY -> BCMP
#[no_mangle]
pub fn bcmp(block1: *const u8, block2: *const u8, size: usize) -> u8 {
    for index in 0..size {unsafe {
        if *block1.add(index) != *block2.add(index) {return 1}
    }}
    return 0;
}

//> MEMORY -> MEMCMP
#[no_mangle]
pub fn memcmp(block1: *const u8, block2: *const u8, size: usize) -> i32 {
    for index in 0..size {
        let left = unsafe {*block1.add(index)};
        let right = unsafe {*block2.add(index)};
        if left != right {return (left as usize - right as usize) as i32}
    }
    return 0;
}


//^
//^ RUSTC
//^

//> RUSTC -> PERSONALITY
#[no_mangle]
pub fn rust_eh_personality() -> () {}

//> RUSTC -> UNWIND RESUME
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
    loop {}
}

//> RUSTC -> PANIC HANDLER
#[panic_handler]
fn panic(info: &crate::PanicInfo) -> ! {
    crate::stack::exit(255);
}