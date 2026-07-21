//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::tokenizer::token::Token;


//^
//^ FILTER
//^

//> FILTER -> FUNCTION
pub fn filter<'valid>(
    tokens: Vec<Token<'valid>>
) -> Vec<Token<'valid>> {return tokens.into_iter().filter(|token| {
    !token.responsibility().is_null()
}).collect()}