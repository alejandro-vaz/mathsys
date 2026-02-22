//^
//^ HEAD
//^

//> HEAD -> MODULES
pub(super) mod context;
pub(super) mod nonterminal;

//> HEAD -> PRELUDE
use crate::prelude::{
    FastMap, 
    FastSet, 
    SmallVec
};

//> HEAD -> LOCAL
use self::{
    nonterminal::{
        Partition, 
        Object, 
        NonTerminal, 
        Item
    },
    context::Context,
    super::{
        issues::Issue,
        tokenizer::token::{
            ORDER, 
            Responsibility
        },
        parser::{
            Backpointer, 
            MINPOINTERS, 
            Part
        },
        syntax::{
            Start,
            level4::{
                Level4,
                Factor
            },
            level5::Level5
        }
    }
};


//^
//^ SOLVER
//^

//> SOLVER -> STRUCT
pub(super) struct Solver {} impl Solver {
    pub(super) const fn new() -> Self {return Solver {}}
    pub(super) fn run<'resolving>(&self, pool: &FastMap<Backpointer<'resolving>, FastSet<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>) -> Result<Start, Issue> {
        let Partition::NonTerminal(NonTerminal::Start(start)) = self.build(pool, pool.iter().map(|item| item.0).find(|backpointer| if let Part::NonTerminal(Object::Start) = backpointer.symbol {true} else {false}).ok_or(Issue::SyntaxError)?, &mut Context::new(), true) else {return Err(Issue::SyntaxError)};
        return Ok(start);
    }
    fn build<'resolving, 'active>(&self, pool: &'active FastMap<Backpointer<'resolving>, FastSet<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>, node: &'active Backpointer<'resolving>, context: &mut Context, write: bool) -> Partition<'resolving> {
        return if let Part::Token(token) = &node.symbol {Partition::Token(token.clone())} else {
            let mut children = Vec::new();
            for item in self.solve(&mut pool.get(node).unwrap().clone().into_iter().collect::<Vec<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>(), pool, context, &node.symbol) {match self.build(pool, &item, context, true) {
                Partition::Internal(items) => children.extend(items),
                Partition::NonTerminal(item) => children.push(Item::NonTerminal(item)),
                Partition::Token(token) => if let Responsibility::Total = ORDER.get(&token.kind).unwrap().1 {children.push(Item::Token(token))}
            }}
            match node.symbol.clone() {
                Part::NonTerminal(object) => Partition::NonTerminal(object.summon(children, if write {Some(context)} else {None})),
                Part::Internal(code) => Partition::Internal(children),
                other => unreachable!()
            }
        };
    }
    fn solve<'resolving>(&self, candidates: &mut Vec<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>, pool: &FastMap<Backpointer<'resolving>, FastSet<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>, context: &mut Context, building: &Part) -> SmallVec<[Backpointer<'resolving>; MINPOINTERS]> {for index in 0.. {
        match candidates.len() {
            0 => panic!(),
            1 => return candidates.pop().unwrap(),
            _ => ()
        };
        candidates.retain(|derivation| derivation.get(index).is_some());
        let mut symbols = Vec::new();
        for derivation in candidates.iter() {
            let pointer = &derivation[index];
            if !symbols.iter().any(|thing| *thing == pointer) {symbols.push(pointer)}
        }
        let built = symbols.into_iter().map(|symbol| (symbol, self.build(pool, symbol, context, false))).collect::<Vec<(&Backpointer, Partition)>>();
        let mut winner = &built[0];
        for contender in built.iter().skip(1) {
            let Partition::NonTerminal(first) = &winner.1 else {panic!()};
            let Partition::NonTerminal(second) = &contender.1 else {panic!()};
            if self.choose(first, second, context, false, building) {winner = contender}
        };
        let cloned = winner.0.clone();
        candidates.retain(|derivation| derivation[index] == cloned);
    }; unreachable!()}
    fn choose(&self, first: &NonTerminal, second: &NonTerminal, context: &Context, reincident: bool, building: &Part) -> bool {return match (building, first, second) {
        (at, NonTerminal::Level4(Level4::Factor(Factor {
            value: Level5::Variable(variable),
            ..
        })), NonTerminal::Level4(Level4::Factor(Factor {
            value: Level5::Call(call),
            ..
        }))) => context.isFn(variable),
        other => return if reincident {panic!("{first:?} && {second:?}")} else {!self.choose(second, first, context, true, building)}
    }}
}