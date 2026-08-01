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
        position.cursor += amount;
        match token {
            Token::EndOfFile => break tokens.push(token),
            Token::Newlines => {
                position.column = unsafe {NonZero::new_unchecked(1)};
                position.line = unsafe {position.line.unchecked_add(amount)};
            },
            _ => position.column = unsafe {position.column.unchecked_add(amount)}
        }
        tokens.push(token);
    };
    return tokens;
}

//> TOKENIZER -> SCAN
fn scan<'input>(
    content: &'input [u8], 
    position: &Position,
    filename: &'input str
) -> Result<(Token<'input>, usize), Failure<'input>> {
    return match &content[position.cursor..] {
        [b' ', following @ ..] => Ok((Token::Spaces, meanwhile(following, b' ') + 1)),
        [b'\n', following @ ..] => Ok((Token::Newlines, meanwhile(following, b'\n') + 1)),
        [b'#', following @ ..] => Ok((Token::Comment, until(following, b'\n') + 1)),
        [b'"', following @ ..] => {
            let amount = delimited(following, b'"').ok_or_else(|| Failure::UnmatchedModuleDelimiter {
                filename: filename, 
                start: *position
            })? + 1;
            Ok((Token::Module {
                name: str::from_utf8(
                    &content[position.cursor .. position.cursor + amount]
                ).map_err(|error| Failure::IrregularText {
                    filename: filename,
                    starting: *position, 
                    error: error 
                })?
            }, amount))
        },
        [b'?', ..] => Ok((Token::Undefined, 1)),
        [b'^', ..] => Ok((Token::Exponentiation, 1)),
        [b'|', ..] => Ok((Token::Pipe, 1)),
        [b',', ..] => Ok((Token::Comma, 1)),
        [b'(', ..] => Ok((Token::Open, 1)),
        [b')', ..] => Ok((Token::Close, 1)),
        [b'[', ..] => Ok((Token::Enter, 1)),
        [b']', ..] => Ok((Token::Exit, 1)),
        [b'*', ..] => Ok((Token::Operator {
            multiplication: true
        }, 1)),
        [b'/', ..] => Ok((Token::Operator {
            multiplication: false
        }, 1)),
        [b'+', ..] => Ok((Token::Sign {
            positive: true
        }, 1)),
        [b'-', b'>', ..] => Ok((Token::To, 2)),
        [b'-', ..] => Ok((Token::Sign {
            positive: false
        }, 1)),
        [b':', b'=', ..] => Ok((Token::Definition, 2)),
        [b'=', ..] => Ok((Token::Equality, 1)),
        [b'0'..=b'9', following @ ..] => {
            let amount = number(following) + 1;
            Ok((Token::Number {
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
                "use" => (Token::Use, 3),
                "lim" => (Token::Limit, 3),
                "inf" => (Token::Infinite, 3),
                "of" => (Token::Of, 2),
                other => (Token::Identifier {
                    name: other
                }, amount)
            })
        },
        [] => Ok((Token::EndOfFile, 0)),
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