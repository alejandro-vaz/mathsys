//^
//^ HEAD
//^

//> HEAD -> MODULES
pub(super) mod context;
pub(super) mod nonterminal;

//> HEAD -> PRELUDE
use crate::prelude::{
    Map, 
    Set, 
    SmallVec,
    Time
};

//> HEAD -> LOCAL
use self::{
    nonterminal::{
        Partition, 
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
            MINPOINTERS,
            state::Backpointer,
            grammar::Part
        },
        syntax::{
            Start,
            level4::{
                Level4,
                Factor
            },
            level5::Level5,
            object::Object
        }
    }
};


//^
//^ SOLVER
//^

//> SOLVER -> STRUCT
pub(super) struct Solver {} impl Solver {
    pub(super) const fn new() -> Self {return Solver {}}
    pub(super) fn run<'resolving>(
        &self, 
        pool: &Map<Backpointer<'resolving>, Set<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>, 
        context: &mut Context, 
        settings: &Settings
    ) -> Result<Start, Issue> {
        let time = Time::now();
        let Partition::NonTerminal(NonTerminal::Start(start)) = self.build(pool, pool.iter().map(|item| item.0).find(|backpointer| if let Part::NonTerminal(Object::Start) = backpointer.symbol {true} else {false}).ok_or(Issue::SyntaxError)?, context, true, settings, &mut Map::new())? else {return Err(Issue::SyntaxError)};
        println!("{:?}", time.elapsed());
        println!("{}", pool.len());
        return Ok(start);
    }
    fn build<'resolving, 'active>(
        &self, 
        pool: &'active Map<Backpointer<'resolving>, Set<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>, 
        node: &'active Backpointer<'resolving>, 
        context: &mut Context, 
        write: bool, 
        settings: &Settings,
        memory: &mut Map<&'active Backpointer<'resolving>, Partition<'resolving>>
    ) -> Result<Partition<'resolving>, Issue> {
        if let Some(cached) = memory.get(node) && !write {return Ok(cached.clone())}
        return Ok(if let Part::Token(token) = &node.symbol {Partition::Token(token.clone())} else {
            let mut children = Vec::new();
            for item in self.solve(&mut pool.get(node).unwrap().iter().collect::<Vec<&SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>(), pool, context, settings, memory)? {match self.build(pool, &item, context, write, settings, memory)? {
                Partition::Internal(items) => children.extend(items),
                Partition::NonTerminal(item) => children.push(Item::NonTerminal(item)),
                Partition::Token(token) => if let Responsibility::Total = ORDER.get(&token.kind).unwrap().1 {children.push(Item::Token(token))}
            }}
            let partition = match &node.symbol {
                Part::NonTerminal(object) => {
                    let assigned = if write {context} else {&mut context.clone()};
                    Partition::NonTerminal(object.summon(children, settings, assigned)?)
                },
                Part::Internal(code) => Partition::Internal(children),
                other => unreachable!()
            };
            if !write {memory.insert(node, partition.clone());};
            partition
        });
    }
    fn solve<'resolving, 'obtained>(
        &self, 
        candidates: &mut Vec<&'obtained SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>, 
        pool: &'obtained Map<Backpointer<'resolving>, Set<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>, 
        context: &mut Context, 
        settings: &Settings,
        memory: &mut Map<&'obtained Backpointer<'resolving>, Partition<'resolving>>
    ) -> Result<&'obtained SmallVec<[Backpointer<'resolving>; MINPOINTERS]>, Issue> {for index in 0.. {
        match candidates.len() {
            0 => panic!(),
            1 => return Ok(candidates.pop().unwrap()),
            other => ()
        };
        candidates.retain(|derivation| derivation.get(index).is_some());
        let built = candidates.iter().map(|derivation| (&derivation[index], self.build(pool, &derivation[index], context, false, settings, memory))).collect::<Vec<(&Backpointer, Result<Partition, Issue>)>>();
        let mut winner = &built[0];
        for contender in built.iter().skip(1) {
            let Ok(Partition::NonTerminal(first)) = &winner.1 else {panic!()};
            let Ok(Partition::NonTerminal(second)) = &contender.1 else {panic!()};
            if if let Some(decision) = self.choose(first, second, context) {decision} else if let Some(decision) = self.choose(second, first, context) {!decision} else {panic!()} {winner = contender};
        };
        candidates.retain(|derivation| &derivation[index] == winner.0);
    }; unreachable!()}
    fn choose(&self, first: &NonTerminal, second: &NonTerminal, context: &Context) -> Option<bool> {return match (first, second) {
        (NonTerminal::Level4(Level4::Factor(Factor {
            value: Level5::Variable(variable),
            ..
        })), NonTerminal::Level4(Level4::Factor(Factor {
            value: Level5::Call(call),
            ..
        }))) => Some(context.isFn(variable)),
        other => return None
    }}
}