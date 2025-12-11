//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::Display;
use crate::runtime::Object;
use crate::Debug;


//^
//^ UNDEFINED
//^

//> UNDEFINED -> STRUCT
#[derive(Clone)]
pub struct Undefined {}

//> UNDEFINED -> IMPLEMENTATION
impl Undefined {
    pub fn unequivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => return item.unequivalency(&self.to()),
        Object::Nexists(item) => return item.unequivalency(&self.to()),
        Object::Number(item) => return item.unequivalency(&self.to()),
        Object::Tensor(item) => return item.unequivalency(&self.to()),
        Object::Undefined(item) => true,
        Object::Variable(item) => false
    }}
    pub fn equivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => return item.equivalency(&self.to()),
        Object::Nexists(item) => return item.equivalency(&self.to()),
        Object::Number(item) => return item.equivalency(&self.to()),
        Object::Tensor(item) => return item.equivalency(&self.to()),
        Object::Undefined(item) => false,
        Object::Variable(item) => false
    }}
    pub fn negate(&self) -> Object {return self.partial(Object::Undefined(crate::Undefined {}))}
    pub fn summation(&self, to: &Object) -> Object {return self.partial(match to {
        Object::Infinite(item) => return item.summation(&self.to()),
        Object::Nexists(item) => return item.summation(&self.to()),
        Object::Number(item) => return item.summation(&self.to()),
        Object::Tensor(item) => return item.summation(&self.to()),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
    pub fn invert(&self) -> Object {return self.partial(Object::Undefined(crate::Undefined {}))}
    pub fn multiplication(&self, to: &Object) -> Object {return self.partial(match to {
        Object::Infinite(item) => return item.multiplication(&self.to()),
        Object::Nexists(item) => return item.multiplication(&self.to()),
        Object::Number(item) => return item.multiplication(&self.to()),
        Object::Tensor(item) => return item.multiplication(&self.to()),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
}

//> UNDEFINED -> COMMON
impl Display for Undefined {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl Debug for Undefined {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 
impl Value for Undefined {} impl Undefined {
    pub fn to(&self) -> Object {return Object::Undefined(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Undefined")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, 
        ""
    )}
}