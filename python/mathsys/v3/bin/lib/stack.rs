//^
//^ IMPORTS
//^

//> IMPORTS -> BLOCK
unsafe extern "C" {
    fn _write(pointer: *const u8) -> ();
    fn _exit(code: u8) -> !;
}


//^
//^ WRAPPERS
//^

//> WRAPPERS -> WRITE
#[inline(always)]
pub fn write(pointer: *const u8) -> () {unsafe{_write(pointer)}}

//> WRAPPERS -> EXIT
#[inline(always)]
pub fn exit(code: u8) -> ! {unsafe{_exit(code)}}