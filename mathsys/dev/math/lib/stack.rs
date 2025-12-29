//^
//^ IMPORTS
//^

//> IMPORTS -> WRITE
#[unsafe(no_mangle)]
#[linkage = "weak"]
unsafe extern "C" fn _write(pointer: *const u8) -> () {}

//> IMPORTS -> EXIT
#[unsafe(no_mangle)]
#[linkage = "weak"]
unsafe extern "C" fn _exit(code: u8) -> ! {panic!()}


//^
//^ WRAPPERS
//^

//> WRAPPERS -> WRITE
#[inline(always)]
pub fn write(pointer: *const u8) -> () {unsafe{_write(pointer)}}

//> WRAPPERS -> EXIT
#[inline(always)]
pub fn exit(code: u8) -> ! {unsafe{_exit(code)}}