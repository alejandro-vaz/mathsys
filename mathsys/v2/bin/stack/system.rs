//
//  IMPORTS
//

// IMPORTS -> BLOCK
unsafe extern "C" {
    fn systemWrite(pointer: *const u8) -> ();
    fn systemExit(code: u8) -> !;
}


//
//  WRAPPERS
//

// WRAPPERS -> WRITE
#[inline(always)]
pub fn write(pointer: *const u8) -> () {unsafe{systemWrite(pointer)}}

// WRAPPERS -> EXIT
#[inline(always)]
pub fn exit(code: u8) -> ! {unsafe{systemExit(code)}}