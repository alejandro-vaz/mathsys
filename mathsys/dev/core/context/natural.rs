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
    Group::Rational => Object::Rational(crate::Rational::new(
        self.value,
        1,
        true
    )),
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
    Object::Rational(item) => item.sign && self.value * item.denominator == item.numerator,
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
        Object::Rational(item) => if item.sign {Object::Rational(crate::Rational::new(
            self.value * item.denominator + item.numerator,
            item.denominator,
            true
        ))} else {Object::Rational(crate::Rational::new(
            if self.value * item.denominator >= item.numerator {self.value * item.denominator - item.numerator} else {item.numerator - self.value * item.denominator},
            item.denominator,
            self.value * item.denominator >= item.numerator
        ))}
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => Object::Natural(crate::Natural::new(
            self.value + item.value
        ))
    }}
}

//> NATURAL -> MULTIPLICATION
impl Natural {
    pub fn invert(&self) -> Object {return Object::Rational(crate::Rational::new(
        1,
        self.value,
        true
    ))}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.to()),
        Object::Integer(item) => item.multiplication(&self.to()),
        Object::Natural(item) => Object::Natural(crate::Natural::new(
            self.value * item.value
        )),
        Object::Nexists(item) => self.to(),
        Object::Rational(item) => Object::Rational(crate::Rational::new(
            item.numerator * self.value,
            item.denominator,
            item.sign
        )),
        Object::Tensor(item) => crash(Code::Todo),
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