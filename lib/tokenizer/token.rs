//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::kind::Kind;


//^
//^ TOKEN
//^

//> TOKEN -> STRUCT
#[derive(PartialEq, Eq, Hash, Clone, Debug)]
pub struct Token<'input> {
    pub value: &'input str,
    pub kind: Kind
}