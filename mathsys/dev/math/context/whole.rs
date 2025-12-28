//^
//^ HEAD
//^

//> HEAD -> CRATES
use num_bigint::BigUint;

//> HEAD -> CROSS-SCOPE TRAIT
use crate::{Infinite, Integer, Natural, Nexists, Rational, Tensor, Undefined, Variable};
use crate::runtime::Runtime;
use crate::value::Value;
use crate::object::Object;
use crate::group::Group;
use crate::stdout::{crash, Code};


//^
//^ WHOLE
//^

//> WHOLE -> STRUCT
#[derive(Clone)]
pub struct Whole {
    pub value: BigUint
} impl Whole {pub fn new(value: BigUint) -> Object {return Object::Whole(Whole {
    value: value
})}}

//> WHOLE -> CASTING
impl Whole {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => Integer::new(
        self.value.clone(),
        true
    ),
    Group::Natural => Natural::new(
        self.value.clone()
    ),
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => Rational::new(
        self.value.clone(),
        1u32.into(),
        true
    ),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => Undefined::new(),
    Group::Variable => crash(Code::FailedCast),
    Group::Whole => self.to()
}}}

//> WHOLE -> EQUIVALENCY
impl Whole {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.to()),
    Object::Integer(item) => item.equivalency(&self.to()),
    Object::Natural(item) => item.equivalency(&self.to()),
    Object::Nexists(item) => item.equivalency(&self.to()),
    Object::Rational(item) => item.equivalency(&self.to()),
    Object::Tensor(item) => item.equivalency(&self.to()),
    Object::Undefined(item) => item.equivalency(&self.to()),
    Object::Variable(item) => item.equivalency(&self.to()),
    Object::Whole(item) => &item.value == &self.value
}}}

//> WHOLE -> SUMMATION
impl Whole {
    pub fn absolute(&self) -> Object {return self.to()}
    pub fn negate(&self) -> Object {return Integer::new(
        self.value.clone(),
        false
    )}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.to()),
        Object::Integer(item) => item.summation(&self.to()),
        Object::Natural(item) => item.summation(&self.to()),
        Object::Nexists(item) => item.summation(&self.to()),
        Object::Rational(item) => item.summation(&self.to()),
        Object::Tensor(item) => item.summation(&self.to()),
        Object::Undefined(item) => item.summation(&self.to()),
        Object::Variable(item) => item.summation(&self.to()),
        Object::Whole(item) => Whole::new(
            &self.value + &item.value
        )
    }}
}

//> WHOLE -> MULTIPLICATION
impl Whole {
    pub fn invert(&self) -> Object {return if self.value != BigUint::ZERO {Rational::new(
        1u32.into(),
        self.value.clone(),
        true
    )} else {Undefined::new()}}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.to()),
        Object::Integer(item) => item.multiplication(&self.to()),
        Object::Natural(item) => item.multiplication(&self.to()),
        Object::Nexists(item) => item.multiplication(&self.to()),
        Object::Rational(item) => item.multiplication(&self.to()),
        Object::Tensor(item) => item.multiplication(&self.to()),
        Object::Undefined(item) => item.multiplication(&self.to()),
        Object::Variable(item) => item.multiplication(&self.to()),
        Object::Whole(item) => Whole::new(
            &self.value * &item.value
        )
    }}
}

//> WHOLE -> REPRESENTATION
impl crate::Display for Whole {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for Whole {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> WHOLE -> COMMON
impl Value for Whole {} impl Whole {
    pub fn to(&self) -> Object {return Object::Whole(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Whole")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "value = {}",
        self.value
    )}
}