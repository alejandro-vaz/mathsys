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
    syntax::level2::{
        Expression,
        Level2
    }
};

//> HEAD -> CORE
use core::mem::take;

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Report,
    systemio::SystemIO
};


//^
//^ 2ºLEVEL
//^

//> 2ºLEVEL -> EXPRESSION
impl<'valid> Spawn<'valid> for Expression<'valid> {
    fn spawn(
        children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<"">, 
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {
        let mut terms = Vec::new();
        let mut current = Vec::new();
        for child in children {match child {
            Item::Token(Token::SIGN {positive}) => current.push(positive),
            Item::NonTerminal(NonTerminal::Level3(level3)) => terms.push((take(&mut current), level3)),
            _ => unreachable!()
        }};
        return NonTerminal::Level2(Level2::Expression(Self {
            terms: terms
        }));
    }
}