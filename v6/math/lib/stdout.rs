//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    write, exit, fmt, Instant, Settings
};


//^
//^ STDOUT
//^

//> STDOUT -> INSTANCE
static mut STDOUT: Stdout = Stdout::new();

//> STDOUT -> STRUCT
struct Stdout {
    cache: Vec<u8>,
    start: Option<Instant>,
    _debug: bool,
    _class: bool,
    _chore: bool,
    _trace: bool,
    _alert: bool,
    _point: bool
}

//> STDOUT -> NEW
impl Stdout {pub const fn new() -> Stdout {return Stdout {
    cache: Vec::new(),
    start: None,
    _debug: true,
    _class: true,
    _chore: true,
    _trace: true,
    _alert: true,
    _point: true
}}}

//> STDOUT -> INIT
impl Stdout {pub fn init(&mut self, settings: &Settings) -> () {
    self.start = Some(Instant::now());
    self._debug = settings.debug;
    self._class = settings.class;
    self._chore = settings.chore;
    self._trace = settings.trace;
    self._alert = settings.alert;
    self._point = settings.point;
    self.print(format!(
        "LOGIN: Running Mathsys v{}, consuming {} tokens.",
        settings.version, settings.ir.len()
    ), &[0x1B, 0x5B, 0x31, 0x3B, 0x39, 0x32, 0x3B, 0x34, 0x39, 0x6D]);
    for call in [("Debug", settings.debug), ("Class", settings.class), ("Chore", settings.chore), ("Trace", settings.trace), ("Alert", settings.alert), ("Point", settings.point)] {self.debug(format!(
        "{} calls are {}",
        call.0, if call.1 {"activated"} else {"deactivated"}
    ))}
}}
pub fn init(settings: &Settings) -> () {unsafe {STDOUT.init(settings)}}


//^
//^ PRINT
//^

//> PRINT -> FUNCTION
impl Stdout {fn print(&mut self, string: String, append: &[u8]) -> () {
    let mut bytes = Vec::with_capacity(
        append.len() + string.len() + 6
    );
    bytes.extend_from_slice(append);
    bytes.extend_from_slice(string.as_bytes());
    bytes.extend_from_slice(&[0x1B, 0x5B, 0x30, 0x6D, 0x0A]);
    self.cache.extend_from_slice(&bytes);
}}


//^
//^ CALLS
//^

//> CALLS -> CRASH
impl Stdout {pub fn crash(&mut self, code: Code) -> ! {
    let value = code.clone() as u8;
    self.print(format!(
        "CRASH: {{{value}}} {}{}.",
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
            Code::CyclicEvaluation => "Ran into runtime cyclic evaluation",
            Code::Todo => "Todo"
        },
        if let Some(instant) = self.start {format!(" ({})", instant.elapsed().as_micros())} else {
            alert("Program timer not initialized");
            String::new()
        }
    ), &[0x0A, 0x1B, 0x5B, 0x31, 0x3B, 0x39, 0x31, 0x3B, 0x34, 0x39, 0x6D]);
    self.cache.push(0x00);
    write(self.cache.as_ptr());
    exit(value);
}}
pub fn crash(code: Code) -> ! {unsafe {STDOUT.crash(code)}}

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
    CyclicEvaluation = 13,
    #[allow(unused)]
    Todo = 255
}


//^
//^ DETAIL
//^

//> DETAIL -> SPACE
impl Stdout {pub fn space<Type: fmt::Display>(&mut self, message: Type) -> () {self.print(format!(
    "SPACE: {message}{}.",
    if let Some(instant) = self.start {format!(" ({})", instant.elapsed().as_micros())} else {
        alert("Program timer not initialized");
        String::new()
    }
), &[0x0A, 0x1B, 0x5B, 0x30, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D])}}
pub fn space<Type: fmt::Display>(message: Type) -> () {unsafe {STDOUT.space(message)}}


//^
//^ LOOKUP
//^

//> LOOKUP -> DEBUG
impl Stdout {pub fn debug<Type: fmt::Display>(&mut self, message: Type) -> () {if !self._debug {return} self.print(format!(
    "    DEBUG: {message}.",
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x35, 0x3B, 0x34, 0x39, 0x6D])}}
pub fn debug<Type: fmt::Display>(message: Type) -> () {unsafe {STDOUT.debug(message)}}

//> LOOKUP -> ALERT
impl Stdout {pub fn alert<Type: fmt::Display>(&mut self, message: Type) -> () {if !self._alert {return} self.print(format!(
    "    ALERT: {message}.",
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x38, 0x3B, 0x35, 0x3B, 0x32, 0x30, 0x38, 0x3B, 0x34, 0x39, 0x6D])}}
pub fn alert<Type: fmt::Display>(message: Type) -> () {unsafe {STDOUT.alert(message)}}

//> LOOKUP -> TRACE
impl Stdout {pub fn trace<Type: fmt::Display>(&mut self, message: Type) -> () {if !self._trace {return} self.print(format!(
    "    TRACE: {message}.",
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x36, 0x3B, 0x34, 0x39, 0x6D])}}
pub fn trace<Type: fmt::Display>(message: Type) -> () {unsafe {STDOUT.trace(message)}}

//> LOOKUP -> CHORE
impl Stdout {pub fn chore<Type: fmt::Display>(&mut self, message: Type) -> () {if !self._chore {return} self.print(format!(
    "    CHORE: {message}.",
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x33, 0x3B, 0x34, 0x39, 0x6D])}}
pub fn chore<Type: fmt::Display>(message: Type) -> () {unsafe {STDOUT.chore(message)}}

//> LOOKUP -> CLASS
impl Stdout {pub fn class<Type: fmt::Display>(&mut self, message: Type) -> () {if !self._class {return} self.print(format!(
    "    CLASS: {message}.",
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x32, 0x6D])}}
pub fn class<Type: fmt::Display>(message: Type) -> () {unsafe {STDOUT.class(message)}}

//> LOOKUP -> POINT
impl Stdout {pub fn point<Type: fmt::Display>(&mut self, message: Type) -> () {if !self._point {return} self.print(format!(
    "    POINT: {message}.",
), &[0x1B, 0x5B, 0x32, 0x3B, 0x33, 0x37, 0x6D])}}
pub fn point<Type: fmt::Display>(message: Type) -> () {unsafe {STDOUT.point(message)}}