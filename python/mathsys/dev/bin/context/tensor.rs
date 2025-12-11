//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::Display;
use crate::runtime::Object;
use crate::Debug;


//^
//^ TENSOR
//^

//> TENSOR -> CONSTRUCT
#[derive(Clone)]
pub struct Tensor {}

//> TENSOR -> IMPLEMENTATION
impl Tensor {
    pub fn unequivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => return item.unequivalency(&self.to()),
        Object::Nexists(item) => return item.unequivalency(&self.to()),
        Object::Number(item) => return item.unequivalency(&self.to()),
        Object::Tensor(item) => false,
        Object::Undefined(item) => false,
        Object::Variable(item) => true
    }}
    pub fn equivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => return item.equivalency(&self.to()),
        Object::Nexists(item) => return item.equivalency(&self.to()),
        Object::Number(item) => return item.equivalency(&self.to()),
        Object::Tensor(item) => true,
        Object::Undefined(item) => false,
        Object::Variable(item) => false
    }}
    pub fn negate(&self) -> Object {return self.partial(Object::Tensor(crate::Tensor {}))}
    pub fn summation(&self, to: &Object) -> Object {return self.partial(match to {
        Object::Infinite(item) => return item.summation(&self.to()),
        Object::Nexists(item) => return item.summation(&self.to()),
        Object::Number(item) => return item.summation(&self.to()),
        Object::Tensor(item) => self.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
    pub fn invert(&self) -> Object {return self.partial(Object::Undefined(crate::Undefined {}))}
    pub fn multiplication(&self, to: &Object) -> Object {return self.partial(match to {
        Object::Infinite(item) => return item.multiplication(&self.to()),
        Object::Nexists(item) => return item.multiplication(&self.to()),
        Object::Number(item) => return item.multiplication(&self.to()),
        Object::Tensor(item) => self.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
}

//> NUMBER -> COMMON
impl Display for Tensor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl Debug for Tensor {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 
impl Value for Tensor {} impl Tensor {
    pub fn to(&self) -> Object {return Object::Tensor(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Tensor")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, 
        ""
    )}
}