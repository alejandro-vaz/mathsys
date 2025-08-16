//
//  IMPORTS
//

// IMPORTS -> BLOCK
unsafe extern "C" {
    fn systemWrite(pointer: *const u8) -> ();
    fn systemExit() -> !;
}


//
//  WRAPPERS
//

// WRAPPERS -> WRITE
pub unsafe fn write(pointer: *const u8) -> () {systemWrite(pointer)}

// WRAPPERS -> SYSTEMEXIT
pub unsafe fn exit() -> ! {systemExit()}