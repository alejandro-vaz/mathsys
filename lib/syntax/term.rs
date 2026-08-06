//^
//^ HEAD
//^

//> HEAD -> SUPER
use super::factor::Factor;


//^
//^ TERM
//^

//> TERM -> STRUCT
pub struct Term<'valid> {
    pub numerator: Vec<Factor<'valid>>,
    pub denominator: Vec<Factor<'valid>>
}