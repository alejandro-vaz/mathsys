//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod context;
pub mod item;
pub mod level1;
pub mod level2;
pub mod level3;
pub mod level4;
pub mod level5;
pub mod nonterminal;
pub mod spawn;
pub mod start;
pub mod subtree;

//> HEAD -> LIBUTILS
use libutils::{
    stack_array::Array,
    active_reporting::Report,
    systemio::SystemIO
};

//> HEAD -> CRATE
use crate::{
    parser::{
        parsed::Parsed,
        pointer::Pointer,
        data::AMBIGUITY
    },
    grammar::{
        object::Object,
        constants::DERIVATIONS,
        rule::Rule
    },
    failure::Failure,
    syntax::{
        Start,
        level4::{
            Level4,
            Factor
        },
        level5::Level5
    }
};

//> HEAD -> STD
use std::collections::{
    HashSet as Set,
    HashMap as Map
};

//> HEAD -> CONTEXT
use context::Context;

//> HEAD -> ITEM
use item::Item;

//> HEAD -> NONTERMINAL
use nonterminal::NonTerminal;

//> HEAD -> SUBTREE
use subtree::Subtree;


//^
//^ SOLVER
//^

//> SOLVER -> SOLVE
pub fn solve<'valid>(
    pool: Map<Pointer<'valid>, Array<Array<Pointer<'valid>, DERIVATIONS>, AMBIGUITY>>,
    context: &mut Context<'valid>,
    filename: &'valid str,
    systemio: &'valid SystemIO<Failure<'valid>>,
    resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
    mut report: Report<"Solver">
) -> Start<'valid> {return build(
    pool.keys().find(|&item| {
        return if let Parsed::Rule(Rule::Object(Object::Start)) = item.parsed {true} else {false}
    }).unwrap(),
    &pool,
    context,
    filename,
    systemio,
    resolver,
    report.to()
).into_item().unwrap().into_non_terminal().unwrap().into_start().unwrap()}

//> SOLVER -> BUILD
fn build<'valid, 'active>(
    node: &'active Pointer<'valid>,
    pool: &'active Map<
        Pointer<'valid>, 
        Array<Array<Pointer<'valid>, DERIVATIONS>, AMBIGUITY>
    >,
    context: &mut Context<'valid>,
    filename: &'valid str,
    systemio: &'valid SystemIO<Failure<'valid>>,
    resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
    mut report: Report<"">
) -> Subtree<'valid> {
    let rule = match node.parsed {
        Parsed::Rule(rule) => rule,
        Parsed::Token(token) => return Subtree::Item(Item::Token(token))
    };
    let mut children = Vec::new();
    for production in disambiguate(
        pool.get(node).unwrap(),
        pool,
        context,
        filename,
        systemio,
        resolver,
        report.to()
    ) {match build(
        production,
        pool,
        context,
        filename,
        systemio,
        resolver,
        report.to()
    ) {
        Subtree::Item(Item::NonTerminal(nonterminal)) => children.push(Item::NonTerminal(nonterminal)),
        Subtree::Item(Item::Token(token)) => if token.responsibility().is_full() {
            children.push(Item::Token(token))
        },
        Subtree::Vec(vec) => children.extend(vec)
    }};
    return match rule {
        Rule::Object(object) => Subtree::Item(Item::NonTerminal(object.summon(
            children, 
            context, 
            report, 
            systemio, 
            resolver, 
            filename
        ))),
        Rule::usize(_) => Subtree::Vec(children)
    }
}

//> SOLVER -> DISAMBIGUATE
fn disambiguate<'valid, 'active>(
    candidates: &'active Array<Array<Pointer<'valid>, DERIVATIONS>, AMBIGUITY>,
    pool: &'active Map<
        Pointer<'valid>, 
        Array<Array<Pointer<'valid>, DERIVATIONS>, AMBIGUITY>
    >,
    context: &mut Context<'valid>,
    filename: &'valid str,
    systemio: &'valid SystemIO<Failure<'valid>>,
    resolver: &'valid fn(&'valid str, Report<"Resolver">) -> &'valid [u8],
    mut report: Report<"">
) -> &'active Array<Pointer<'valid>, DERIVATIONS> {
    let mut candidates = Vec::from_iter(candidates.into_iter().map(|candidate| (
        candidate, 
        context.clone()
    )));
    for index in 0.. {
        if candidates.len() == 1 {break}
        candidates.retain(|derivation| derivation.0.get(index).is_some());
        let mut built = Vec::new();
        let mut seen = Set::new();
        for (derivation, context) in &mut candidates {
            let current = &derivation[index];
            if seen.insert(current) {built.push((current, build(
                current,
                pool,
                context,
                filename,
                systemio,
                resolver,
                report.to()
            ), &*context));}
        }
        built.sort_by(|first, second| first.1.cmp(&second.1));
        let winner = choose(&built, context);
        candidates.retain(|(derivation, _)| &derivation[index] == winner);
    };
    return candidates.pop().unwrap().0;
}

//> SOLVER -> CHOOSE
fn choose<'valid, 'active>(
    possibilities: &Vec<(
        &'active Pointer<'valid>, 
        Subtree<'valid>,
        &Context<'valid>
    )>, 
    context: &Context<'valid>
) -> &'active Pointer<'valid> {match possibilities.as_slice() {
    [(first, Subtree::Item(Item::NonTerminal(NonTerminal::Level4(Level4::Factor(Factor {
        value: Level5::Variable(variable),
        ..
    })))), _), (second, Subtree::Item(Item::NonTerminal(NonTerminal::Level4(Level4::Factor(Factor {
        value: Level5::Call(_),
        ..
    })))), _)] => if context.functions.contains(variable.name) {second} else {first},
    [(pointer, _, _)] => pointer,
    _ => unreachable!()
}}