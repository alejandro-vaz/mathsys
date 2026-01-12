//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    BigUint, Code, Group, Natural, Object, Rational, Sign, Tensor, Undefined, Whole, Zero, crash
};


//^
//^ INTEGER
//^

//> INTEGER -> STRUCT
#[derive(Clone, Debug)]
pub struct Integer {
    pub value: BigUint,
    pub sign: Sign
} impl Integer {pub fn new(value: impl Into<BigUint>, sign: Sign) -> Object {
    let value0 = value.into();
    let sign0 = if value0.is_zero() {Sign::Positive} else {sign};
    return Object::Integer(Integer {
        value: value0,
        sign: sign0
    });
}}

//> INTEGER -> CASTING
impl Integer {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => self.into(),
    Group::Natural => if self.sign.into() {Natural::new(
        self.value.clone()
    )} else {crash(Code::FailedCast)},
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => Rational::new(
        self.value.clone(),
        1u32,
        self.sign
    ),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => Undefined::new(),
    Group::Variable => crash(Code::FailedCast),
    Group::Whole => if self.sign.into() {Whole::new(
        self.value.clone()
    )} else {crash(Code::FailedCast)}
}}}

//> INTEGER -> EQUIVALENCY
impl Integer {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.into()),
    Object::Integer(item) => self.value == item.value && self.sign == item.sign,
    Object::Natural(item) => self.sign.into() && self.value == item.value,
    Object::Nexists(item) => false,
    Object::Rational(item) => self.sign == item.sign && &self.value * &item.denominator == item.numerator,
    Object::Tensor(item) => false,
    Object::Undefined(item) => false,
    Object::Variable(item) => false,
    Object::Whole(item) => self.sign.into() && self.value == item.value
}}}

//> INTEGER -> SUMMATION
impl Integer {
    pub fn absolute(&self) -> Object {return Integer::new(
        self.value.clone(),
        Sign::Positive
    )}
    pub fn negate(&self) -> Object {return Integer::new(
        self.value.clone(),
        !self.sign
    )}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.into()),
        Object::Integer(item) => if self.sign == item.sign {Integer::new(
            &self.value + &item.value,
            self.sign
        )} else {Integer::new(
            if self.value >= item.value {&self.value - &item.value} else {&item.value - &self.value},
            if self.value >= item.value {self.sign} else {item.sign}
        )},
        Object::Natural(item) => Integer::new(
            if self.sign.into() {&self.value + &item.value} else {if self.value >= item.value {&self.value - &item.value} else {&item.value - &self.value}},
            (if self.sign.into() {true} else {item.value >= self.value}).into()
        ),
        Object::Nexists(item) => self.into(),
        Object::Rational(item) => if self.sign == item.sign {Rational::new(
            &item.numerator + &self.value * &item.denominator,
            item.denominator.clone(),
            self.sign
        )} else {Rational::new(
            if &self.value * &item.denominator >= item.numerator {&self.value * &item.denominator - &item.numerator} else {&item.numerator - &self.value * &item.denominator},
            item.denominator.clone(),
            if &self.value * &item.denominator >= item.numerator {self.sign} else {item.sign}
        )}
        Object::Tensor(item) => Undefined::new(),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => Integer::new(
            if self.sign.into() {&self.value + &item.value} else {if self.value >= item.value {&self.value - &item.value} else {&item.value - &self.value}},
            (if self.sign.into() {true} else {item.value >= self.value}).into()
        )
    }}
}

//> INTEGER -> MULTIPLICATION
impl Integer {
    pub fn invert(&self) -> Object {return if !self.value.is_zero() {Rational::new(
        1u32,
        self.value.clone(),
        self.sign
    )} else {Undefined::new()}}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.into()),
        Object::Integer(item) => Integer::new(
            &self.value * &item.value,
            (self.sign == item.sign).into()
        ),
        Object::Natural(item) => Integer::new(
            &self.value * &item.value,
            self.sign
        ),
        Object::Nexists(item) => self.into(),
        Object::Rational(item) => Rational::new(
            &item.numerator * &self.value,
            item.denominator.clone(),
            (self.sign == item.sign).into()
        ),
        Object::Tensor(item) => Tensor::new(
            item.values.iter().map(|value| self.multiplication(value)).collect()
        ),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => Integer::new(
            &self.value * &item.value,
            self.sign
        )
    }}
}