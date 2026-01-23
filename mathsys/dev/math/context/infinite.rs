//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Object, Group, stdout, Code, Undefined, Whole, Zero, Sign, Tensor
};


//^
//^ INFINITE
//^

//> INFINITE -> STRUCT
#[derive(Clone, Debug)]
pub struct Infinite {
    pub sign: Sign
} impl Infinite {pub fn new(sign: Sign) -> Object {
    let sign0 = sign;
    return Object::Infinite(Infinite {
        sign: sign0
    });
}}

//> INFINITE -> CASTING
impl Infinite {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => self.into(),
    Group::Integer => stdout.crash(Code::FailedCast),
    Group::Natural => stdout.crash(Code::FailedCast),
    Group::Nexists => stdout.crash(Code::FailedCast),
    Group::Rational => stdout.crash(Code::FailedCast),
    Group::Tensor => stdout.crash(Code::FailedCast),
    Group::Undefined => Undefined::new(),
    Group::Variable => stdout.crash(Code::FailedCast),
    Group::Whole => stdout.crash(Code::FailedCast)
}}}

//> INFINITE -> EQUIVALENCY
impl Infinite {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => self.sign == item.sign,
    Object::Integer(item) => false,
    Object::Natural(item) => false,
    Object::Nexists(item) => false,
    Object::Rational(item) => false,
    Object::Tensor(item) => false,
    Object::Undefined(item) => false,
    Object::Variable(item) => false,
    Object::Whole(item) => false
}}}

//> INFINITE -> SUMMATION
impl Infinite {
    pub fn absolute(&self) -> Object {return Infinite::new(
        Sign::Positive
    )}
    pub fn negate(&self) -> Object {return Infinite::new(
        !self.sign
    )}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => if self.sign != item.sign {Undefined::new()} else {self.into()},
        Object::Integer(item) => self.into(),
        Object::Natural(item) => self.into(),
        Object::Nexists(item) => self.into(),
        Object::Rational(item) => self.into(),
        Object::Tensor(item) => Undefined::new(),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => stdout.crash(Code::NoVariableOperation),
        Object::Whole(item) => self.into()
    }}
}

//> INFINITE -> MULTIPLICATION
impl Infinite {
    pub fn invert(&self) -> Object {return Whole::new(
        0u32
    )}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => Infinite::new(
            (self.sign == item.sign).into()
        ),
        Object::Integer(item) => if item.value.is_zero() {Undefined::new()} else {Infinite::new(
            (self.sign == item.sign).into()
        )},
        Object::Natural(item) => self.into(),
        Object::Nexists(item) => self.into(),
        Object::Rational(item) => if item.numerator.is_zero() {Undefined::new()} else {Infinite::new(
            (self.sign == item.sign).into()
        )},
        Object::Tensor(item) => Tensor::new(
            item.values.iter().map(|value| self.multiplication(value)).collect()
        ),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => stdout.crash(Code::NoVariableOperation),
        Object::Whole(item) => if item.value.is_zero() {Undefined::new()} else {self.into()}
    }}
}