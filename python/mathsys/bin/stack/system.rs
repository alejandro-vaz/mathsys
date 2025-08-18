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
pub unsafe fn write(pointer: *const u8) -> () {systemWrite(pointer)}

// WRAPPERS -> SYSTEMEXIT
pub unsafe fn exit(code: u8) -> ! {systemExit(code)}