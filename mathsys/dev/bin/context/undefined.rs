//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::value::Value;
use crate::object::Object;


//^
//^ UNDEFINED
//^

//> UNDEFINED -> STRUCT
#[derive(Clone)]
pub struct Undefined {}

//> UNDEFINED -> EQUIVALENCY
impl Undefined {
    pub fn unequivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => item.unequivalency(&self.to()),
        Object::Nexists(item) => item.unequivalency(&self.to()),
        Object::Number(item) => item.unequivalency(&self.to()),
        Object::Tensor(item) => item.unequivalency(&self.to()),
        Object::Undefined(item) => true,
        Object::Variable(item) => false
    }}
    pub fn equivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => item.equivalency(&self.to()),
        Object::Nexists(item) => item.equivalency(&self.to()),
        Object::Number(item) => item.equivalency(&self.to()),
        Object::Tensor(item) => item.equivalency(&self.to()),
        Object::Undefined(item) => false,
        Object::Variable(item) => false
    }}
}

//> UNDEFINED -> SUMMATION
impl Undefined {
    pub fn negate(&self) -> Object {return Object::Undefined(crate::Undefined {})}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.to()),
        Object::Nexists(item) => item.summation(&self.to()),
        Object::Number(item) => item.summation(&self.to()),
        Object::Tensor(item) => item.summation(&self.to()),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
}

//> UNDEFINED -> MULTIPLICATION
impl Undefined {
    pub fn invert(&self) -> Object {return Object::Undefined(crate::Undefined {})}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.to()),
        Object::Nexists(item) => item.multiplication(&self.to()),
        Object::Number(item) => item.multiplication(&self.to()),
        Object::Tensor(item) => item.multiplication(&self.to()),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    }}
}

//> UNDEFINED -> REPRESENTATION
impl crate::Display for Undefined {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for Undefined {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> UNDEFINED -> COMMON
impl Value for Undefined {} impl Undefined {
    pub fn to(&self) -> Object {return Object::Undefined(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Undefined")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, 
        ""
    )}
}