//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::{Infinite, Integer, Natural, Nexists, Rational, Undefined, Variable, Whole};
use crate::runtime::Runtime;
use crate::value::Value;
use crate::object::Object;
use crate::group::Group;
use crate::stdout::{crash, Code};


//^
//^ TENSOR
//^

//> TENSOR -> STRUCT
#[derive(Clone)]
pub struct Tensor {} impl Tensor {pub fn new() -> Object {return Object::Tensor(Tensor {})}}

//> TENSOR -> CASTING
impl Tensor {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => crash(Code::FailedCast),
    Group::Natural => crash(Code::FailedCast),
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => crash(Code::FailedCast),
    Group::Tensor => self.to(),
    Group::Undefined => Undefined::new(),
    Group::Variable => crash(Code::FailedCast),
    Group::Whole => crash(Code::FailedCast)
}}}

//> TENSOR -> EQUIVALENCY
impl Tensor {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.to()),
    Object::Integer(item) => item.equivalency(&self.to()),
    Object::Natural(item) => item.equivalency(&self.to()),
    Object::Nexists(item) => item.equivalency(&self.to()),
    Object::Rational(item) => item.equivalency(&self.to()),
    Object::Tensor(item) => true,
    Object::Undefined(item) => false,
    Object::Variable(item) => false,
    Object::Whole(item) => false
}}}

//> TENSOR -> SUMMATION
impl Tensor {
    pub fn absolute(&self) -> Object {crash(Code::Todo)}
    pub fn negate(&self) -> Object {crash(Code::Todo)}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.to()),
        Object::Integer(item) => item.summation(&self.to()),
        Object::Natural(item) => item.summation(&self.to()),
        Object::Nexists(item) => item.summation(&self.to()),
        Object::Rational(item) => item.summation(&self.to()),
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => crash(Code::Todo)
    }}
}

//> TENSOR -> MULTIPLICATION
impl Tensor {
    pub fn invert(&self) -> Object {return Undefined::new()}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.to()),
        Object::Integer(item) => item.multiplication(&self.to()),
        Object::Natural(item) => item.multiplication(&self.to()),
        Object::Nexists(item) => item.multiplication(&self.to()),
        Object::Rational(item) => item.multiplication(&self.to()),
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => crash(Code::Todo)
    }}
}

//> TENSOR -> REPRESENTATION
impl crate::Display for Tensor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for Tensor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> TENSOR -> COMMON
impl Value for Tensor {} impl Tensor {
    pub fn to(&self) -> Object {return Object::Tensor(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Tensor")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, 
        ""
    )}
}