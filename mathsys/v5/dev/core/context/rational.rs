//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::value::Value;
use crate::object::Object;
use crate::group::Group;
use crate::stdout::{crash, Code};


//^
//^ RATIONAL
//^

//> RATIONAL -> STRUCT
#[derive(Clone)]
pub struct Rational {
    pub numerator: u32,
    pub denominator: u32,
    pub sign: bool
} impl Rational {pub fn new(numerator: u32, denominator: u32, sign: bool) -> Self {return Rational {
    numerator: numerator,
    denominator: if denominator != 0 {denominator} else {crash(Code::RationalDenominatorCannotBeZero)},
    sign: if numerator == 0 {true} else {sign}
}}}

//> RATIONAL -> CASTING
impl Rational {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => Object::Integer(crate::Integer::new(
        (self.numerator + self.denominator / 2) / self.denominator,
        self.sign
    )),
    Group::Natural => Object::Natural(crate::Natural::new(
        if self.sign {(self.numerator + self.denominator / 2) / self.denominator} else {crash(Code::FailedCast)}
    )),
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => self.to(),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => Object::Undefined(crate::Undefined::new()),
    Group::Variable => crash(Code::FailedCast),
    Group::Whole => Object::Whole(crate::Whole::new(
        if self.sign {(self.numerator + self.denominator / 2) / self.denominator} else {crash(Code::FailedCast)}
    ))
}}}

//> RATIONAL -> EQUIVALENCY
impl Rational {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.to()),
    Object::Integer(item) => item.equivalency(&self.to()),
    Object::Natural(item) => item.equivalency(&self.to()),
    Object::Nexists(item) => item.equivalency(&self.to()),
    Object::Rational(item) => self.sign == item.sign && self.numerator * item.denominator == item.numerator * self.denominator,
    Object::Tensor(item) => false,
    Object::Undefined(item) => false,
    Object::Variable(item) => false,
    Object::Whole(item) => self.numerator == self.denominator * item.value
}}}

//> RATIONAL -> SUMMATION
impl Rational {
    pub fn absolute(&self) -> Object {return Object::Rational(crate::Rational::new(
        self.numerator,
        self.denominator,
        true
    ))}
    pub fn negate(&self) -> Object {return Object::Rational(crate::Rational::new(
        self.numerator,
        self.denominator,
        !self.sign
    ))}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.to()),
        Object::Integer(item) => item.summation(&self.to()),
        Object::Natural(item) => item.summation(&self.to()),
        Object::Nexists(item) => item.summation(&self.to()),
        Object::Rational(item) => if self.sign == item.sign {Object::Rational(crate::Rational::new(
            self.numerator * item.denominator + self.denominator * item.numerator,
            self.denominator * item.denominator,
            self.sign
        ))} else {Object::Rational(crate::Rational::new(
            if self.numerator * item.denominator >= self.denominator * item.numerator {self.numerator * item.denominator - self.denominator * item.numerator} else {self.denominator * item.numerator - self.numerator * item.denominator},
            self.denominator * item.denominator,
            if self.numerator * item.denominator >= self.denominator * item.numerator {self.sign} else {item.sign}
        ))},
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => if self.sign {Object::Rational(crate::Rational::new(
            self.numerator + self.denominator * item.value,
            self.denominator,
            true
        ))} else {Object::Rational(crate::Rational::new(
            if self.numerator >= self.denominator * item.value {self.numerator - self.denominator * item.value} else {self.denominator * item.value - self.numerator},
            self.denominator,
            if self.numerator >= self.denominator * item.value {self.sign} else {!self.sign}
        ))}
    }}
}

//> RATIONAL -> MULTIPLICATION
impl Rational {
    pub fn invert(&self) -> Object {return Object::Rational(crate::Rational::new(
        self.denominator,
        self.numerator,
        self.sign
    ))}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.to()),
        Object::Integer(item) => item.multiplication(&self.to()),
        Object::Natural(item) => item.multiplication(&self.to()),
        Object::Nexists(item) => item.multiplication(&self.to()),
        Object::Rational(item) => Object::Rational(crate::Rational::new(
            self.numerator * item.numerator,
            self.denominator * item.denominator,
            self.sign == item.sign
        )),
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.to(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => Object::Rational(crate::Rational::new(
            self.numerator * item.value,
            self.denominator,
            self.sign
        ))
    }}
}



//> RATIONAL -> REPRESENTATION
impl crate::Display for Rational {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.display(formatter)}}
impl crate::Debug for Rational {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {self.debug(formatter)}} 

//> RATIONAL -> COMMON
impl Value for Rational {} impl Rational {
    pub fn to(&self) -> Object {return Object::Rational(self.clone())}
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter, "Rational")}
    pub fn debug(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {write!(formatter,
        "numerator = {}, denominator = {}, sign = {}",
        self.numerator, self.denominator, self.sign
    )}
}