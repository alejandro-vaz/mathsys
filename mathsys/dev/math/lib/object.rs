//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::group::Group;
use crate::stdout::{chore, trace};


//^
//^ OBJECT
//^

//> OBJECT -> ENUM
#[derive(Clone)]
pub enum Object {
    Infinite(crate::Infinite),
    Integer(crate::Integer),
    Natural(crate::Natural),
    Nexists(crate::Nexists),
    Rational(crate::Rational),
    Tensor(crate::Tensor),
    Undefined(crate::Undefined),
    Variable(crate::Variable),
    Whole(crate::Whole)
} 

//> OBJECT -> CASTING
impl Object {pub fn cast(&self, group: Group) -> Object {self.castLocale(&group); return self.partial(match self {
    Object::Infinite(item) => item.cast(group),
    Object::Integer(item) => item.cast(group),
    Object::Natural(item) => item.cast(group),
    Object::Nexists(item) => item.cast(group),
    Object::Rational(item) => item.cast(group),
    Object::Tensor(item) => item.cast(group),
    Object::Undefined(item) => item.cast(group),
    Object::Variable(item) => item.cast(group),
    Object::Whole(item) => item.cast(group)
})}}

//> OBJECT -> EQUIVALENCY
impl Object {pub fn equivalency(&self, to: &Object) -> bool {self.equivalencyLocale(to); return self.partial(match self {
    Object::Infinite(item) => item.equivalency(to),
    Object::Integer(item) => item.equivalency(to),
    Object::Natural(item) => item.equivalency(to),
    Object::Nexists(item) => item.equivalency(to),
    Object::Rational(item) => item.equivalency(to),
    Object::Tensor(item) => item.equivalency(to),
    Object::Undefined(item) => item.equivalency(to),
    Object::Variable(item) => item.equivalency(to),
    Object::Whole(item) => item.equivalency(to)
})}}

//> OBJECT -> SUMMATION
impl Object {
    pub fn absolute(&self) -> Object {self.absoluteLocale(); return self.partial(match self {
        Object::Infinite(item) => item.absolute(),
        Object::Integer(item) => item.absolute(),
        Object::Natural(item) => item.absolute(),
        Object::Nexists(item) => item.absolute(),
        Object::Rational(item) => item.absolute(),
        Object::Tensor(item) => item.absolute(),
        Object::Undefined(item) => item.absolute(),
        Object::Variable(item) => item.absolute(),
        Object::Whole(item) => item.absolute()
    })}
    pub fn negate(&self) -> Object {self.negateLocale(); return self.partial(match self {
        Object::Infinite(item) => item.negate(),
        Object::Integer(item) => item.negate(),
        Object::Natural(item) => item.negate(),
        Object::Nexists(item) => item.negate(),
        Object::Rational(item) => item.negate(),
        Object::Tensor(item) => item.negate(),
        Object::Undefined(item) => item.negate(),
        Object::Variable(item) => item.negate(),
        Object::Whole(item) => item.negate()
    })}
    pub fn summation(&self, to: &Object) -> Object {self.summationLocale(to); return self.partial(match self {
        Object::Infinite(item) => item.summation(to),
        Object::Integer(item) => item.summation(to),
        Object::Natural(item) => item.summation(to),
        Object::Nexists(item) => item.summation(to),
        Object::Rational(item) => item.summation(to),
        Object::Tensor(item) => item.summation(to),
        Object::Undefined(item) => item.summation(to),
        Object::Variable(item) => item.summation(to),
        Object::Whole(item) => item.summation(to)
    })}
}

