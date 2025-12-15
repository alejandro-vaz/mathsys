//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::value::Value;
use crate::object::Object;
use crate::group::Group;
use crate::stdout::{crash, Code};


//^
//^ INFINITE
//^

//> INFINITE -> STRUCT
#[derive(Clone)]
pub struct Infinite {
    pub negative: bool
}

//> INFINITE -> CASTING
impl Infinite {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => self.to(),
    Group::Nexists => crash(Code::UnexpectedValue),
    Group::Number => crash(Code::UnexpectedValue),
    Group::Tensor => crash(Code::UnexpectedValue),
    Group::Undefined => Object::Undefined(crate::Undefined {}),
    Group::Variable => crash(Code::UnexpectedValue)
}}}

//> INFINITE -> EQUIVALENCY
impl Infinite {
    pub fn unequivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => !self.equivalency(to),
        Object::Nexists(item) => true,
        Object::Number(item) => true,
        Object::Tensor(item) => true,
        Object::Undefined(item) => false,
        Object::Variable(item) => true
    }}
    pub fn equivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => self.negative == item.negative,
        Object::Nexists(item) => false,
        Object::Number(item) => false,
        Object::Tensor(item) => false,
        Object::Undefined(item) => false,
        Object::Variable(item) => false
    }}
}

//> INFINITE -> SUMMATION
impl Infinite {
    pub fn negate(&self) -> Object {return Object::Infinite(crate::Infinite {
        negative: !self.negative
    })}
    pub fn summation(&self, to: &Object) -> Object {match to {
        Object::Infinite(item) => if self.negative != item.negative {Object::Undefined(crate::Undefined {})} else {self.to()},
        Object::Nexists(item) => self.to(),
        Object::Number(item) => self.to(),
        Object::Tensor(item) => self.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::UnexpectedValue)
    }}
}

//> INFINITE -> MULTIPLICATION
impl Infinite {
    pub fn invert(&self) -> Object {return Object::Number(crate::Number {
        value: 0,
        shift: 0,
        negative: false
    })}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => Object::Infinite(crate::Infinite {
            negative: self.negative != item.negative
        }),
        Object::Nexists(item) => self.to(),
        Object::Number(item) => if item.value == 0 {Object::Number(crate::Number {
            value: 0, 
            shift: 0, 
            negative: false
        })} else {Object::Infinite(crate::Infinite {
            negative: self.negative != item.negative
        })},
        Object::Tensor(item) => self.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::UnexpectedValue)
    }}
}

//> INFINITE -> REPRESENTATION
impl crate::Display for Infinite {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for Infinite {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> INFINITE -> COMMON
impl Value for Infinite {} impl Infinite {
    pub fn to(&self) -> Object {return Object::Infinite(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Infinite")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "negative = {}",
        self.negative    
    )}
}