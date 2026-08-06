//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::term::Term;


//^
//^ EXPRESSION
//^

//> EXPRESSION -> STRUCT
pub struct Expression<'valid> {
    pub terms: Vec<(Vec<bool>, Term<'valid>)>
}