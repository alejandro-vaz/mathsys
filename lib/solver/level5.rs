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
    syntax::level5::{
        Absolute,
        Variable,
        Nest,
        Level5,
        Vector,
        Number,
        Infinite,
        Undefined,
        Call
    },
    failure::Failure
};

//> HEAD -> LIBUTILS
use libutils::{
    active_reporting::Report,
    systemio::SystemIO
};


//^
//^ 5ºLEVEL
//^

//> 5ºLEVEL -> INFINITE
 impl<'valid> Spawn<'valid> for Infinite {
    fn spawn(
        _children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<"">, 
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {return NonTerminal::Level5(Level5::Infinite(Self))}
}

//> 5ºLEVEL -> VARIABLE
impl<'valid> Spawn<'valid> for Variable<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<"">, 
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {return NonTerminal::Level5(Level5::Variable(Self {
        name: children.remove(0).into_token().unwrap().into_identifier().unwrap()
    }))}
}

//> 5ºLEVEL -> NEST
impl<'valid> Spawn<'valid> for Nest<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<"">, 
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {return NonTerminal::Level5(Level5::Nest(Self {
        value: children.pop().map(|item| item.into_non_terminal().unwrap().into_level2().unwrap())
    }))}
}

//> 5ºLEVEL -> VECTOR
impl<'valid> Spawn<'valid> for Vector<'valid> {
    fn spawn(
        children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<"">, 
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {return NonTerminal::Level5(Level5::Vector(Self {
        values: children.into_iter().map(|item| item.into_non_terminal().unwrap().into_level2().unwrap()).collect()
    }))}
}

//> 5ºLEVEL -> NUMBER
impl<'valid> Spawn<'valid> for Number<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<"">, 
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {return NonTerminal::Level5(Level5::Number(Self {
        number: children.remove(0).into_token().unwrap().into_number().unwrap()
    }))}
}

//> 5ºLEVEL -> ABSOLUTE
impl<'valid> Spawn<'valid> for Absolute<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>,
        _context: &mut Context<'valid>, 
        _report: Report<"">, 
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {return NonTerminal::Level5(Level5::Absolute(Self {
        value: children.remove(0).into_non_terminal().unwrap().into_level2().unwrap()
    }))}
}

//> 5ºLEVEL -> UNDEFINED
impl<'valid> Spawn<'valid> for Undefined {
    fn spawn(
        _children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<"">, 
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {return NonTerminal::Level5(Level5::Undefined(Self))}
}

//> 5ºLEVEL -> CALL
impl<'valid> Spawn<'valid> for Call<'valid> {
    fn spawn(
        mut children: Vec<Item<'valid>>, 
        _context: &mut Context<'valid>, 
        _report: Report<"">, 
        _systemio: &'valid SystemIO<Failure<'valid>>,
        _resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
        _filename: &'valid str
    ) -> NonTerminal<'valid> {return NonTerminal::Level5(Level5::Call(Self {
        to: children.remove(0).into_non_terminal().unwrap().into_level5().unwrap().into_variable().unwrap(),
        with: children.into_iter().map(|item| item.into_non_terminal().unwrap().into_level2().unwrap()).collect()
    }))}
}