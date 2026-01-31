//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::level3::Level3;


//^
//^ 2ºLEVEL
//^

//> 2ºLEVEL -> NAMESPACE
pub enum Level2 {
    Expression(Expression)
}

//> 2ºLEVEL -> EXPRESSION
pub struct Expression {
    terms: Vec<(Vec<bool>, Level3)>
} impl Expression {
    pub fn summon() -> Self {return Expression {
        terms: Vec::new()
    }}
}