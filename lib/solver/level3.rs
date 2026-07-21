//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::{
    spawn::Spawn,
    item::Item,
    nonterminal::NonTerminal,
    context::Context
};

//> HEAD -> CRATE
use crate::{
    tokenizer::token::Token,
    failure::Failure,
    syntax::level3::{
        Level3,
        Term
    }
};

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Report,
    systemio::SystemIO
};


//^
//^ 3ºLEVEL
//^

//> 3ºLEVEL -> TERM
impl<'valid> Spawn<'valid> for Term<'valid> {
    fn spawn(
        children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<"">, 
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {
        let mut numerator = Vec::new();
        let mut denominator = Vec::new();
        let mut location = true;
        for child in children {match child {
            Item::Token(Token::OPERATOR {multiplication}) => location = multiplication,
            Item::NonTerminal(NonTerminal::Level4(level4)) => (if location {&mut numerator} else {&mut denominator}).push(level4),
            _ => unreachable!()
        }};
        return NonTerminal::Level3(Level3::Term(Self {
            numerator: numerator,
            denominator: denominator
        }));
    }
}