//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::runtime::Value;
use crate::Display;
use crate::runtime::Object;
use crate::Debug;


//^
//^ INFINITE
//^

//> INFINITE -> STRUCT
#[derive(Clone)]
pub struct Infinite {
    pub negative: bool
}

//> INFINITE -> IMPLEMENTATION
impl Infinite {
    pub fn unequivalency(&self, to: &Object) -> bool {return match to {
        Object::Infinite(item) => return !self.equivalency(to),
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
    pub fn negate(&self) -> Object {return self.partial(Object::Infinite(crate::Infinite {
        negative: !self.negative
    }))}
    pub fn summation(&self, to: &Object) -> Object {return self.partial(match to {
        Object::Infinite(item) => {
            if self.negative != item.negative {Object::Undefined(crate::Undefined {})} else {self.to()}
        },
        Object::Nexists(item) => self.to(),
        Object::Number(item) => self.to(),
        Object::Tensor(item) => self.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
    pub fn invert(&self) -> Object {return self.partial(Object::Number(crate::Number {
        value: 0,
        shift: 0,
        negative: false
    }))}
    pub fn multiplication(&self, to: &Object) -> Object {return self.partial(match to {
        Object::Infinite(item) => Object::Infinite(crate::Infinite {
            negative: self.negative != item.negative
        }),
        Object::Nexists(item) => self.to(),
        Object::Number(item) => if item.value == 0 {Object::Undefined(crate::Undefined {})} else {Object::Infinite(crate::Infinite {
            negative: self.negative != item.negative
        })},
        Object::Tensor(item) => self.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crate::stdout::crash(crate::stdout::Code::UnexpectedValue)
    })}
}

//> INFINITE -> COMMON
impl Display for Infinite {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl Debug for Infinite {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 
impl Value for Infinite {} impl Infinite {
    pub fn to(&self) -> Object {return Object::Infinite(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Infinite")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "negative = {}",
        self.negative    
    )}
}