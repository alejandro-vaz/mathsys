//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::tokenizer::token::Token;

//> HEAD -> SUPER
use super::rule::Rule;

//> HEAD -> CORE
use core::range::Range;


//^
//^ PARSED
//^

//> PARSED -> ENUM
#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)] // rm debug
pub enum Parsed<'valid> {
    Terminal {
        token: &'valid Token<'valid>,
        index: usize
    },
    NonTerminal {
        rule: &'static Rule,
        span: Range<usize>,
    }
}