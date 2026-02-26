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
    SmallVec,
    Time
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
        Settings,
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
    pub(super) fn run<'resolving>(&self, pool: &FastMap<Backpointer<'resolving>, FastSet<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>, context: &mut Context, settings: &Settings) -> Result<Start, Issue> {
        let time = Time::now();
        let Partition::NonTerminal(NonTerminal::Start(start)) = self.build(pool, pool.iter().map(|item| item.0).find(|backpointer| if let Part::NonTerminal(Object::Start) = backpointer.symbol {true} else {false}).ok_or(Issue::SyntaxError)?, context, true, settings)? else {return Err(Issue::SyntaxError)};
        println!("{:?}", time.elapsed());
        println!("{}", pool.len());
        return Ok(start);
    }
    fn build<'resolving, 'active>(&self, pool: &'active FastMap<Backpointer<'resolving>, FastSet<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>, node: &'active Backpointer<'resolving>, context: &mut Context, write: bool, settings: &Settings) -> Result<Partition<'resolving>, Issue> {
        return Ok(if let Part::Token(token) = &node.symbol {Partition::Token(token.clone())} else {
            let mut children = Vec::new();
            for item in self.solve(&mut pool.get(node).unwrap().clone().into_iter().collect::<Vec<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>(), pool, context, settings)? {match self.build(pool, &item, context, true, settings)? {
                Partition::Internal(items) => children.extend(items),
                Partition::NonTerminal(item) => children.push(Item::NonTerminal(item)),
                Partition::Token(token) => if let Responsibility::Total = ORDER.get(&token.kind).unwrap().1 {children.push(Item::Token(token))}
            }}
            match node.symbol.clone() {
                Part::NonTerminal(object) => {
                    let assigned = if write {context} else {&mut context.clone()};
                    Partition::NonTerminal(object.summon(children, settings, assigned)?)
                },
                Part::Internal(code) => Partition::Internal(children),
                other => unreachable!()
            }
        });
    }
    fn solve<'resolving>(&self, candidates: &mut Vec<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>, pool: &FastMap<Backpointer<'resolving>, FastSet<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>, context: &mut Context, settings: &Settings) -> Result<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>, Issue> {for index in 0.. {
        match candidates.len() {
            0 => panic!(),
            1 => return Ok(candidates.pop().unwrap()),
            other => ()
        };
        candidates.retain(|derivation| derivation.get(index).is_some());
        let built = candidates.iter().map(|derivation| &derivation[index]).map(|symbol| (symbol, self.build(pool, symbol, context, false, settings))).collect::<Vec<(&Backpointer, Result<Partition, Issue>)>>();
        let mut winner = &built[0];
        for contender in built.iter().skip(1) {
            let Ok(Partition::NonTerminal(first)) = &winner.1 else {panic!()};
            let Ok(Partition::NonTerminal(second)) = &contender.1 else {panic!()};
            if if let Some(decision) = self.choose(first, second, context) {decision} else if let Some(decision) = self.choose(second, first, context) {!decision} else {panic!()} {winner = contender};
        };
        let cloned = winner.0.clone();
        candidates.retain(|derivation| derivation[index] == cloned);
    }; unreachable!()}
    fn choose(&self, first: &NonTerminal, second: &NonTerminal, context: &Context) -> Option<bool> {return Some(match (first, second) {
        (NonTerminal::Level4(Level4::Factor(Factor {
            value: Level5::Variable(variable),
            ..
        })), NonTerminal::Level4(Level4::Factor(Factor {
            value: Level5::Call(call),
            ..
        }))) => context.isFn(variable),
        other => return None
    })}
}