//> OBJECT -> MULTIPLICATION
impl Object {
    pub fn invert(&self) -> Object {self.invertLocale(); return self.partial(match self {
        Object::Infinite(item) => item.invert(),
        Object::Integer(item) => item.invert(),
        Object::Natural(item) => item.invert(),
        Object::Nexists(item) => item.invert(),
        Object::Rational(item) => item.invert(),
        Object::Tensor(item) => item.invert(),
        Object::Undefined(item) => item.invert(),
        Object::Variable(item) => item.invert(),
        Object::Whole(item) => item.invert()
    })}
    pub fn multiplication(&self, to: &Object) -> Object {self.multiplicationLocale(to); return self.partial(match self {
        Object::Infinite(item) => item.multiplication(to),
        Object::Integer(item) => item.multiplication(to),
        Object::Natural(item) => item.multiplication(to),
        Object::Nexists(item) => item.multiplication(to),
        Object::Rational(item) => item.multiplication(to),
        Object::Tensor(item) => item.multiplication(to),
        Object::Undefined(item) => item.multiplication(to),
        Object::Variable(item) => item.multiplication(to),
        Object::Whole(item) => item.multiplication(to)
    })}
} 

//> OBJECT -> REPRESENTATION
impl crate::Display for Object {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {match self {
    Object::Infinite(item) => item.display(formatter),
    Object::Integer(item) => item.display(formatter),
    Object::Natural(item) => item.display(formatter),
    Object::Nexists(item) => item.display(formatter),
    Object::Rational(item) => item.display(formatter),
    Object::Tensor(item) => item.display(formatter),
    Object::Undefined(item) => item.display(formatter),
    Object::Variable(item) => item.display(formatter),
    Object::Whole(item) => item.display(formatter)
}}} impl crate::Debug for Object {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {match self {
    Object::Infinite(item) => item.debug(formatter),
    Object::Integer(item) => item.debug(formatter),
    Object::Natural(item) => item.debug(formatter),
    Object::Nexists(item) => item.debug(formatter),
    Object::Rational(item) => item.debug(formatter),
    Object::Tensor(item) => item.debug(formatter),
    Object::Undefined(item) => item.debug(formatter),
    Object::Variable(item) => item.debug(formatter),
    Object::Whole(item) => item.debug(formatter)
}}}

//> OBJECT -> COMMON
impl Object {
    fn partial<Type: crate::Display + crate::Debug>(&self, value: Type) -> Type {chore(format!(
        "{} > {:?}",
        value, value
    )); return value}
    fn castLocale(&self, group: &Group) -> () {trace(format!(
        "Casting {} to {}",
        self, group
    ))}
    fn equivalencyLocale(&self, to: &Object) -> () {trace(format!(
        "Checking equivalency of {} and {}",
        self, to
    )); self.info(); to.info()}
    fn absoluteLocale(&self) -> () {trace(format!(
        "Taking absolute value of {}",
        self
    )); self.info()}
    fn negateLocale(&self) -> () {trace(format!(
        "Negating {}",
        self
    )); self.info()}
    fn summationLocale(&self, to: &Object) -> () {trace(format!(
        "Adding {} and {}",
        self, to
    )); self.info(); to.info()}
    fn invertLocale(&self) -> () {trace(format!(
        "Inverting {}",
        self
    )); self.info()}
    fn multiplicationLocale(&self, to: &Object) -> () {trace(format!(
        "Multiplying {} and {}",
        self, to
    )); self.info(); to.info()}
    pub fn info(&self) -> () {match self {
        Object::Infinite(item) => item.info(),
        Object::Integer(item) => item.info(),
        Object::Natural(item) => item.info(),
        Object::Nexists(item) => item.info(),
        Object::Rational(item) => item.info(),
        Object::Tensor(item) => item.info(),
        Object::Undefined(item) => item.info(),
        Object::Variable(item) => item.info(),
        Object::Whole(item) => item.info()
    }}
    pub fn is(&self, group: Group) -> bool {return match self {
        Object::Infinite(item) => group == Group::Undefined || group == Group::Infinite,
        Object::Integer(item) => group == Group::Undefined || group == Group::Integer,
        Object::Natural(item) => group == Group::Undefined || group == Group::Natural,
        Object::Nexists(item) => group == Group::Undefined || group == Group::Nexists,
        Object::Rational(item) => group == Group::Undefined || group == Group::Rational,
        Object::Tensor(item) => group == Group::Undefined || group == Group::Tensor,
        Object::Undefined(item) => true,
        Object::Variable(item) => group == Group::Undefined || group == Group::Variable,
        Object::Whole(item) => group == Group::Undefined || group == Group::Whole
    }}
}