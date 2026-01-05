//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Object, Group, crash, Code, Undefined
};


//^
//^ NEXISTS
//^

//> NEXISTS -> STRUCT
#[derive(Clone, Debug)]
pub struct Nexists {} impl Nexists {pub fn new() -> Object {
    return Object::Nexists(Nexists {})
}}

//> NEXISTS -> CASTING
impl Nexists {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => crash(Code::FailedCast),
    Group::Natural => crash(Code::FailedCast),
    Group::Nexists => self.into(),
    Group::Rational => crash(Code::FailedCast),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => Undefined::new(),
    Group::Variable => crash(Code::FailedCast),
    Group::Whole => crash(Code::FailedCast)
}}}

//> NEXISTS -> EQUIVALENCY
impl Nexists {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.into()),
    Object::Integer(item) => item.equivalency(&self.into()),
    Object::Natural(item) => item.equivalency(&self.into()),
    Object::Nexists(item) => false,
    Object::Rational(item) => false,
    Object::Tensor(item) => false,
    Object::Undefined(item) => false,
    Object::Variable(item) => false,
    Object::Whole(item) => false
}}}

//> NEXISTS -> SUMMATION
impl Nexists {
    pub fn absolute(&self) -> Object {return self.into()}
    pub fn negate(&self) -> Object {return Nexists::new()}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.into()),
        Object::Integer(item) => item.summation(&self.into()),
        Object::Natural(item) => item.summation(&self.into()),
        Object::Nexists(item) => item.into(),
        Object::Rational(item) => item.into(),
        Object::Tensor(item) => item.into(),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => item.into()
    }}
}

//> NEXISTS -> MULTIPLICATION
impl Nexists {
    pub fn invert(&self) -> Object {return Nexists::new()}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.into()),
        Object::Integer(item) => item.multiplication(&self.into()),
        Object::Natural(item) => item.multiplication(&self.into()),
        Object::Nexists(item) => item.into(),
        Object::Rational(item) => item.into(),
        Object::Tensor(item) => item.into(),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => item.into()
    }}
}