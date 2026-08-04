//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::tokenizer::token::Token;

//> HEAD -> SUPER
use super::rule::Rule;

//> HEAD -> CORE
use core::range::RangeInclusive;


//^
//^ PARSED
//^

//> PARSED -> ENUM
#[derive(PartialEq, Eq, Hash)]
pub enum Parsed<'valid> {
    Terminal {
        token: &'valid Token<'valid>,
        /// this could theoretically be removed, maybe in the future
        index: usize
    },
    NonTerminal {
        rule: &'static Rule,
        span: RangeInclusive<usize>,
    }
}