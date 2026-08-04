//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod action;
pub mod automaton;
pub mod bnf;
pub mod closure;
pub mod constants;
pub mod ebnf;
pub mod forest;
pub mod goto;
pub mod grammar;
pub mod machine;
pub mod object;
pub mod production;
pub mod rule;
pub mod symbol;
pub mod tables;

//> HEAD -> CRATE
use crate::tokenizer::token::Token;

//> HEAD -> MACHINE
use machine::Machine;


//^
//^ PARSER
//^

//> PARSER -> FUNCTION6
pub fn parse<'input>(tokens: &'input Vec<Token<'input>>) -> forest::Forest<'input> {
    let mut machine = Machine::default();
    for token in tokens {
        machine.pass(token);
        machine.advance();
    }
    machine.pass(&Token::EndOfFile);
    machine.advance();
    return machine.finish();
}


//^
//^ TESTS
//^

#[cfg(test)]
mod tests {
    use std::collections::HashSet as Set;

    use super::{
        forest::NodeLabel,
        parse
    };
    use crate::tokenizer::token::Token;

    #[test]
    fn builds_a_complete_packed_forest() -> () {
        let tokens = Vec::from([
            Token::Identifier {name: "a"},
            Token::Sign {positive: true},
            Token::Identifier {name: "b"},
            Token::Operator {multiplication: true},
            Token::Identifier {name: "c"}
        ]);
        let forest = parse(&tokens);
        assert!(!forest.roots().is_empty());
        assert!(forest.packed_node_count() > tokens.len());
        let terminals = forest.nodes().iter().filter(|node| {
            matches!(node.label(), NodeLabel::Terminal(_) | NodeLabel::EndOfFile)
        }).count();
        assert_eq!(terminals, tokens.len() + 1);
        for root in forest.roots() {
            assert_eq!(forest.node(*root).left_extent(), 0);
            assert_eq!(forest.node(*root).right_extent(), tokens.len() + 1);
        }
    }

    #[test]
    fn keeps_packed_family_extents_consistent() -> () {
        let tokens = Vec::from([
            Token::Identifier {name: "a"},
            Token::Identifier {name: "b"},
            Token::Identifier {name: "c"},
            Token::Identifier {name: "d"}
        ]);
        let forest = parse(&tokens);
        assert!(!forest.roots().is_empty());
        for node in forest.nodes() {for family in node.families() {
            if let Some(left) = family.left() {
                assert_eq!(forest.node(left).left_extent(), node.left_extent());
                assert_eq!(forest.node(left).right_extent(), family.pivot());
            }
            if let Some(right) = family.right() {
                assert_eq!(forest.node(right).left_extent(), family.pivot());
                assert_eq!(forest.node(right).right_extent(), node.right_extent());
            }
        }}
    }

    #[test]
    fn shares_a_long_juxtaposition() -> () {
        let mut tokens = Vec::new();
        for _ in 0..128 {tokens.push(Token::Identifier {name: "x"})}
        let forest = parse(&tokens);
        assert!(!forest.roots().is_empty());
        assert!(forest.node_count() < tokens.len() * tokens.len());
        for node in forest.nodes() {
            let mut families = Set::new();
            for family in node.families() {
                assert!(families.insert(*family));
            }
        }
    }
}
