//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod context;
pub mod level1;
pub mod level2;
pub mod level3;
pub mod level4;
pub mod level5;
pub mod start;
pub mod types;

//> HEAD -> LIBUTILS
use libutils::{
    array::Array,
    report::{
        Name,
        Report,
        Same
    }
};

//> HEAD -> CRATE
use crate::{
    parser::types::{
        Pointer,
        Parsed
    },
    extensor::types::{
        Object,
        DERIVATIONS
    },
    filter::{
        Responsibility,
        RESPONSIBILITIES
    },
    Interpreter,
    Resolver
};

//> HEAD -> STD
use std::collections::{
    HashSet as Set,
    HashMap as Map
};

//> HEAD -> CONTEXT
use context::Context;

//> HEAD -> START
use start::Start;

//> HEAD -> NONTERMINAL
use types::{
    Subtree,
    Item,
    NonTerminal
};

//> HEAD -> LEVEL4
use level4::{
    Level4,
    Factor
};

//> HEAD -> LEVEL5
use level5::Level5;


//^
//^ SOLVER
//^

//> SOLVER -> SOLVE
pub fn solve<'valid>(
    pool: Map<Pointer<'valid>, Set<Array<Pointer<'valid>, DERIVATIONS>>>,
    context: Option<&mut Context<'valid>>,
    filename: &'valid str,
    interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
    mut report: Report<Name<"Solver">>
) -> Option<Start<'valid>> {
    let context = match context {
        None => &mut Context::default(),
        Some(previous) => previous
    };
    return Some(build(
        pool.keys().find(|&item| {
            return if let Parsed::Object(Object::Start) = item.parsed {true} else {false}
        }).unwrap(),
        &pool,
        context,
        true,
        filename,
        interpreter,
        &mut Map::new(),
        report.to()
    )?.into_non_terminal().ok().unwrap().into_start().ok().unwrap());
}

//> SOLVER -> BUILD
fn build<'valid, 'active>(
    node: &'active Pointer<'valid>,
    pool: &'active Map<Pointer<'valid>, Set<Array<Pointer<'valid>, DERIVATIONS>>>,
    context: &mut Context<'valid>,
    write: bool,
    filename: &'valid str,
    interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
    memory: &mut Map<&'active Pointer<'valid>, Subtree<'valid>>,
    mut report: Report<Same>
) -> Option<Subtree<'valid>> {
    let writeable = if write {context} else {&mut context.clone()};
    if !write && let Some(cached) = memory.get(node).cloned() {return Some(cached)}
    if let Parsed::Token(token) = &node.parsed {return Some(Subtree::Token(token.clone()))}
    let mut children = Vec::new();
    for production in disambiguate(
        pool.get(node).unwrap(),
        pool,
        writeable,
        filename,
        interpreter,
        memory,
        report.to()
    )? {match build(
        production,
        pool,
        writeable,
        true,
        filename,
        interpreter,
        memory,
        report.to()
    )? {
        Subtree::NonTerminal(nonterminal) => children.push(Item::NonTerminal(nonterminal)),
        Subtree::Token(token) => if let Responsibility::Total = RESPONSIBILITIES[token.kind as usize] {
            children.push(Item::Token(token));
        },
        Subtree::Vec(vec) => children.extend(vec)
    }};
    let partition = match &node.parsed {
        Parsed::Object(object) => Subtree::NonTerminal(object.summon(children, writeable, report.to(), interpreter, filename)?),
        Parsed::usize(_) => Subtree::Vec(children),
        Parsed::Token(_) => unreachable!()
    };
    if !write {memory.insert(node, partition.clone());}
    return Some(partition);
}

//> SOLVER -> DISAMBIGUATE
fn disambiguate<'valid, 'active>(
    candidates: &'active Set<Array<Pointer<'valid>, DERIVATIONS>>,
    pool: &'active Map<Pointer<'valid>, Set<Array<Pointer<'valid>, DERIVATIONS>>>,
    context: &mut Context<'valid>,
    filename: &'valid str,
    interpreter: &'valid Interpreter<'valid, impl Resolver<'valid>>,
    memory: &mut Map<&'active Pointer<'valid>, Subtree<'valid>>,
    mut report: Report<Same>
) -> Option<&'active Array<Pointer<'valid>, DERIVATIONS>> {
    let mut index = 0;
    let mut candidates = Vec::from_iter(candidates);
    while candidates.len() > 1 {
        candidates.retain(|derivation| derivation.get(index).is_some());
        let mut built = Vec::new();
        let mut seen = Set::new();
        for &derivation in &candidates {
            let current = &derivation[index];
            if seen.insert(current) {built.push((current, build(
                current,
                pool,
                context,
                false,
                filename,
                interpreter,
                memory,
                report.to()
            )?));}
        }
        if built.len() > 1 {
            let winner = choose(&built, context);
            candidates.retain(|derivation| &derivation[index] == winner);
        }
        index += 1;
    };
    return Some(candidates.pop().unwrap());
}

//> SOLVER -> CHOOSE
fn choose<'valid, 'active>(
    possibilities: &Vec<(&'active Pointer<'valid>, Subtree<'valid>)>, 
    context: &mut Context<'valid>
) -> &'active Pointer<'valid> {match possibilities.as_slice() {
    [(first, Subtree::NonTerminal(NonTerminal::Level4(Level4::Factor(Factor {
        value: Level5::Variable(variable),
        ..
    })))), (second, Subtree::NonTerminal(NonTerminal::Level4(Level4::Factor(Factor {
        value: Level5::Call(_),
        ..
    }))))] => if context.functions.contains(variable.name) {second} else {first},
    _ => unreachable!()
}}