//^
//^ FORMATTING
//^

//> FORMATTING -> FUNCTION
fn print(string: &str, append: &[u8]) -> () {
    let mut bytes = crate::Vec::with_capacity(
        append.len() + string.len() + 6
    );
    bytes.extend_from_slice(append);
    bytes.extend_from_slice(string.as_bytes());
    bytes.extend_from_slice(&[0x1B, 0x5B, 0x30, 0x6D, 0x0A, 0x00]);
    crate::stack::write(bytes.as_ptr());
}


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
        "CRASH: {{{}}} {}.",
        value,
        match value {
            0 => "Run finished successfully",
            1 => "Tried to modify value of immutable variable",
            2 => "Found unexpected value type",
            3 => "Locale not found",
            4 => "Malformed Intermediate Representation",
            5 => "Unknown IR object code",
            6 => "Attempted to mutcast a different object type",
            7 => "Attempted a double annotation of a variable",
            8 => "Mismatched variable type and type of its value",
            9 => "Attempted to downcast a different object type",
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
    MalformedIR = 4,
    UnknownIRObject = 5,
    FailedMutcast = 6,
    DoubleAnnotation = 7,
    RuntimeTypeMismatch = 8,
    FailedDowncast = 9
}


//^
//^ DETAIL
//^

//> DETAIL -> SPACE
pub fn space<Type: crate::Display>(message: Type) -> () {print(&crate::format!(
    "SPACE: {}.",
    message
), &[0x0A, 0x1B, 0x5B, 0x30, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D]); drop(message)}


//^
//^ LOOKUP
//^

//> LOOKUP -> DEBUG
pub fn debug<Type: crate::Display>(message: Type) -> () {print(&crate::format!(
    "    DEBUG: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x35, 0x3B, 0x34, 0x39, 0x6D]); drop(message)}

//> LOOKUP -> ALERT
pub fn alert<Type: crate::Display>(message: Type) -> () {print(&crate::format!(
    "    ALERT: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x38, 0x3B, 0x35, 0x3B, 0x32, 0x30, 0x38, 0x3B, 0x34, 0x39, 0x6D]); drop(message)}

//> LOOKUP -> TRACE
pub fn trace<Type: crate::Display>(message: Type) -> () {print(&crate::format!(
    "    TRACE: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x36, 0x3B, 0x34, 0x39, 0x6D]); drop(message)}

//> LOOKUP -> CLASS
pub fn chore<Type: crate::Display>(message: Type) -> () {print(&crate::format!(
    "    CHORE: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D]); drop(message)}

//> LOOKUP -> CHORE
pub fn class<Type: crate::Display>(message: Type) -> () {print(&crate::format!(
    "    CLASS: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x32, 0x6D])}