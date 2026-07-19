//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::tokenizer::{
    token::Token,
    kind::Kind
};

//> HEAD -> STRUM
use strum::EnumCount;


//^
//^ RESPONSIBILITY
//^

//> RESPONSIBILITY -> ENUM
#[derive(Clone, Copy)]
pub enum Responsibility {
    Null,
    Structural,
    Total
}

//> RESPONSIBILITY -> MAP
pub static RESPONSIBILITIES: [Responsibility; Kind::COUNT] = {
    let mut map = [Responsibility::Null; Kind::COUNT];
    map[Kind::UNDEFINED as usize] = Responsibility::Structural;
    map[Kind::LIMIT as usize] = Responsibility::Structural;
    map[Kind::PIPE as usize] = Responsibility::Structural;
    map[Kind::TO as usize] = Responsibility::Structural;
    map[Kind::OF as usize] = Responsibility::Structural;
    map[Kind::INFINITE as usize] = Responsibility::Structural;
    map[Kind::USE as usize] = Responsibility::Structural;
    map[Kind::IDENTIFIER as usize] = Responsibility::Total;
    map[Kind::EXPONENTIATION as usize] = Responsibility::Structural;
    map[Kind::RATIONAL as usize] = Responsibility::Total;
    map[Kind::NUMBER as usize] = Responsibility::Total;
    map[Kind::DEFINITION as usize] = Responsibility::Structural;
    map[Kind::EQUALITY as usize] = Responsibility::Structural;
    map[Kind::OPERATOR as usize] = Responsibility::Total;
    map[Kind::SIGN as usize] = Responsibility::Total;
    map[Kind::OPEN as usize] = Responsibility::Structural;
    map[Kind::CLOSE as usize] = Responsibility::Structural;
    map[Kind::ENTER as usize] = Responsibility::Structural;
    map[Kind::COMMA as usize] = Responsibility::Structural;
    map[Kind::EXIT as usize] = Responsibility::Structural;
    map[Kind::SPACES as usize] = Responsibility::Null;
    map[Kind::NEWLINES as usize] = Responsibility::Structural;
    map[Kind::MODULE as usize] = Responsibility::Total;
    map[Kind::COMMENT as usize] = Responsibility::Null;
    map[Kind::ENDOFFILE as usize] = Responsibility::Structural;
    map
};


//^
//^ FILTER
//^

//> FILTER -> FUNCTION
pub fn filter<'valid>(
    tokens: Vec<Token<'valid>>
) -> Vec<Token<'valid>> {return tokens.into_iter().filter(|token| {
    if let Kind::SPACES | Kind::COMMENT = token.kind {false} else {true}
}).collect()}