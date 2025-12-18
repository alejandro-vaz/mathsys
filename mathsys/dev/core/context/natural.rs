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
} impl Natural {pub fn new(value: u32) -> Self {return Natural {
    value: if value != 0 {value} else {crash(Code::NaturalCannotBeZero)}
}}}

//> NATURAL -> CASTING
impl Natural {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => Object::Integer(crate::Integer::new(
        self.value,
        true
    )),
    Group::Natural => self.to(),
    Group::Nexists => crash(Code::FailedCast),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => Object::Undefined(crate::Undefined::new()),
    Group::Variable => crash(Code::FailedCast),
    Group::Whole => Object::Whole(crate::Whole::new(
        self.value
    ))
}}}

//> NATURAL -> EQUIVALENCY
impl Natural {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.to()),
    Object::Integer(item) => item.equivalency(&self.to()),
    Object::Natural(item) => self.value == item.value,
    Object::Nexists(item) => false,
    Object::Tensor(item) => false,
    Object::Undefined(item) => false,
    Object::Variable(item) => false,
    Object::Whole(item) => self.value == item.value
}}}

//> NATURAL -> SUMMATION
impl Natural {
    pub fn absolute(&self) -> Object {return self.to()}
    pub fn negate(&self) -> Object {return Object::Integer(crate::Integer::new(
        self.value,
        false
    ))}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.to()),
        Object::Integer(item) => item.summation(&self.to()),
        Object::Natural(item) => Object::Natural(crate::Natural::new(
            self.value + item.value
        )),
        Object::Nexists(item) => self.to(),
        Object::Tensor(item) => self.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => Object::Natural(crate::Natural::new(
            self.value + item.value
        ))
    }}
}

//> NATURAL -> MULTIPLICATION
impl Natural {
    pub fn invert(&self) -> Object {crash(Code::Todo)}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.to()),
        Object::Integer(item) => item.multiplication(&self.to()),
        Object::Natural(item) => Object::Natural(crate::Natural::new(
            self.value * item.value
        )),
        Object::Nexists(item) => self.to(),
        Object::Tensor(item) => item.to(),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => Object::Whole(crate::Whole::new(
            self.value * item.value
        ))
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