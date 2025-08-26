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
    for index in 0..size {
        if *block1.add(index) != *block2.add(index) {return 1}
    }
    return 0;
}