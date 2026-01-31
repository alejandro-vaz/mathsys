//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::level2::Level2;


//^
//^ 5ºLEVEL
//^

//> 5ºLEVEL -> NAMESPACE
pub enum Level5 {
    Infinite(Infinite),
    Variable(Variable),
    Nest(Nest),
    Tensor(Tensor),
    Whole(Whole),
    Absolute(Absolute),
    Undefined(Undefined),
    Rational(Rational),
    Casts(Casts)
}

//> 5ºLEVEL -> INFINITE
pub struct Infinite {

} impl Infinite {
    pub fn summon() -> Self {return Infinite {}}
}

//> 5ºLEVEL -> VARIABLE
pub struct Variable {

} impl Variable {
    pub fn summon() -> Self {return Variable {}}
}

//> 5ºLEVEL -> NEST
pub struct Nest {
    value: Option<Level2>
} impl Nest {
    pub fn summon() -> Self {return Nest {
        value: None
    }}
}

//> 5ºLEVEL -> TENSOR
pub struct Tensor {
    values: Vec<Level2>
} impl Tensor {
    pub fn summon() -> Self {return Tensor {
        values: Vec::new()
    }}
}

//> 5ºLEVEL -> WHOLE
pub struct Whole {

} impl Whole {
    pub fn summon() -> Self {return Whole {}}
}

//> 5ºLEVEL -> ABSOLUTE
pub struct Absolute {
} impl Absolute {
    pub fn summon() -> Self {return  Absolute {}}
}

//> 5ºLEVEL -> UNDEFINED
pub struct Undefined {

} impl Undefined {
    pub fn summon() -> Self {return Undefined {}}
}

//> 5ºLEVEL -> RATIONAL
pub struct Rational {

} impl Rational {
    pub fn summon() -> Self {return Rational {}}
}

//> 5ºLEVEL -> CASTS
pub struct Casts {

} impl Casts {
    pub fn summon() -> Self {return Casts {}}
}