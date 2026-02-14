//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::super::solver::nonterminal::{Spawn, NonTerminal, Item};
use super::level3::Level3;


//^
//^ 2ºLEVEL
//^

//> 2ºLEVEL -> NAMESPACE
#[derive(Debug, Clone)]
pub enum Level2 {
    Expression(Expression)
}

//> 2ºLEVEL -> EXPRESSION
#[derive(Debug, Clone)]
pub struct Expression {
    signs: Vec<Vec<bool>>,
    terms: Vec<Level3>
} impl Spawn for Expression {fn summon(items: Vec<Item>) -> NonTerminal {
    let mut signs = Vec::new();
    let mut terms = Vec::new();
    let mut current = Vec::new();
    for item in items {
        if let Item::Token(token) = item {current.push(token.value == "+")}
        else {
            signs.push(current.clone());
            let Item::NonTerminal(NonTerminal::Level3(level3)) = item else {panic!()};
            terms.push(level3);
            current.clear();
        }
    }
    return NonTerminal::Level2(Level2::Expression(Self {
        signs: signs,
        terms: terms
    }));
}}