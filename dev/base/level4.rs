//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::level2::Level2;


//^
//^ 4ºLEVEL
//^

//> 4ºLEVEL -> NAMESPACE
pub enum Level4 {
    Factor(Factor),
    Limit(Limit)
}

//> 4ºLEVEL -> FACTOR
pub struct Factor {
    exponent: Option<Level2>
} impl Factor {
    pub fn summon() -> Self {return Factor {
        exponent: None
    }}
}

//> 4ºLEVEL -> LIMIT
pub struct Limit {
    exponent: Option<Level2>
} impl Limit {
    pub fn summon() -> Self {return Limit {
        exponent: None
    }}
}