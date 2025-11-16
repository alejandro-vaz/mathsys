//^
//^ FORMATTING
//^

//> FORMATTING -> FUNCTION
fn print(string: &str, append: &[u8]) -> () {
    let mut bytes = crate::Vec::new();
    bytes.extend_from_slice(append);
    let mut characters = string.chars();
    for count in 0..crate::SETTINGS.width {
        if let Some(character) = characters.next() {
            let mut buffer = [0u8; 4];
            bytes.extend_from_slice(character.encode_utf8(&mut buffer).as_bytes());
        } else {bytes.push(b' ')}
    }
    bytes.extend_from_slice(signature().as_bytes());
    bytes.extend_from_slice(&[0x1B, 0x5B, 0x30, 0x6D, 0x0A, 0x00]);
    crate::stack::write(bytes.as_ptr());
}

//> FORMATTING -> MEMORY SIGNATURE
fn signature() -> crate::String {return crate::format!(
    "    {}",
    crate::formatting::scientific(crate::ALLOCATOR.mark())
)}


//^
//^ CALLS
//^

//> CALLS -> LOGIN
pub fn login() -> () {print(&crate::format!(
    "LOGIN: Running Mathsys v{}.{}.{}, consuming {} tokens.",
    crate::SETTINGS.version[0],
    crate::SETTINGS.version[1],
    crate::SETTINGS.version[2],
    &crate::SETTINGS.ir.len()
), &[0x1B, 0x5B, 0x31, 0x3B, 0x39, 0x32, 0x3B, 0x34, 0x39, 0x6D])}

//> CALLS -> CRASH
pub fn crash(code: Code) -> ! {
    let value = code as u8;
    print(&crate::format!(
        "CRASH: {} {}.",
        value,
        match value {
            0 => "Run finished successfully",
            1 => "Tried to modify value of immutable variable",
            2 => "Found unexpected value type",
            3 => "Locale not found",
            4 => "Out of memory bounds",
            5 => "Malformed Intermediate Representation",
            6 => "Unknown IR object code",
            7 => "Start object not found",
            8 => "Attempted to downcast a different object type",
            9 => "Invalid deallocation data",
            255 => crate::stack::exit(255),
            other => loop {}
        }
    ), &[0x0A, 0x1B, 0x5B, 0x31, 0x3B, 0x39, 0x31, 0x3B, 0x34, 0x39, 0x6D]);
    crate::stack::exit(value);
}

//> CALLS -> CRASH ENUM
pub enum Code {
    Success = 0,
    ImmutableModification = 1,
    UnexpectedValue = 2,
    LocaleNotFound = 3,
    OutOfMemory = 4,
    MalformedIR = 5,
    UnknownIRObject = 6,
    StartNotFound = 7,
    FailedDowncast = 8,
    FailedMutcast = 9,
    Fatal = 255
}


//^
//^ DETAIL
//^

//> DETAIL -> SPACE
pub fn space(message: &str) -> () {print(&crate::format!(
    "SPACE: {}.",
    message
), &[0x0A, 0x1B, 0x5B, 0x30, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D])}

//> DETAIL -> ISSUE
pub fn issue(message: &str) -> () {print(&crate::format!(
    "ISSUE: {}.",
    message
), &[0x0A, 0x1B, 0x5B, 0x30, 0x3B, 0x33, 0x31, 0x3B, 0x34, 0x39, 0x6D])}


//^
//^ LOOKUP
//^

//> LOOKUP -> DEBUG
pub fn debug(message: &str) -> () {print(&crate::format!(
    "    DEBUG: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x35, 0x3B, 0x34, 0x39, 0x6D])}

//> LOOKUP -> ALERT
pub fn alert(message: &str) -> () {print(&crate::format!(
    "    ALERT: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x38, 0x3B, 0x35, 0x3B, 0x32, 0x30, 0x38, 0x3B, 0x34, 0x39, 0x6D])}

//> LOOKUP -> TRACE
pub fn trace(message: &str) -> () {print(&crate::format!(
    "    TRACE: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x36, 0x3B, 0x34, 0x39, 0x6D])}