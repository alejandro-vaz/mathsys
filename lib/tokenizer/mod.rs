//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod position;
pub mod responsibility;
pub mod token;

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Report,
    systemio::SystemIO
};

//> HEAD -> TOKEN
use token::Token;

//> HEAD -> CORE
use core::num::NonZero;

//> HEAD -> CRATE
use crate::failure::Failure;

//> HEAD -> POSITION
use position::Position;


//^
//^ TOKENIZER
//^

//> TOKENIZER -> FUNCTION
pub fn tokenize<'input>(
    content: &'input [u8], 
    filename: &'input str,
    systemio: &'input SystemIO<Failure<'input>>,
    report: Report<"Tokenizer">
) -> Vec<Token<'input>> {
    let mut tokens = Vec::new();
    let mut position = Position {..};
    loop {
        let (token, amount) = match scan(content, &position, filename) {
            Ok(tuple) => tuple,
            Err(failure) => (systemio.critical)(failure, &*report)
        };
        tokens.push(token);
        position.cursor += amount;
        match token {
            Token::ENDOFFILE => break,
            Token::NEWLINES => {
                position.column = unsafe {NonZero::new_unchecked(1)};
                position.line = unsafe {position.line.unchecked_add(amount)};
            },
            _ => position.column = unsafe {position.column.unchecked_add(amount)}
        }
    }
    return tokens;
}

//> TOKENIZER -> SCAN
fn scan<'input>(
    content: &'input [u8], 
    position: &Position,
    filename: &'input str
) -> Result<(Token<'input>, usize), Failure<'input>> {
    return match &content[position.cursor..] {
        [b' ', following @ ..] => Ok((Token::SPACES, meanwhile(following, b' ') + 1)),
        [b'\n', following @ ..] => Ok((Token::NEWLINES, meanwhile(following, b'\n') + 1)),
        [b'#', following @ ..] => Ok((Token::COMMENT, until(following, b'\n') + 1)),
        [b'"', following @ ..] => {
            let amount = delimited(following, b'"').ok_or_else(|| Failure::UnmatchedModuleDelimiter {
                filename: filename, 
                start: *position
            })? + 1;
            Ok((Token::MODULE {
                name: str::from_utf8(
                    &content[position.cursor .. position.cursor + amount]
                ).map_err(|error| Failure::IrregularText {
                    filename: filename,
                    starting: *position, 
                    error: error 
                })?
            }, amount))
        },
        [b'?', ..] => Ok((Token::UNDEFINED, 1)),
        [b'^', ..] => Ok((Token::EXPONENTIATION, 1)),
        [b'|', ..] => Ok((Token::PIPE, 1)),
        [b',', ..] => Ok((Token::COMMA, 1)),
        [b'(', ..] => Ok((Token::OPEN, 1)),
        [b')', ..] => Ok((Token::CLOSE, 1)),
        [b'[', ..] => Ok((Token::ENTER, 1)),
        [b']', ..] => Ok((Token::EXIT, 1)),
        [b'*', ..] => Ok((Token::OPERATOR {
            multiplication: true
        }, 1)),
        [b'/', ..] => Ok((Token::OPERATOR {
            multiplication: false
        }, 1)),
        [b'+', ..] => Ok((Token::SIGN {
            positive: true
        }, 1)),
        [b'-', b'>', ..] => Ok((Token::TO, 2)),
        [b'-', ..] => Ok((Token::SIGN {
            positive: false
        }, 1)),
        [b':', b'=', ..] => Ok((Token::DEFINITION, 2)),
        [b'=', ..] => Ok((Token::EQUALITY, 1)),
        [b'0'..=b'9', following @ ..] => {
            let amount = number(following) + 1;
            Ok((Token::NUMBER {
                value: str::from_utf8(
                    &content[position.cursor .. position.cursor + amount]
                ).map_err(|error| Failure::IrregularText {
                    filename: filename,
                    starting: *position, 
                    error: error 
                })?
            }, amount))
        }
        [b'A'..=b'Z' | b'a'..=b'z' | b'$'..=b'%', following @ ..] => {
            let amount = 1 + identifier(following);
            Ok(match str::from_utf8(
                &content[position.cursor .. position.cursor + amount]
            ).map_err(|error| Failure::IrregularText {
                filename: filename,
                starting: *position, 
                error: error 
            })? {
                "use" => (Token::USE, 3),
                "lim" => (Token::LIMIT, 3),
                "inf" => (Token::INFINITE, 3),
                "of" => (Token::OF, 2),
                other => (Token::IDENTIFIER {
                    name: other
                }, amount)
            })
        },
        [] => Ok((Token::ENDOFFILE, 0)),
        _ => Err(Failure::UnknownToken {
            filename: filename,
            position: *position
        })
    };
}

//> TOKENIZER -> DELIMITED
fn delimited(
    content: &[u8], 
    value: u8
) -> Option<usize> {
    let mut amount = 0;
    for now in content {
        amount += 1;
        if *now == value {return Some(amount)}
    }
    return None;
}

//> TOKENIZER -> UNTIL
fn until(content: &[u8], value: u8) -> usize {
    let mut amount = 0;
    for now in content {
        if *now == value {return amount};
        amount += 1;
    }
    return amount;
}

//> TOKENIZER -> MEANWHILE
fn meanwhile(content: &[u8], value: u8) -> usize {
    let mut amount = 0;
    for now in content {
        if *now != value {return amount};
        amount += 1;
    }
    return amount;
}

//> TOKENIZER -> IDENTIFIER
fn identifier(content: &[u8]) -> usize {
    let mut amount = 0;
    for now in content {
        if !matches!(now, b'A'..=b'Z' | b'a'..=b'z' | b'$'..=b'%') {return amount}
        amount += 1;
    };
    return amount;
}

//> TOKENIZER -> NUMBER
fn number(content: &[u8]) -> usize {
    let mut amount = 0;
    let mut decimal = false;
    for now in content {
        match now {
            b'.' if !decimal => {decimal = true}
            b'0'..=b'9' | b'_' => (),
            _ => break
        };
        amount += 1;
    };
    return amount;
}