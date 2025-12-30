//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    crash,
    Code,
    fmt
};

//^
//^ OPCODE
//^

//> OPCODE -> ENUM
#[derive(Clone, Copy, PartialEq)]
pub enum Opcode {
    Continue,
    Start,
    Declaration,
    Definition,
    Annotation,
    Node,
    Equation,
    Comment,
    Use,
    Expression,
    Term,
    Factor,
    Limit,
    Infinite,
    Variable,
    Nest,
    Tensor,
    Whole,
    Absolute
}

//> OPCODE -> BUILD
impl From<u8> for Opcode {fn from(value: u8) -> Self {match value {
    0x00 => Opcode::Continue,
    0x01 => Opcode::Start,
    0x02 => Opcode::Declaration,
    0x03 => Opcode::Definition,
    0x04 => Opcode::Annotation,
    0x05 => Opcode::Node,
    0x06 => Opcode::Equation,
    0x07 => Opcode::Comment,
    0x08 => Opcode::Use,
    0x09 => Opcode::Expression,
    0x0A => Opcode::Term,
    0x0B => Opcode::Factor,
    0x0C => Opcode::Limit,
    0x0D => Opcode::Infinite,
    0x0E => Opcode::Variable,
    0x0F => Opcode::Nest,
    0x10 => Opcode::Tensor,
    0x11 => Opcode::Whole,
    0x12 => Opcode::Absolute,
    other => crash(Code::UnknownIRObject)
}}}

//> OPCODE -> DISPLAY
impl fmt::Display for Opcode {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
    "{}",
    match self {
        Opcode::Continue => "",
        Opcode::Start => "_Start",
        Opcode::Declaration => "_Declaration",
        Opcode::Definition => "_Definition",
        Opcode::Annotation => "_Annotation",
        Opcode::Node => "_Node",
        Opcode::Equation => "_Equation",
        Opcode::Comment => "_Comment",
        Opcode::Use => "_Use",
        Opcode::Expression => "_Expression",
        Opcode::Term => "_Term",
        Opcode::Factor => "_Factor",
        Opcode::Limit => "_Limit",
        Opcode::Infinite => "_Infinite",
        Opcode::Variable => "_Variable",
        Opcode::Nest => "_Nest",
        Opcode::Tensor => "_Tensor",
        Opcode::Whole => "_Whole",
        Opcode::Absolute => "_Absolute"
    }
)}}