//
//  ASSEMBLY
//

// ASSEMBLY -> IMPORTS
unsafe extern "C" {
    pub fn numberAdd(x: usize, y: usize) -> usize;
    pub fn write(pointer: *const u8) -> ();
    pub fn exit() -> !;
}