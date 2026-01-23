//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    BigUint, Object, stdout, Code, Zero, Group, Integer, Rational, Undefined, Whole, Tensor, Sign
};


//^
//^ NATURAL
//^

//> NATURAL -> STRUCT
#[derive(Clone, Debug)]
pub struct Natural {
    pub value: BigUint
} impl Natural {pub fn new(value: impl Into<BigUint>) -> Object {
    let value0 = value.into();
    let value1 = if !value0.is_zero() {value0} else {stdout.crash(Code::NaturalCannotBeZero)};
    return Object::Natural(Natural {
        value: value1
    });
}}

//> NATURAL -> CASTING
impl Natural {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => stdout.crash(Code::FailedCast),
    Group::Integer => Integer::new(
        self.value.clone(),
        Sign::Positive
    ),
    Group::Natural => self.into(),
    Group::Nexists => stdout.crash(Code::FailedCast),
    Group::Rational => Rational::new(
        self.value.clone(),
        1u32,
        Sign::Positive
    ),
    Group::Tensor => stdout.crash(Code::FailedCast),
    Group::Undefined => Undefined::new(),
    Group::Variable => stdout.crash(Code::FailedCast),
    Group::Whole => Whole::new(
        self.value.clone()
    )
}}}

//> NATURAL -> EQUIVALENCY
impl Natural {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.into()),
    Object::Integer(item) => item.equivalency(&self.into()),
    Object::Natural(item) => self.value == item.value,
    Object::Nexists(item) => false,
    Object::Rational(item) => item.sign.into() && &self.value * &item.denominator == item.numerator,
    Object::Tensor(item) => false,
    Object::Undefined(item) => false,
    Object::Variable(item) => false,
    Object::Whole(item) => self.value == item.value
}}}

//> NATURAL -> SUMMATION
impl Natural {
    pub fn absolute(&self) -> Object {return self.into()}
    pub fn negate(&self) -> Object {return Integer::new(
        self.value.clone(),
        Sign::Negative
    )}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.into()),
        Object::Integer(item) => item.summation(&self.into()),
        Object::Natural(item) => Natural::new(
            &self.value + &item.value
        ),
        Object::Nexists(item) => self.into(),
        Object::Rational(item) => if item.sign.into() {Rational::new(
            &self.value * &item.denominator + &item.numerator,
            item.denominator.clone(),
            Sign::Positive
        )} else {Rational::new(
            if &self.value * &item.denominator >= item.numerator {&self.value * &item.denominator - &item.numerator} else {&item.numerator - &self.value * &item.denominator},
            item.denominator.clone(),
            (&self.value * &item.denominator >= item.numerator).into()
        )}
        Object::Tensor(item) => Undefined::new(),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => stdout.crash(Code::NoVariableOperation),
        Object::Whole(item) => Natural::new(
            &self.value + &item.value
        )
    }}
}

//> NATURAL -> MULTIPLICATION
impl Natural {
    pub fn invert(&self) -> Object {return Rational::new(
        1u32,
        self.value.clone(),
        Sign::Positive
    )}
    pub fn multiplication(&self, to: &Object) -> Object {return match &to.into() {
        Object::Infinite(item) => item.multiplication(&self.into()),
        Object::Integer(item) => item.multiplication(&self.into()),
        Object::Natural(item) => Natural::new(
            &self.value * &item.value
        ),
        Object::Nexists(item) => self.into(),
        Object::Rational(item) => Rational::new(
            &item.numerator * &self.value,
            item.denominator.clone(),
            item.sign
        ),
        Object::Tensor(item) => Tensor::new(
            item.values.iter().map(|value| self.multiplication(value)).collect()
        ),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => stdout.crash(Code::NoVariableOperation),
        Object::Whole(item) => Whole::new(
            &self.value * &item.value
        )
    }}
}