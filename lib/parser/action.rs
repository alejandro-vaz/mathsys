//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::rule::Rule;


//^
//^ ACTION
//^

//> ACTION -> ENUM
#[derive(PartialEq, Eq, Hash, Debug)] // rm debug
pub enum Action {
    Shift {
        goto: usize
    },
    Reduce {
        rule: &'static Rule,
        length: usize
    },
    Accept
}