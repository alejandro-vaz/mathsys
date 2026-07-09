//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod token;

//> HEAD -> LIBUTILS
use libutils::report::{
    Name, 
    Report
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
pub fn tokenize<'input>(content: &'input str, report: Report<Name<"Tokenizer">>) -> Option<Vec<Token<'input>>> {
    let bytes = content.as_bytes();
    let mut tokens = Vec::new();
    let mut cursor = 0usize;
    loop {
        let Some((kind, amount)) = scan(bytes, cursor) else {report.issue(Failure::UnknownToken)?};
        tokens.push(Token::new(cursor, amount, kind, &content));
        if let Kind::ENDOFFILE = kind {break}
        cursor += amount;
    }
    return Some(tokens);
}

//> TOKENIZER -> SCAN
fn scan(bytes: &[u8], cursor: usize) -> Option<(Kind, usize)> {
    let length = bytes.len();
    if cursor >= length {return Some((Kind::ENDOFFILE, 0))}
    let first = unsafe {bytes.get_unchecked(cursor)};
    return match first {
        b' ' => {
            let mut amount = 1;
            while let Some(b' ') = bytes.get(amount + cursor) {amount += 1}
            Some((Kind::SPACES, amount))
        },
        b'\n' => {
            let mut amount = 1;
            while let Some(b'\n') = bytes.get(amount + cursor) {amount += 1}
            Some((Kind::NEWLINES, amount))
        },
        b'#' => {
            let mut amount = 1;
            while let Some(byte if *byte != b'\n') = bytes.get(amount + cursor) {amount += 1}
            Some((Kind::COMMENT, amount))
        },
        b'"' => {
            let mut amount = 1;
            while let Some(byte if *byte != b'"') = bytes.get(amount + cursor) {amount += 1}
            Some((Kind::MODULE, amount))
        },
        b'?' => Some((Kind::UNDEFINED, 1)),
        b'^' => Some((Kind::EXPONENTIATION, 1)),
        b'|' => Some((Kind::PIPE, 1)),
        b',' => Some((Kind::COMMA, 1)),
        b'(' => Some((Kind::OPEN, 1)),
        b')' => Some((Kind::CLOSE, 1)),
        b'[' => Some((Kind::ENTER, 1)),
        b']' => Some((Kind::EXIT, 1)),
        b'*' | b'/' => Some((Kind::OPERATOR, 1)),
        b'+' => Some((Kind::SIGN, 1)),
        b'-' => if let Some(b'>') = bytes.get(cursor + 1) {Some((Kind::TO, 2))} else {Some((Kind::SIGN, 1))}
        b':' if let Some(b'=') = bytes.get(cursor + 1) => Some((Kind::DEFINITION, 2)),
        b'=' => Some((Kind::EQUALITY, 1)),
        b'0'..=b'9' => {
            let mut amount = 1;
            let mut decimal = false;
            loop {
                let Some(byte) = bytes.get(cursor + amount) else {break Some((if decimal {Kind::RATIONAL} else {Kind::NUMBER}, amount))};
                match byte {
                    b'_' | b'0'..=b'9' => amount += 1,
                    b'.' => if decimal {break Some((Kind::RATIONAL, amount))} else {decimal = true; amount += 1},
                    _ => break Some((if decimal {Kind::RATIONAL} else {Kind::NUMBER}, amount)) 
                }
            }
        },
        b'u' if let Some(b's') = bytes.get(cursor + 1) && let Some(b'e') = bytes.get(cursor + 2) => Some((Kind::USE, 3)),
        b'l' if let Some(b'i') = bytes.get(cursor + 1) && let Some(b'm') = bytes.get(cursor + 2) => Some((Kind::LIMIT, 3)),
        b'i' if let Some(b'n') = bytes.get(cursor + 1) && let Some(b'f') = bytes.get(cursor + 2) => Some((Kind::INFINITE, 3)),
        b'o' if let Some(b'f') = bytes.get(cursor + 1) => Some((Kind::OF, 2)),
        b'A'..=b'z' | b'%' | b'$' => {
            let mut amount = 1;
            while let Some(b'A'..=b'z' | b'%' | b'$') = bytes.get(cursor + amount) {amount += 1}
            Some((Kind::IDENTIFIER, amount))
        }
        _ => None
    }
}