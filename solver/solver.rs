//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    FastMap, FastSet, SmallVec
};

//> HEAD -> LOCAL
use super::{
    nonterminal::{Partition, Object, NonTerminal, Item},
    super::{
        issues::Issue,
        tokenizer::tokenizer::{ORDER, Responsibility},
        parser::parser::{Backpointer, MINPOINTERS, Part},
        syntax::{
            start::Start,
            level1::Level1
        }
    }
};


//^
//^ SOLVER
//^

//> SOLVER -> STRUCT
pub struct Solver {
    opposite: bool = false
} impl Solver {
    pub fn new() -> Self {return Solver {..}}
    pub fn run<'resolving>(&mut self, pool: &FastMap<Backpointer<'resolving>, FastSet<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>) -> Result<Start, Issue> {
        let Partition::NonTerminal(NonTerminal::Start(start)) = self.select(pool, pool.iter().map(|item| item.0).find(|backpointer| if let Part::NonTerminal(Object::Start) = backpointer.symbol {true} else {false}).ok_or(Issue::SyntaxError)?) else {return Err(Issue::SyntaxError)};
        return Ok(start);
    }
    fn select<'resolving, 'active>(&mut self, pool: &'active FastMap<Backpointer<'resolving>, FastSet<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>, node: &'active Backpointer<'resolving>) -> Partition<'resolving> {
        return if let Part::Token(token) = &node.symbol {Partition::Token(token.clone())} else {
            let mut candidates = pool.get(node).unwrap().clone().into_iter().collect::<Vec<SmallVec<[Backpointer<'resolving>; MINPOINTERS]>>>();
            let mut index = 0;
            while candidates.len() > 1 {
                candidates.retain(|derivation| derivation.get(index).is_some());
                if candidates.len() <= 1 {break};
                let mut symbols = Vec::new() as Vec<&Backpointer>;
                for derivation in &candidates {
                    let pointer = &derivation[index];
                    if !symbols.iter().any(|thing| *thing == pointer) {symbols.push(pointer)}
                }
                let built = symbols.into_iter().map(|symbol| (symbol, self.select(pool, symbol))).collect::<Vec<(&Backpointer, Partition<'resolving>)>>();
                let mut winner = &built[0];
                for contender in built.iter().skip(1) {
                    let Partition::NonTerminal(first) = &winner.1 else {continue};
                    let Partition::NonTerminal(second) = &contender.1 else {continue};
                    if self.resolve(first, second) {winner = contender}
                };
                let end = winner.0.clone();
                candidates.retain(|derivation| derivation[index] == end);
                index += 1;
            }
            let derivation = candidates.pop().unwrap();
            let mut children = Vec::new();
            for item in derivation {match self.select(pool, &item) {
                Partition::Internal(items) => children.extend(items),
                Partition::NonTerminal(item) => children.push(Item::NonTerminal(item)),
                Partition::Token(token) if let Responsibility::Total = ORDER.get(&token.kind).unwrap().1 => children.push(Item::Token(token)),
                other => continue
            }}
            match &node.symbol {
                Part::NonTerminal(object) => Partition::NonTerminal(object.summon(children)),
                Part::Internal(code) => Partition::Internal(children),
                other => unreachable!()
            }
        };
    }
    fn resolve(&mut self, first: &NonTerminal, second: &NonTerminal) -> bool {return match (first, second) {
        (NonTerminal::Level1(Level1::Declaration(declaration)), NonTerminal::Level1(Level1::Equation(equation))) => false,
        other => if self.opposite {panic!("{first:?} && {second:?}")} else {
            self.opposite = true;
            let result = !self.resolve(second, first);
            self.opposite = false;
            return result;
        }
    }}  
}