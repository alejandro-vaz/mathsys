//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    BigUint, Code, Group, Integer, Natural, Number, Object, One, Sign, Tensor, Undefined, Value, Whole, Zero, crash, fmt
};


//^
//^ RATIONAL
//^

//> RATIONAL -> STRUCT
#[derive(Clone)]
pub struct Rational {
    pub numerator: BigUint,
    pub denominator: BigUint,
    pub sign: Sign
} impl Rational {pub fn new(numerator: impl Into<BigUint>, denominator: impl Into<BigUint>, sign: impl Into<Sign>) -> Object {
    let numerator0 = numerator.into();
    let denominator0 = denominator.into();
    let sign0 = if numerator0.is_zero() {Sign::Positive} else {sign.into()};
    let denominator1 = if !denominator0.is_zero() {denominator0} else {crash(Code::RationalDenominatorCannotBeZero)};
    let gcd = numerator0.gcd(&denominator1);
    let numerator1 = &numerator0 / &gcd;
    let denominator2 = &denominator1 / &gcd;
    return Object::Rational(Rational {
        numerator: numerator1,
        denominator: denominator2,
        sign: sign0
    });
}}

//> RATIONAL -> CASTING
impl Rational {pub fn cast(&self, group: Group) -> Object {return match group {
    Group::Infinite => crash(Code::FailedCast),
    Group::Integer => Integer::new(
        if self.denominator.is_one() {self.numerator.clone()} else {crash(Code::FailedCast)},
        self.sign
    ),
    Group::Natural => Natural::new(
        if self.denominator.is_one() && self.sign.into() && !self.numerator.is_zero() {self.numerator.clone()} else {crash(Code::FailedCast)}
    ),
    Group::Nexists => crash(Code::FailedCast),
    Group::Rational => self.into(),
    Group::Tensor => crash(Code::FailedCast),
    Group::Undefined => Undefined::new(),
    Group::Variable => crash(Code::FailedCast),
    Group::Whole => Whole::new(
        if self.denominator.is_one() && self.sign.into() {self.numerator.clone()} else {crash(Code::FailedCast)}
    )
}}}

//> RATIONAL -> EQUIVALENCY
impl Rational {pub fn equivalency(&self, to: &Object) -> bool {return match to {
    Object::Infinite(item) => item.equivalency(&self.into()),
    Object::Integer(item) => item.equivalency(&self.into()),
    Object::Natural(item) => item.equivalency(&self.into()),
    Object::Nexists(item) => item.equivalency(&self.into()),
    Object::Rational(item) => self.sign == item.sign && &self.numerator * &item.denominator == &item.numerator * &self.denominator,
    Object::Tensor(item) => false,
    Object::Undefined(item) => false,
    Object::Variable(item) => false,
    Object::Whole(item) => self.sign.into() && self.numerator == &self.denominator * &item.value
}}}

//> RATIONAL -> SUMMATION
impl Rational {
    pub fn absolute(&self) -> Object {return Rational::new(
        self.numerator.clone(),
        self.denominator.clone(),
        true
    )}
    pub fn negate(&self) -> Object {return Rational::new(
        self.numerator.clone(),
        self.denominator.clone(),
        !self.sign
    )}
    pub fn summation(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.summation(&self.into()),
        Object::Integer(item) => item.summation(&self.into()),
        Object::Natural(item) => item.summation(&self.into()),
        Object::Nexists(item) => item.summation(&self.into()),
        Object::Rational(item) => if self.sign == item.sign {Rational::new(
            &self.numerator * &item.denominator + &self.denominator * &item.numerator,
            &self.denominator * &item.denominator,
            self.sign
        )} else {Rational::new(
            if &self.numerator * &item.denominator >= &self.denominator * &item.numerator {&self.numerator * &item.denominator - &self.denominator * &item.numerator} else {&self.denominator * &item.numerator - &self.numerator * &item.denominator},
            &self.denominator * &item.denominator,
            if &self.numerator * &item.denominator >= &self.denominator * &item.numerator {self.sign} else {item.sign}
        )},
        Object::Tensor(item) => crash(Code::Todo),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => if self.sign.into() {Rational::new(
            &self.numerator + &self.denominator * &item.value,
            self.denominator.clone(),
            true
        )} else {Rational::new(
            if self.numerator >= &self.denominator * &item.value {&self.numerator - &self.denominator * &item.value} else {&self.denominator * &item.value - &self.numerator},
            self.denominator.clone(),
            if self.numerator >= &self.denominator * &item.value {self.sign} else {!self.sign}
        )}
    }}
}

//> RATIONAL -> MULTIPLICATION
impl Rational {
    pub fn invert(&self) -> Object {return if !self.numerator.is_zero() {Rational::new(
        self.denominator.clone(),
        self.numerator.clone(),
        self.sign
    )} else {Undefined::new()}}
    pub fn multiplication(&self, to: &Object) -> Object {return match to {
        Object::Infinite(item) => item.multiplication(&self.into()),
        Object::Integer(item) => item.multiplication(&self.into()),
        Object::Natural(item) => item.multiplication(&self.into()),
        Object::Nexists(item) => item.multiplication(&self.into()),
        Object::Rational(item) => Rational::new(
            &self.numerator * &item.numerator,
            &self.denominator * &item.denominator,
            self.sign == item.sign
        ),
        Object::Tensor(item) => Tensor::new(
            item.values.iter().map(|value| self.multiplication(value)).collect()
        ),
        Object::Undefined(item) => item.into(),
        Object::Variable(item) => crash(Code::NoVariableOperation),
        Object::Whole(item) => Rational::new(
            &self.numerator * &item.value,
            self.denominator.clone(),
            self.sign
        )
    }}
}



//> RATIONAL -> REPRESENTATION
impl fmt::Display for Rational {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.display(formatter)}}
impl fmt::Debug for Rational {fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {self.debug(formatter)}} 

//> RATIONAL -> COMMON
impl Value for Rational {} impl Rational {
    pub fn info(&self) -> () {self.data()}
    pub fn display(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter, "Rational")}
    pub fn debug(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {write!(formatter,
        "{} > numerator = {} ~ denominator = {} ~ sign = {}",
        self, self.numerator, self.denominator, self.sign
    )}
}