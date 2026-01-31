//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::level4::Level4;


//^
//^ 3ºLEVEL
//^

//> 3ºLEVEL -> NAMESPACE
pub enum Level3 {
    Term(Term)
}

//> 3ºLEVEL -> TERM
pub struct Term {
    numerator: Vec<Level4>,
    denominator: Vec<Level4>
} impl Term {
    pub fn summon() -> Self {return Term {
        numerator: Vec::new(),
        denominator: Vec::new()
    }}
}

