//^
//^ IMPORTS
//^

//> IMPORTS -> WRITE
#[unsafe(no_mangle)]
#[linkage = "weak"]
unsafe extern "C" fn mathsys_write(pointer: *const u8) -> () {}

//> IMPORTS -> EXIT
#[unsafe(no_mangle)]
#[linkage = "weak"]
unsafe extern "C" fn mathsys_exit(code: u8) -> ! {panic!()}


//^
//^ WRAPPERS
//^

//> WRAPPERS -> WRITE
#[inline(always)]
pub fn write(pointer: *const u8) -> () {unsafe{mathsys_write(pointer)}}

//> WRAPPERS -> EXIT
#[inline(always)]
pub fn exit(code: u8) -> ! {unsafe{mathsys_exit(code)}}