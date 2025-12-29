//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Object,
    Group,
    crash,
    Code,
    Value,
    fmt
};


//^
//^ UNDEFINED
//^

//> UNDEFINED -> STRUCT
#[derive(Clone)]
pub struct Undefined {} impl Undefined {pub fn new() -> Object {
    return Object::Undefined(Undefined {})
}}

//> UNDEFINED -> CASTING
impl Undefined {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => crash(Code::FailedCast),
    Group::Natural => crash(Code::FailedCast),
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => crash(Code::FailedCast),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => self.into(),
    Group::Variable => crash(Code::FailedCast),
    Group::Whole => crash(Code::FailedCast)
}}}

//> UNDEFINED -> EQUIVALENCY
impl Undefined {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.into()),
    Object::Integer(item) => item.equivalency(&self.into()),
    Object::Natural(item) => item.equivalency(&self.into()),
    Object::Nexists(item) => item.equivalency(&self.into()),
    Object::Rational(item) => item.equivalency(&self.into()),
    Object::Tensor(item) => item.equivalency(&self.into()),
    Object::Undefined(item) => false,
    Object::Variable(item) => false,
    Object::Whole(item) => false
}}}

//> UNDEFINED -> SUMMATION
impl Undefined {
    pub fn absolute(&self) -> Object {return self.into()}
    pub fn negate(&self) -> Object {return Undefined::new()}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.into()),
        Object::Integer(item) => item.summation(&self.into()),
        Object::Natural(item) => item.summation(&self.into()),
        Object::Nexists(item) => item.summation(&self.into()),
        Object::Rational(item) => item.summation(&self.into()),
        Object::Tensor(item) => item.summation(&self.into()),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => self.into()
    }}
}

//> UNDEFINED -> MULTIPLICATION
impl Undefined {
    pub fn invert(&self) -> Object {return Undefined::new()}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.into()),
        Object::Integer(item) => item.multiplication(&self.into()),
        Object::Natural(item) => item.multiplication(&self.into()),
        Object::Nexists(item) => item.multiplication(&self.into()),
        Object::Rational(item) => item.multiplication(&self.into()),
        Object::Tensor(item) => item.multiplication(&self.into()),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => self.into()
    }}
}

//> UNDEFINED -> REPRESENTATION
impl fmt::Display for Undefined {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for Undefined {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> UNDEFINED -> COMMON
impl Value for Undefined {} impl Undefined {
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "Undefined")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, 
        "{}",
        self
    )}
}