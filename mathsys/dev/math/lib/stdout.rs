//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    write,
    exit,
    fmt,
    NOW
};


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
    write(bytes.as_ptr());
}


//^
//^ CALLS
//^

//> CALLS -> LOGIN
pub fn login(ir: &'static [u8], version: &'static str) -> () {print(&format!(
    "LOGIN: Running Mathsys {}, consuming {} tokens.",
    version,
    ir.len()
), &[0x1B, 0x5B, 0x31, 0x3B, 0x39, 0x32, 0x3B, 0x34, 0x39, 0x6D])}

//> CALLS -> CRASH
pub fn crash(code: Code) -> ! {
    let value = code.clone() as u8;
    print(&format!(
        "CRASH: {{{}}} {}{}.",
        value,
        match code {
            Code::Success => "Run finished successfully",
            Code::UnknownIRObject => "Attempted to parse an unknown IR object",
            Code::UnexpectedEndOfIR => "IR ended prematurely",
            Code::UnknownGroupCode => "No group matches specified code",
            Code::FailedNamedRetrieval => "Failed to retrieve a value with specific group",
            Code::FailedCast => "Couldn't cast a group into another",
            Code::NoVariableOperation => "Can't operate with variables directly",
            Code::DoubleGroupAnnotation => "Cannot annotate a variable to different groups",
            Code::ImmutableModification => "Attempted to mutate the value of an immutable variable",
            Code::NaturalCannotBeZero => "Tried to assign 0 value to a natural number",
            Code::RationalDenominatorCannotBeZero => "Tried to create a rational number with denominator zero",
            Code::FailedIRDecompression => "Failed to decompress IR",
            Code::RuntimeHigherObject => "ID of runtime object supplied higher than memory length",
            Code::UnknownSignValue => "Unknown sign value in IR",
            Code::Todo => "Todo"
        },
        unsafe {match NOW {
            None => format!(""),
            Some(instant) => format!(" ({})", instant.elapsed().as_micros())
        }}
    ), &[0x0A, 0x1B, 0x5B, 0x31, 0x3B, 0x39, 0x31, 0x3B, 0x34, 0x39, 0x6D]);
    exit(value);
}

//> CALLS -> CRASH ENUM
#[derive(Clone)]
pub enum Code {
    Success = 0,
    UnknownIRObject = 1,
    UnexpectedEndOfIR = 2,
    UnknownGroupCode = 3,
    FailedNamedRetrieval = 4,
    FailedCast = 5,
    NoVariableOperation = 6,
    DoubleGroupAnnotation = 7,
    ImmutableModification = 8,
    NaturalCannotBeZero = 9,
    RationalDenominatorCannotBeZero = 10,
    FailedIRDecompression = 11,
    RuntimeHigherObject = 12,
    UnknownSignValue = 13,
    Todo = 255
}


//^
//^ DETAIL
//^

//> DETAIL -> SPACE
pub fn space<Type: fmt::Display>(message: Type) -> () {print(&format!(
    "SPACE: {}.",
    message
), &[0x0A, 0x1B, 0x5B, 0x30, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D])}


//^
//^ LOOKUP
//^

//> LOOKUP -> DEBUG
pub fn debug<Type: fmt::Display>(message: Type) -> () {print(&format!(
    "    DEBUG: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x35, 0x3B, 0x34, 0x39, 0x6D])}

//> LOOKUP -> ALERT
pub fn alert<Type: fmt::Display>(message: Type) -> () {print(&format!(
    "    ALERT: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x38, 0x3B, 0x35, 0x3B, 0x32, 0x30, 0x38, 0x3B, 0x34, 0x39, 0x6D])}

//> LOOKUP -> TRACE
pub fn trace<Type: fmt::Display>(message: Type) -> () {print(&format!(
    "    TRACE: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x36, 0x3B, 0x34, 0x39, 0x6D])}

//> LOOKUP -> CHORE
pub fn chore<Type: fmt::Display>(message: Type) -> () {print(&format!(
    "    CHORE: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D])}

//> LOOKUP -> CLASS
pub fn class<Type: fmt::Display>(message: Type) -> () {print(&format!(
    "    CLASS: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x32, 0x6D])}

//> LOOKUP -> POINT
pub fn point<Type: fmt::Display>(message: Type) -> () {print(&format!(
    "    POINT: {}.",
    message
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x37, 0x6D])}