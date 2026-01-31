//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    crash, Code
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
    Absolute,
    Undefined,
    Rational,
    Casts
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
    0x07 => Opcode::Use,
    0x08 => Opcode::Expression,
    0x09 => Opcode::Term,
    0x0A => Opcode::Factor,
    0x0B => Opcode::Limit,
    0x0C => Opcode::Infinite,
    0x0D => Opcode::Variable,
    0x0E => Opcode::Nest,
    0x0F => Opcode::Tensor,
    0x10 => Opcode::Whole,
    0x11 => Opcode::Absolute,
    0x12 => Opcode::Undefined,
    0x13 => Opcode::Rational,
    0x14 => Opcode::Casts,
    other => crash(Code::UnknownIRObject)
}}}