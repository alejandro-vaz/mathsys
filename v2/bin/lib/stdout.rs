//
//  FORMATTING
//

// FORMATTING -> FUNCTION
fn print(string: &str, append: &[u8]) -> () {
    let mut bytes = crate::Vec::new();
    bytes.extend_from_slice(append);
    let mut characters = string.chars();
    for count in 0..crate::SETTINGS.width {
        if let Some(character) = characters.next() {
            let mut buffer = [0u8; 4];
            bytes.extend_from_slice(character.encode_utf8(&mut buffer).as_bytes());
        } else {
            bytes.push(b' ');
        }
    }
    bytes.extend_from_slice(signature().as_bytes());
    bytes.extend_from_slice(&[0x1B, 0x5B, 0x30, 0x6D, 0x0A, 0x00]);
    crate::stack::system::write(bytes.as_ptr());
}

// FORMATTING -> MEMORY SIGNATURE
fn signature() -> crate::String {
    return crate::format!(
        "    {}",
        crate::number::scientific(crate::ALLOCATOR.mark() - crate::ALLOCATOR.start())
    );
}


//
//  CALLS
//

// CALLS -> LOGIN
pub fn login() -> () {
    crate::ALLOCATOR.tempSpace(|| {
        print(
            &crate::format!(
                "LOGIN: Running Mathsys v{}.{}.{}, consuming {} tokens.",
                crate::SETTINGS.version[0],
                crate::SETTINGS.version[1],
                crate::SETTINGS.version[2],
                &crate::SETTINGS.ir.len()
            ), 
            &[0x1B, 0x5B, 0x31, 0x3B, 0x39, 0x32, 0x3B, 0x34, 0x39, 0x6D]
        );
    })
}

// CALLS -> CRASH
pub fn crash(code: u8) -> ! {
    crate::ALLOCATOR.tempSpace(|| {
        print(
            &crate::format!(
                "CRASH: {}.",
                match code {
                    0 => "Run finished successfully",
                    1 => "Out of memory",
                    2 => "Error parsing IR",
                    255 => panic!(),
                    _ => "Unknown reason"
                }
            ),
            &[0x0A, 0x1B, 0x5B, 0x31, 0x3B, 0x39, 0x31, 0x3B, 0x34, 0x39, 0x6D]
        );
    });
    crate::stack::system::exit(code);
}


//
//  DETAIL
//

// DETAIL -> SPACE
pub fn space(message: &str) -> () {
    if crate::SETTINGS.detail {
        crate::ALLOCATOR.tempSpace(|| {
            print(
                &crate::format!(
                    "SPACE: {}.",
                    message
                ),
                &[0x0A, 0x1B, 0x5B, 0x30, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D]
            );
        })
    }
}

// DETAIL -> ISSUE
pub fn issue(message: &str) -> () {
    if crate::SETTINGS.detail {
        crate::ALLOCATOR.tempSpace(|| {
            print(
                &crate::format!(
                    "ISSUE: {}.",
                    message
                ),
                &[0x0A, 0x1B, 0x5B, 0x30, 0x3B, 0x33, 0x31, 0x3B, 0x34, 0x39, 0x6D]
            );
        })
    }
}


//
//  LOOKUP
//

// LOOKUP -> DEBUG
pub fn debug(message: &str) -> () {
    if crate::SETTINGS.lookup {
        crate::ALLOCATOR.tempSpace(|| {
            print(
                &crate::format!(
                    "    DEBUG: {}.",
                    message
                ),
                &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x35, 0x3B, 0x34, 0x39, 0x6D]
            );
        })
    }
}

// LOOKUP -> ALERT
pub fn alert(message: &str) -> () {
    if crate::SETTINGS.lookup {
        crate::ALLOCATOR.tempSpace(|| {
            print(
                &crate::format!(
                    "    ALERT: {}.",
                    message
                ),
                &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D]
            );
        })
    }
}

// LOOKUP -> TRACE
pub fn trace(message: &str) -> () {
    if crate::SETTINGS.lookup {
        crate::ALLOCATOR.tempSpace(|| {
            print(
                &crate::format!(
                    "    TRACE: {}.",
                    message
                ),
                &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x36, 0x3B, 0x34, 0x39, 0x6D]
            );
        })
    }
}