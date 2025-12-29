//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Object,
    Group,
    crash,
    Code,
    Undefined,
    Whole,
    fmt,
    Value,
    Zero,
    Sign
};


//^
//^ INFINITE
//^

//> INFINITE -> STRUCT
#[derive(Clone)]
pub struct Infinite {
    pub sign: Sign
} impl Infinite {pub fn new(sign: impl Into<Sign>) -> Object {
    let sign0 = sign.into();
    return Object::Infinite(Infinite {
        sign: sign0
    });
}}

//> INFINITE -> CASTING
impl Infinite {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => self.into(),
    Group::Integer => crash(Code::FailedCast),
    Group::Natural => crash(Code::FailedCast),
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => crash(Code::FailedCast),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => Undefined::new(),
    Group::Variable => crash(Code::FailedCast),
    Group::Whole => crash(Code::FailedCast)
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
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
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
            self.sign == item.sign
        ),
        Object::Integer(item) => if item.value.is_zero() {Undefined::new()} else {Infinite::new(
            self.sign == item.sign
        )},
        Object::Natural(item) => self.into(),
        Object::Nexists(item) => self.into(),
        Object::Rational(item) => if item.numerator.is_zero() {Undefined::new()} else {Infinite::new(
            self.sign == item.sign
        )},
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => if item.value.is_zero() {Undefined::new()} else {self.into()}
    }}
}

//> INFINITE -> REPRESENTATION
impl fmt::Display for Infinite {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for Infinite {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> INFINITE -> COMMON
impl Value for Infinite {} impl Infinite {
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "Infinite")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "{} > sign = {}",
        self, self.sign    
    )}
}