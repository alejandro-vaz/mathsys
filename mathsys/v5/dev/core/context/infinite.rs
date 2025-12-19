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
    pub sign: bool
} impl Infinite {pub fn new(sign: bool) -> Self {return Infinite {
    sign: sign
}}}

//> INFINITE -> CASTING
impl Infinite {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => self.to(),
    Group::Integer => crash(Code::FailedCast),
    Group::Natural => crash(Code::FailedCast),
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => crash(Code::FailedCast),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => Object::Undefined(crate::Undefined::new()),
    Group::Variable => crash(Code::FailedCast),
    Group::Whole => crash(Code::FailedCast)
}}}

//> INFINITE -> EQUIVALENCY
impl Infinite {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => self.sign == item.sign,
    Object::Integer(item) => false,
    Object::Natural(item) => false,
    Object::Nexists(item) => false,
    Object::Rational(item) => false,
    Object::Tensor(item) => false,
    Object::Undefined(item) => false,
    Object::Variable(item) => false,
    Object::Whole(item) => false
}}}

//> INFINITE -> SUMMATION
impl Infinite {
    pub fn absolute(&self) -> Object {return Object::Infinite(crate::Infinite::new(
        true
    ))}
    pub fn negate(&self) -> Object {return Object::Infinite(crate::Infinite::new(
        !self.sign
    ))}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => if self.sign != item.sign {Object::Undefined(crate::Undefined::new())} else {self.to()},
        Object::Integer(item) => self.to(),
        Object::Natural(item) => self.to(),
        Object::Nexists(item) => self.to(),
        Object::Rational(item) => self.to(),
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => self.to()
    }}
}

//> INFINITE -> MULTIPLICATION
impl Infinite {
    pub fn invert(&self) -> Object {return Object::Whole(crate::Whole::new(
        0
    ))}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => Object::Infinite(crate::Infinite::new(
            self.sign == item.sign
        )),
        Object::Integer(item) => if item.value == 0 {Object::Whole(crate::Whole::new(
            0
        ))} else {Object::Infinite(crate::Infinite::new(
            self.sign == item.sign
        ))},
        Object::Natural(item) => self.to(),
        Object::Nexists(item) => self.to(),
        Object::Rational(item) => if item.numerator == 0 {Object::Whole(crate::Whole::new(
            0
        ))} else {Object::Infinite(crate::Infinite::new(
            self.sign == item.sign
        ))},
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => if item.value == 0 {Object::Whole(crate::Whole::new(
            0
        ))} else {self.to()}
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
        "sign = {}",
        self.sign    
    )}
}