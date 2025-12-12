//
//  MEMORY
//

// MEMORY -> COPY
#[no_mangle]
pub fn memcpy(destination: *mut u8, source: *const u8, size: usize) -> *mut u8 {
    for index in 0..size {unsafe {
        *destination.add(index) = *source.add(index);
    }}
    return destination;
}

// MEMORY -> SET
#[no_mangle]
pub fn memset(destination: *mut u8, set: usize, size: usize) -> *mut u8 {
    for index in 0..size {unsafe {
        *destination.add(index) = set as u8;
    }}
    return destination;
}

// MEMORY -> BCMP
#[no_mangle]
pub fn bcmp(block1: *const u8, block2: *const u8, size: usize) -> usize {
    for index in 0..size {unsafe {
        if *block1.add(index) != *block2.add(index) {return 1}
    }}
    return 0;
}