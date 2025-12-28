//^
//^ HEAD
//^

use num_bigint::BigUint;

//> HEAD -> CROSS-SCOPE TRAIT
use crate::{Infinite, Natural, Nexists, Rational, Tensor, Undefined, Variable, Whole};
use crate::runtime::Runtime;
use crate::value::Value;
use crate::object::Object;
use crate::group::Group;
use crate::stdout::{crash, Code};


//^
//^ INTEGER
//^

//> INTEGER -> STRUCT
#[derive(Clone)]
pub struct Integer {
    pub value: BigUint,
    pub sign: bool
} impl Integer {pub fn new(value: BigUint, sign: bool) -> Object {return Object::Integer(Integer {
    value: value.clone(),
    sign: if value == BigUint::ZERO {true} else {sign}
})}}

//> INTEGER -> CASTING
impl Integer {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => self.to(),
    Group::Natural => if self.sign {Natural::new(
        self.value.clone()
    )} else {crash(Code::FailedCast)},
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => Rational::new(
        self.value.clone(),
        1u32.into(),
        self.sign
    ),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => Undefined::new(),
    Group::Variable => crash(Code::FailedCast),
    Group::Whole => if self.sign {Whole::new(
        self.value.clone()
    )} else {crash(Code::FailedCast)}
}}}

//> INTEGER -> EQUIVALENCY
impl Integer {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.to()),
    Object::Integer(item) => self.value == item.value && self.sign == item.sign,
    Object::Natural(item) => self.sign && self.value == item.value,
    Object::Nexists(item) => false,
    Object::Rational(item) => self.sign == item.sign && &self.value * &item.denominator == item.numerator,
    Object::Tensor(item) => false,
    Object::Undefined(item) => false,
    Object::Variable(item) => false,
    Object::Whole(item) => self.sign && self.value == item.value
}}}

//> INTEGER -> SUMMATION
impl Integer {
    pub fn absolute(&self) -> Object {return Integer::new(
        self.value.clone(),
        true
    )}
    pub fn negate(&self) -> Object {return Integer::new(
        self.value.clone(),
        !self.sign
    )}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.to()),
        Object::Integer(item) => if self.sign == item.sign {Integer::new(
            &self.value + &item.value,
            self.sign
        )} else {Integer::new(
            if self.value >= item.value {&self.value - &item.value} else {&item.value - &self.value},
            if self.value >= item.value {self.sign} else {item.sign}
        )},
        Object::Natural(item) => Integer::new(
            if self.sign {&self.value + &item.value} else {if self.value >= item.value {&self.value - &item.value} else {&item.value - &self.value}},
            if self.sign {true} else {item.value >= self.value}
        ),
        Object::Nexists(item) => self.to(),
        Object::Rational(item) => if self.sign == item.sign {Rational::new(
            &item.numerator + &self.value * &item.denominator,
            item.denominator.clone(),
            self.sign
        )} else {Rational::new(
            if &self.value * &item.denominator >= item.numerator {&self.value * &item.denominator - &item.numerator} else {&item.numerator - &self.value * &item.denominator},
            item.denominator.clone(),
            if &self.value * &item.denominator >= item.numerator {self.sign} else {item.sign}
        )}
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => Integer::new(
            if self.sign {&self.value + &item.value} else {if self.value >= item.value {&self.value - &item.value} else {&item.value - &self.value}},
            if self.sign {true} else {item.value >= self.value}
        )
    }}
}

//> INTEGER -> MULTIPLICATION
impl Integer {
    pub fn invert(&self) -> Object {return if self.value != BigUint::ZERO {Rational::new(
        1u32.into(),
        self.value.clone(),
        self.sign
    )} else {Undefined::new()}}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.to()),
        Object::Integer(item) => Integer::new(
            &self.value * &item.value,
            self.sign == item.sign
        ),
        Object::Natural(item) => Integer::new(
            &self.value * &item.value,
            self.sign
        ),
        Object::Nexists(item) => self.to(),
        Object::Rational(item) => Rational::new(
            &item.numerator * &self.value,
            item.denominator.clone(),
            self.sign == item.sign
        ),
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => Integer::new(
            &self.value * &item.value,
            self.sign
        )
    }}
}

//> INTEGER -> REPRESENTATION
impl crate::Display for Integer {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for Integer {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> INTEGER -> COMMON
impl Value for Integer {} impl Integer {
    pub fn to(&self) -> Object {return Object::Integer(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Integer")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "value = {}, sign = {}",
        self.value, self.sign
    )}
}