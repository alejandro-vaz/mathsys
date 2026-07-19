//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod kind;
pub mod position;
pub mod token;

use core::str;
use std::num::NonZero;

//> HEAD -> LIBUTILS
use libutils::active_reporting::Report;

//> HEAD -> TOKEN
use token::Token;

//> HEAD -> KIND
use kind::Kind;

//> HEAD -> CRATE
use crate::{
    failure::Failure,
    Interpreter,
    Resolver
};

//> HEAD -> POSITION
use position::Position;


//^
//^ TOKENIZER
//^

//> TOKENIZER -> FUNCTION
pub fn tokenize<'input>(
    content: &'input str, 
    report: Report<"Tokenizer">,
    interpreter: &'input Interpreter<'input, impl Resolver<'input>>
) -> Vec<Token<'input>> {
    let bytes = content.as_bytes();
    let pointer = bytes.as_ptr();
    let mut tokens = Vec::new();
    let mut position = Position {..};
    loop {
        let (kind, amount) = match scan(bytes, &position.cursor) {
            Some(tuple) => tuple,
            None => (interpreter.critical)(Failure::UnknownToken {
                line: position.line,
                column: position.column
            }, &*report)
        };
        tokens.push(Token {
            kind: kind,
            value: unsafe {str::from_raw_parts(pointer.add(position.cursor), amount)}
        });
        if let Kind::ENDOFFILE = kind {break}
        match kind {
            Kind::NEWLINES => {
                position.column = NonZero::new(1).unwrap();
                position.line = position.line.checked_add(amount).unwrap();
            },
            _ => position.column = position.column.checked_add(amount).unwrap()
        }
        position.cursor += amount;
    }
    return tokens;
}

//> TOKENIZER -> SCAN
fn scan(bytes: &[u8], cursor: &usize) -> Option<(Kind, usize)> {
    return match bytes.get(*cursor) {
        Some(b' ') => {
            let mut amount = 1;
            while let Some(b' ') = bytes.get(cursor + amount) {amount += 1}
            Some((Kind::SPACES, amount))
        },
        Some(b'\n') => {
            let mut amount = 1;
            while let Some(b'\n') = bytes.get(cursor + amount) {amount += 1}
            Some((Kind::NEWLINES, amount))
        },
        Some(b'#') => {
            let mut amount = 1;
            while let Some(byte) = bytes.get(cursor + amount) && *byte != b'\n' {amount += 1}
            Some((Kind::COMMENT, amount))
        },
        Some(b'"') => {
            let mut amount = 1;
            while let Some(byte) = bytes.get(cursor + amount) && *byte != b'"' {amount += 1}
            amount += 1;
            Some((Kind::MODULE, amount))
        },
        Some(b'?') => Some((Kind::UNDEFINED, 1)),
        Some(b'^') => Some((Kind::EXPONENTIATION, 1)),
        Some(b'|') => Some((Kind::PIPE, 1)),
        Some(b',') => Some((Kind::COMMA, 1)),
        Some(b'(') => Some((Kind::OPEN, 1)),
        Some(b')') => Some((Kind::CLOSE, 1)),
        Some(b'[') => Some((Kind::ENTER, 1)),
        Some(b']') => Some((Kind::EXIT, 1)),
        Some(b'*' | b'/') => Some((Kind::OPERATOR, 1)),
        Some(b'+') => Some((Kind::SIGN, 1)),
        Some(b'-') => if let Some(b'>') = bytes.get(cursor + 1) {Some((Kind::TO, 2))} else {
            Some((Kind::SIGN, 1))
        }
        Some(b':') if let Some(b'=') = bytes.get(cursor + 1) => Some((Kind::DEFINITION, 2)),
        Some(b'=') => Some((Kind::EQUALITY, 1)),
        Some(b'0'..=b'9') => {
            let mut amount = 1;
            let mut decimal = false;
            while let Some(byte @ (b'_' | b'0'..=b'9' | b'.')) = bytes.get(cursor + amount) {
                if *byte == b'.' {
                    if decimal {break} else {decimal = true}
                }
                amount += 1;
            }
            Some((if decimal {Kind::RATIONAL} else {Kind::NUMBER}, amount))
        },
        Some(b'u') if let Some(b's') = bytes.get(cursor + 1) && let Some(b'e') = bytes.get(cursor + 2) => Some((Kind::USE, 3)),
        Some(b'l') if let Some(b'i') = bytes.get(cursor + 1) && let Some(b'm') = bytes.get(cursor + 2) => Some((Kind::LIMIT, 3)),
        Some(b'i') if let Some(b'n') = bytes.get(cursor + 1) && let Some(b'f') = bytes.get(cursor + 2) => Some((Kind::INFINITE, 3)),
        Some(b'o') if let Some(b'f') = bytes.get(cursor + 1) => Some((Kind::OF, 2)),
        Some(b'A'..=b'Z' | b'a'..=b'z' | b'%' | b'$') => {
            let mut amount = 1;
            while let Some(
                b'A'..=b'Z' 
                | b'a'..=b'z' 
                | b'%' 
                | b'$'
            ) = bytes.get(cursor + amount) {amount += 1}
            Some((Kind::IDENTIFIER, amount))
        }
        Some(_) => None,
        None => Some((Kind::ENDOFFILE, 0))
    }
}