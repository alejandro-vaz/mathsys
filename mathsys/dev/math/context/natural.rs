//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    BigUint,
    Object,
    crash,
    Code,
    Zero,
    Group,
    Integer,
    Rational,
    fmt,
    Undefined,
    Whole,
    Value,
    Tensor
};


//^
//^ NATURAL
//^

//> NATURAL -> STRUCT
#[derive(Clone)]
pub struct Natural {
    pub value: BigUint
} impl Natural {pub fn new(value: impl Into<BigUint>) -> Object {
    let value0 = value.into();
    let value1 = if !value0.is_zero() {value0} else {crash(Code::NaturalCannotBeZero)};
    return Object::Natural(Natural {
        value: value1
    });
}}

//> NATURAL -> CASTING
impl Natural {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => Integer::new(
        self.value.clone(),
        true
    ),
    Group::Natural => self.into(),
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => Rational::new(
        self.value.clone(),
        1u32,
        true
    ),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => Undefined::new(),
    Group::Variable => crash(Code::FailedCast),
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
        false
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
            true
        )} else {Rational::new(
            if &self.value * &item.denominator >= item.numerator {&self.value * &item.denominator - &item.numerator} else {&item.numerator - &self.value * &item.denominator},
            item.denominator.clone(),
            &self.value * &item.denominator >= item.numerator
        )}
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
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
        true
    )}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
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
            item.values.iter().map(|value| value.multiplication(&self.into())).collect()
        ),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => Whole::new(
            &self.value * &item.value
        )
    }}
}

//> NATURAL -> REPRESENTATION
impl fmt::Display for Natural {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for Natural {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> NATURAL -> COMMON
impl Value for Natural {} impl Natural {
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "Natural")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "{} > value = {}",
        self, self.value
    )}
}