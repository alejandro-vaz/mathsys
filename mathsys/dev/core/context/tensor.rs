//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::value::Value;
use crate::object::Object;
use crate::group::Group;
use crate::stdout::{crash, Code};


//^
//^ TENSOR
//^

//> TENSOR -> STRUCT
#[derive(Clone)]
pub struct Tensor {}

//> TENSOR -> CASTING
impl Tensor {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::UnexpectedValue),
    Group::Nexists => crash(Code::UnexpectedValue),
    Group::Number => crash(Code::UnexpectedValue),
    Group::Tensor => self.to(),
    Group::Undefined => Object::Undefined(crate::Undefined {}),
    Group::Variable => crash(Code::UnexpectedValue)
}}}

//> TENSOR -> EQUIVALENCY
impl Tensor {
    pub fn unequivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => item.unequivalency(&self.to()),
        Object::Nexists(item) => item.unequivalency(&self.to()),
        Object::Number(item) => item.unequivalency(&self.to()),
        Object::Tensor(item) => false,
        Object::Undefined(item) => false,
        Object::Variable(item) => true
    }}
    pub fn equivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => item.equivalency(&self.to()),
        Object::Nexists(item) => item.equivalency(&self.to()),
        Object::Number(item) => item.equivalency(&self.to()),
        Object::Tensor(item) => true,
        Object::Undefined(item) => false,
        Object::Variable(item) => false
    }}
}

//> TENSOR -> SUMMATION
impl Tensor {
    pub fn negate(&self) -> Object {return Object::Tensor(crate::Tensor {})}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.to()),
        Object::Nexists(item) => item.summation(&self.to()),
        Object::Number(item) => item.summation(&self.to()),
        Object::Tensor(item) => self.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::UnexpectedValue)
    }}
}

//> TENSOR -> MULTIPLICATION
impl Tensor {
    pub fn invert(&self) -> Object {return Object::Undefined(crate::Undefined {})}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.to()),
        Object::Nexists(item) => item.multiplication(&self.to()),
        Object::Number(item) => item.multiplication(&self.to()),
        Object::Tensor(item) => self.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::UnexpectedValue)
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