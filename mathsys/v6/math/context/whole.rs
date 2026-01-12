//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    BigUint, Object, Group, crash, Integer, Natural, Rational, Undefined, Code, Zero, Sign
};


//^
//^ WHOLE
//^

//> WHOLE -> STRUCT
#[derive(Clone, Debug)]
pub struct Whole {
    pub value: BigUint
} impl Whole {pub fn new(value: impl Into<BigUint>) -> Object {
    let value0 = value.into();
    return Object::Whole(Whole {
        value: value0
    });
}}

//> WHOLE -> CASTING
impl Whole {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => Integer::new(
        self.value.clone(),
        Sign::Positive
    ),
    Group::Natural => Natural::new(
        self.value.clone()
    ),
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => Rational::new(
        self.value.clone(),
        1u32,
        Sign::Positive
    ),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => Undefined::new(),
    Group::Variable => crash(Code::FailedCast),
    Group::Whole => self.into()
}}}

//> WHOLE -> EQUIVALENCY
impl Whole {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.into()),
    Object::Integer(item) => item.equivalency(&self.into()),
    Object::Natural(item) => item.equivalency(&self.into()),
    Object::Nexists(item) => item.equivalency(&self.into()),
    Object::Rational(item) => item.equivalency(&self.into()),
    Object::Tensor(item) => item.equivalency(&self.into()),
    Object::Undefined(item) => item.equivalency(&self.into()),
    Object::Variable(item) => item.equivalency(&self.into()),
    Object::Whole(item) => &item.value == &self.value
}}}

//> WHOLE -> SUMMATION
impl Whole {
    pub fn absolute(&self) -> Object {return self.into()}
    pub fn negate(&self) -> Object {return Integer::new(
        self.value.clone(),
        Sign::Negative
    )}
    pub fn summation(&self, to: &Object) -> Object {return match &to.into() {
        Object::Infinite(item) => item.summation(&self.into()),
        Object::Integer(item) => item.summation(&self.into()),
        Object::Natural(item) => item.summation(&self.into()),
        Object::Nexists(item) => item.summation(&self.into()),
        Object::Rational(item) => item.summation(&self.into()),
        Object::Tensor(item) => item.summation(&self.into()),
        Object::Undefined(item) => item.summation(&self.into()),
        Object::Variable(item) => item.summation(&self.into()),
        Object::Whole(item) => Whole::new(
            &self.value + &item.value
        )
    }}
}

//> WHOLE -> MULTIPLICATION
impl Whole {
    pub fn invert(&self) -> Object {return if !self.value.is_zero() {Rational::new(
        1u32,
        self.value.clone(),
        Sign::Positive
    )} else {Undefined::new()}}
    pub fn multiplication(&self, to: &Object) -> Object {return match to.into() {
        Object::Infinite(item) => item.multiplication(&self.into()),
        Object::Integer(item) => item.multiplication(&self.into()),
        Object::Natural(item) => item.multiplication(&self.into()),
        Object::Nexists(item) => item.multiplication(&self.into()),
        Object::Rational(item) => item.multiplication(&self.into()),
        Object::Tensor(item) => item.multiplication(&self.into()),
        Object::Undefined(item) => item.multiplication(&self.into()),
        Object::Variable(item) => item.multiplication(&self.into()),
        Object::Whole(item) => Whole::new(
            &self.value * &item.value
        )
    }}
}