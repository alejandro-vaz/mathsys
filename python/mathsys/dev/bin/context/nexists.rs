//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::Display;
use crate::runtime::Object;
use crate::Debug;


//^
//^ NEXISTS
//^

//> NEXISTS -> CONSTRUCT
#[derive(Clone)]
pub struct Nexists {}

//> NEXISTS -> IMPLEMENTATION
impl Nexists {
    pub fn unequivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => return item.unequivalency(&self.to()),
        Object::Nexists(item) => true,
        Object::Number(item) => true,
        Object::Tensor(item) => true,
        Object::Undefined(item) => false,
        Object::Variable(item) => true
    }}
    pub fn equivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => return item.equivalency(&self.to()),
        Object::Nexists(item) => false,
        Object::Number(item) => false,
        Object::Tensor(item) => false,
        Object::Undefined(item) => false,
        Object::Variable(item) => false
    }}
    pub fn negate(&self) -> Object {return self.partial(Object::Nexists(crate::Nexists {}))}
    pub fn summation(&self, to: &Object) -> Object {return self.partial(match to {
        Object::Infinite(item) => return item.summation(&self.to()),
        Object::Nexists(item) => item.to(),
        Object::Number(item) => item.to(),
        Object::Tensor(item) => item.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
    pub fn invert(&self) -> Object {return self.partial(Object::Nexists(crate::Nexists {}))}
    pub fn multiplication(&self, to: &Object) -> Object {return self.partial(match to {
        Object::Infinite(item) => return item.multiplication(&self.to()),
        Object::Nexists(item) => item.to(),
        Object::Number(item) => item.to(),
        Object::Tensor(item) => item.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
}

//> NEXISTS -> COMMON
impl Display for Nexists {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl Debug for Nexists {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 
impl Value for Nexists {} impl Nexists {
    pub fn to(&self) -> Object {return Object::Nexists(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Nexists")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, 
        ""
    )}
}