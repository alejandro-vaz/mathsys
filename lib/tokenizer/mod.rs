//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod token;

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Report,
    systemstd::System
};

//> HEAD -> TOKEN
use token::{
    Token,
    Kind
};

//> HEAD -> CRATE
use crate::failure::Failure;


//^
//^ TOKENIZER
//^

//> TOKENIZER -> FUNCTION
pub fn tokenize<'input>(content: &'input str, report: Report<"Tokenizer">) -> Vec<Token<'input>> {
    let bytes = content.as_bytes();
    let mut tokens = Vec::new();
    let mut cursor = 0usize;
    loop {
        let (kind, amount) = System::expect(scan(bytes, cursor), &*report);
        tokens.push(Token {
            kind: kind,
            value: &content[cursor .. cursor + amount]
        });
        if let Kind::ENDOFFILE = kind {break}
        cursor += amount;
    }
    return tokens;
}

//> TOKENIZER -> SCAN
fn scan(bytes: &[u8], cursor: usize) -> Result<(Kind, usize), Failure<'_>> {
    return match bytes.get(cursor) {
        Some(b' ') => {
            let mut amount = 1;
            while let Some(b' ') = bytes.get(cursor + amount) {amount += 1}
            Ok((Kind::SPACES, amount))
        },
        Some(b'\n') => {
            let mut amount = 1;
            while let Some(b'\n') = bytes.get(cursor + amount) {amount += 1}
            Ok((Kind::NEWLINES, amount))
        },
        Some(b'#') => {
            let mut amount = 1;
            while let Some(byte) = bytes.get(cursor + amount) && *byte != b'\n' {amount += 1}
            Ok((Kind::COMMENT, amount))
        },
        Some(b'"') => {
            let mut amount = 1;
            while let Some(byte) = bytes.get(cursor + amount) && *byte != b'"' {amount += 1}
            amount += 1;
            Ok((Kind::MODULE, amount))
        },
        Some(b'?') => Ok((Kind::UNDEFINED, 1)),
        Some(b'^') => Ok((Kind::EXPONENTIATION, 1)),
        Some(b'|') => Ok((Kind::PIPE, 1)),
        Some(b',') => Ok((Kind::COMMA, 1)),
        Some(b'(') => Ok((Kind::OPEN, 1)),
        Some(b')') => Ok((Kind::CLOSE, 1)),
        Some(b'[') => Ok((Kind::ENTER, 1)),
        Some(b']') => Ok((Kind::EXIT, 1)),
        Some(b'*' | b'/') => Ok((Kind::OPERATOR, 1)),
        Some(b'+') => Ok((Kind::SIGN, 1)),
        Some(b'-') => if let Some(b'>') = bytes.get(cursor + 1) {Ok((Kind::TO, 2))} else {Ok((Kind::SIGN, 1))}
        Some(b':') if let Some(b'=') = bytes.get(cursor + 1) => Ok((Kind::DEFINITION, 2)),
        Some(b'=') => Ok((Kind::EQUALITY, 1)),
        Some(b'0'..=b'9') => {
            let mut amount = 1;
            let mut decimal = false;
            while let Some(byte @ (b'_' | b'0'..=b'9' | b'.')) = bytes.get(cursor + amount) {
                if *byte == b'.' {
                    if decimal {break} else {decimal = true}
                }
                amount += 1;
            }
            Ok((if decimal {Kind::RATIONAL} else {Kind::NUMBER}, amount))
        },
        Some(b'u') if let Some(b's') = bytes.get(cursor + 1) && let Some(b'e') = bytes.get(cursor + 2) => Ok((Kind::USE, 3)),
        Some(b'l') if let Some(b'i') = bytes.get(cursor + 1) && let Some(b'm') = bytes.get(cursor + 2) => Ok((Kind::LIMIT, 3)),
        Some(b'i') if let Some(b'n') = bytes.get(cursor + 1) && let Some(b'f') = bytes.get(cursor + 2) => Ok((Kind::INFINITE, 3)),
        Some(b'o') if let Some(b'f') = bytes.get(cursor + 1) => Ok((Kind::OF, 2)),
        Some(b'A'..=b'Z' | b'a'..=b'z' | b'%' | b'$') => {
            let mut amount = 1;
            while let Some(b'A'..=b'Z' | b'a'..=b'z' | b'%' | b'$') = bytes.get(cursor + amount) {amount += 1}
            Ok((Kind::IDENTIFIER, amount))
        }
        None => Ok((Kind::ENDOFFILE, 0)),
        _ => Err(Failure::UnknownToken {
            index: cursor
        })
    }
}