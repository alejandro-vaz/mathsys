//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::Settings;


//^
//^ FORMATTING
//^

//> FORMATTING -> FUNCTION
fn print(string: &str, append: &[u8]) -> () {
    let mut bytes = Vec::with_capacity(
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
pub fn login(settings: &Settings) -> () {print(&format!(
    "LOGIN: Running Mathsys v{}.{}.{}, consuming {} tokens.",
    settings.version[0],
    settings.version[1],
    settings.version[2],
    settings.ir.len()
), &[0x1B, 0x5B, 0x31, 0x3B, 0x39, 0x32, 0x3B, 0x34, 0x39, 0x6D])}

//> CALLS -> CRASH
pub fn crash(code: Code) -> ! {
    let value = code as u8;
    print(&format!(
        "CRASH: {{{}}} {}.",
        value,
        match value {
            0 => "Run finished successfully",
            1 => "Tried to modify value of immutable variable",
            2 => "Found unexpected value type",
            3 => "Malformed Intermediate Representation",
            4 => "Unknown IR object code",
            5 => "Attempted a double annotation of a variable",
            6 => "Mismatched variable type and type of its value",
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
    MalformedIR = 3,
    UnknownIRObject = 4,
    DoubleAnnotation = 5,
    RuntimeTypeMismatch = 6,
}


//^
//^ DETAIL
//^

//> DETAIL -> SPACE
pub fn space<Type: crate::Display>(message: Type) -> () {print(&format!(
    "SPACE: {}.",
    message
), &[0x0A, 0x1B, 0x5B, 0x30, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D]); drop(message)}


//^
//^ LOOKUP
//^

//> LOOKUP -> DEBUG
pub fn debug<Type: crate::Display>(message: Type) -> () {print(&format!(
    "    DEBUG: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x35, 0x3B, 0x34, 0x39, 0x6D]); drop(message)}

//> LOOKUP -> ALERT
pub fn alert<Type: crate::Display>(message: Type) -> () {print(&format!(
    "    ALERT: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x38, 0x3B, 0x35, 0x3B, 0x32, 0x30, 0x38, 0x3B, 0x34, 0x39, 0x6D]); drop(message)}

//> LOOKUP -> TRACE
pub fn trace<Type: crate::Display>(message: Type) -> () {print(&format!(
    "    TRACE: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x36, 0x3B, 0x34, 0x39, 0x6D]); drop(message)}

//> LOOKUP -> CHORE
pub fn chore<Type: crate::Display>(message: Type) -> () {print(&format!(
    "    CHORE: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D]); drop(message)}

//> LOOKUP -> CLASS
pub fn class<Type: crate::Display>(message: Type) -> () {print(&format!(
    "    CLASS: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x32, 0x6D]); drop(message)}