//
//  MEMORY
//

// MEMORY -> COPY
#[no_mangle]
pub unsafe fn memcpy(destination: *mut u8, source: *const u8, size: usize) -> *mut u8 {
    for index in 0..size {
        *destination.add(index) = *source.add(index);
    }
    return destination;
}

// MEMORY -> SET
#[no_mangle]
pub unsafe fn memset(destination: *mut u8, set: usize, size: usize) -> *mut u8 {
    for index in 0..size {
        *destination.add(index) = set as u8;
    }
    return destination;
}

// MEMORY -> BCMP
#[no_mangle]
pub unsafe fn bcmp(block1: *const u8, block2: *const u8, size: usize) -> isize {
    if block1.is_null() || block2.is_null() {return 0}
    for index in 0..size {
        let a = *block1.add(index);
        let b = *block2.add(index);
        if a != b {
            return if a > b {1} else {-1};
        }
    }
    return 0;
}