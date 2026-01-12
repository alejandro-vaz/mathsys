//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Pointer, Runtime, Class, Object, BigUint, Rational, Sign
};


//^
//^ RATIONAL
//^

//> RATIONAL -> STRUCT
#[derive(Clone, Debug)]
pub struct _Rational {
    pub whole: BigUint,
    pub decimal: BigUint
}

//> RATIONAL -> EVALUATE
impl _Rational {pub fn evaluate(&self, runtime: &mut Runtime, id: Pointer, memory: &Vec<Class>) -> Object {
    //~ EVALUATE -> OPERATIONS
    return Rational::new(
        &self.whole * &BigUint::from(10u32).pow(self.decimal.to_string().chars().rev().collect::<String>().parse().unwrap()) + &self.decimal.to_string().chars().rev().collect::<String>().parse::<BigUint>().unwrap(),
        BigUint::from(10u32).pow(self.decimal.to_string().chars().rev().collect::<String>().parse().unwrap()),
        Sign::Positive
    )
}}