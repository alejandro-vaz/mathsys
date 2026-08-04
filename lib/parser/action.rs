//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::production::Production;


//^
//^ ACTION
//^

//> ACTION -> ENUM
#[derive(PartialEq, Eq, Hash)]
pub enum Action {
    Shift {
        goto: usize
    },
    Reduce {
        production: Production
    }
}
