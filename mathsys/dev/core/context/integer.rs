//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::value::Value;
use crate::object::Object;
use crate::group::Group;
use crate::stdout::{crash, Code};


//^
//^ INTEGER
//^

//> INTEGER -> STRUCT
#[derive(Clone)]
pub struct Integer {
    pub value: u32,
    pub sign: bool
} impl Integer {pub fn new(value: u32, sign: bool) -> Self {return Integer {
    value: value,
    sign: if value == 0 {true} else {sign}
}}}

//> INTEGER -> CASTING
impl Integer {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => self.to(),
    Group::Natural => if self.sign {Object::Natural(crate::Natural::new(
        self.value
    ))} else {crash(Code::FailedCast)},
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => Object::Rational(crate::Rational::new(
        self.value,
        1,
        self.sign
    )),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => Object::Undefined(crate::Undefined::new()),
    Group::Variable => crash(Code::FailedCast),
    Group::Whole => if self.sign {Object::Whole(crate::Whole::new(
        self.value
    ))} else {crash(Code::FailedCast)}
}}}

//> INTEGER -> EQUIVALENCY
impl Integer {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.to()),
    Object::Integer(item) => self.value == item.value && self.sign == item.sign,
    Object::Natural(item) => self.sign && self.value == item.value,
    Object::Nexists(item) => false,
    Object::Rational(item) => self.sign == item.sign && self.value * item.denominator == item.numerator,
    Object::Tensor(item) => false,
    Object::Undefined(item) => false,
    Object::Variable(item) => false,
    Object::Whole(item) => self.sign && self.value == item.value
}}}

//> INTEGER -> SUMMATION
impl Integer {
    pub fn absolute(&self) -> Object {return Object::Integer(crate::Integer::new(
        self.value,
        true
    ))}
    pub fn negate(&self) -> Object {return Object::Integer(crate::Integer::new(
        self.value,
        !self.sign
    ))}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.to()),
        Object::Integer(item) => if self.sign == item.sign {Object::Integer(crate::Integer::new(
            self.value + item.value,
            self.sign
        ))} else {Object::Integer(crate::Integer::new(
            if self.value >= item.value {self.value - item.value} else {item.value - self.value},
            if self.value >= item.value {self.sign} else {item.sign}
        ))},
        Object::Natural(item) => Object::Integer(crate::Integer::new(
            if self.sign {self.value + item.value} else {if self.value >= item.value {self.value - item.value} else {item.value - self.value}},
            if self.sign {true} else {item.value >= self.value}
        )),
        Object::Nexists(item) => self.to(),
        Object::Rational(item) => if self.sign == item.sign {Object::Rational(crate::Rational::new(
            item.numerator + self.value * item.denominator,
            item.denominator,
            self.sign
        ))} else {Object::Rational(crate::Rational::new(
            if self.value * item.denominator >= item.numerator {self.value * item.denominator - item.numerator} else {item.numerator - self.value * item.denominator},
            item.denominator,
            if self.value * item.denominator >= item.numerator {self.sign} else {item.sign}
        ))}
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => Object::Integer(crate::Integer::new(
            if self.sign {self.value + item.value} else {if self.value >= item.value {self.value - item.value} else {item.value - self.value}},
            if self.sign {true} else {item.value >= self.value}
        ))
    }}
}

//> INTEGER -> MULTIPLICATION
impl Integer {
    pub fn invert(&self) -> Object {return Object::Rational(crate::Rational::new(
        1,
        self.value,
        self.sign
    ))}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.to()),
        Object::Integer(item) => Object::Integer(crate::Integer::new(
            self.value * item.value,
            self.sign == item.sign
        )),
        Object::Natural(item) => Object::Integer(crate::Integer::new(
            self.value * item.value,
            self.sign
        )),
        Object::Nexists(item) => self.to(),
        Object::Rational(item) => Object::Rational(crate::Rational::new(
            item.numerator * self.value,
            item.denominator,
            self.sign == item.sign
        )),
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => Object::Integer(crate::Integer::new(
            self.value * item.value,
            self.sign
        ))
    }}
}

//> INTEGER -> REPRESENTATION
impl crate::Display for Integer {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for Integer {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> INTEGER -> COMMON
impl Value for Integer {} impl Integer {
    pub fn to(&self) -> Object {return Object::Integer(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Integer")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "value = {}, sign = {}",
        self.value, self.sign
    )}
}