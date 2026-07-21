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
    failure::Failure,
    tokenizer::token::Token,
    syntax::{
        level4::{
            Factor,
            Limit,
            Level4
        },
        level5::Level5
    }
};

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Report,
    systemio::SystemIO
};



//^
//^ 4ºLEVEL
//^

//> 4ºLEVEL -> FACTOR
impl<'valid> Spawn<'valid> for Factor<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<"">, 
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {return NonTerminal::Level4(Level4::Factor(Self {
        value: children.remove(0).into_non_terminal().unwrap().into_level5().unwrap(),
        exponent: children.pop().map(|item| item.into_non_terminal().unwrap().into_level2().unwrap())
    }))}
}

//> 4ºLEVEL -> LIMIT
impl<'valid> Spawn<'valid> for Limit<'valid> {
    fn spawn(
        children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<"">, 
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {
        let mut iterator = children.into_iter();
        let variable = iterator.next().unwrap().into_non_terminal().unwrap().into_level5().unwrap().into_variable().ok().unwrap();
        let approach = iterator.next().unwrap().into_non_terminal().unwrap().into_level2().unwrap();
        let mut next = iterator.next();
        let direction = if let Some(Item::Token(Token::SIGN {positive})) = next {next = iterator.next(); Some(positive)} else {None};
        let Some(Item::NonTerminal(NonTerminal::Level5(Level5::Nest(nest)))) = next else {panic!()};
        let exponent = if let Some(Item::NonTerminal(NonTerminal::Level2(level2))) = iterator.next() {Some(level2)} else {None};
        return NonTerminal::Level4(Level4::Limit(Self {
            variable: variable,
            approach: approach,
            direction: direction,
            nest: nest,
            exponent: exponent
        }));
    }
}