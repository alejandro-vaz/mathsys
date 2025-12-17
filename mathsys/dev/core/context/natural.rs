//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::value::Value;
use crate::object::Object;
use crate::group::Group;
use crate::stdout::{crash, Code};


//^
//^ NATURAL
//^

//> NATURAL -> STRUCT
#[derive(Clone)]
pub struct Natural {
    pub value: u32
}

//> NATURAL -> CASTING
impl Natural {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::UnexpectedValue),
    Group::Natural => self.to(),
    Group::Nexists => crash(Code::UnexpectedValue),
    Group::Tensor => crash(Code::UnexpectedValue),
    Group::Undefined => Object::Undefined(crate::Undefined {}),
    Group::Variable => crash(Code::UnexpectedValue)
}}}

//> NATURAL -> EQUIVALENCY
impl Natural {
    pub fn unequivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => item.unequivalency(&self.to()),
        Object::Natural(item) => !self.equivalency(to),
        Object::Nexists(item) => true,
        Object::Tensor(item) => true,
        Object::Undefined(item) => false,
        Object::Variable(item) => true
    }}
    pub fn equivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => item.equivalency(&self.to()),
        Object::Natural(item) => self.value == item.value,
        Object::Nexists(item) => false,
        Object::Tensor(item) => false,
        Object::Undefined(item) => false,
        Object::Variable(item) => false
    }}
}

//> NATURAL -> SUMMATION
impl Natural {
    pub fn negate(&self) -> Object {crash(Code::UnexpectedValue)}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.to()),
        Object::Natural(item) => {Object::Natural(crate::Natural {
            value: self.value + item.value
        })},
        Object::Nexists(item) => self.to(),
        Object::Tensor(item) => self.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::UnexpectedValue)
    }}
}

//> NATURAL -> MULTIPLICATION
impl Natural {
    pub fn invert(&self) -> Object {crash(Code::UnexpectedValue)}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.to()),
        Object::Natural(item) => Object::Natural(crate::Natural {
            value: self.value * item.value
        }),
        Object::Nexists(item) => self.to(),
        Object::Tensor(item) => item.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::UnexpectedValue)
    }}
}

//> NATURAL -> REPRESENTATION
impl crate::Display for Natural {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for Natural {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> NATURAL -> COMMON
impl Value for Natural {} impl Natural {
    pub fn to(&self) -> Object {return Object::Natural(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Natural")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "value = {}",
        self.value
    )}
}