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
    Natural(crate::Natural),
    Nexists(crate::Nexists),
    Tensor(crate::Tensor),
    Undefined(crate::Undefined),
    Variable(crate::Variable)
} 

//> OBJECT -> CASTING
impl Object {pub fn cast(&self, group: Group) -> Object {return match self {
    Object::Infinite(item) => item.cast(group),
    Object::Natural(item) => item.cast(group),
    Object::Nexists(item) => item.cast(group),
    Object::Tensor(item) => item.cast(group),
    Object::Undefined(item) => item.cast(group),
    Object::Variable(item) => item.cast(group)
}}}

//> OBJECT -> EQUIVALENCY
impl Object {
    pub fn unequivalency(&self, to: &Object) -> bool {self.genlocale0(to); return self.partial(match self {
        Object::Infinite(item) => item.unequivalency(to),
        Object::Natural(item) => item.unequivalency(to),
        Object::Nexists(item) => item.unequivalency(to),
        Object::Tensor(item) => item.unequivalency(to),
        Object::Undefined(item) => item.unequivalency(to),
        Object::Variable(item) => item.unequivalency(to)
    })}
    pub fn equivalency(&self, to: &Object) -> bool {self.genlocale1(to); return self.partial(match self {
        Object::Infinite(item) => item.equivalency(to),
        Object::Natural(item) => item.equivalency(to),
        Object::Nexists(item) => item.equivalency(to),
        Object::Tensor(item) => item.equivalency(to),
        Object::Undefined(item) => item.equivalency(to),
        Object::Variable(item) => item.equivalency(to)
    })}
}

//> OBJECT -> SUMMATION
impl Object {
    pub fn negate(&self) -> Object {self.genlocale2(); return self.partial(match self {
        Object::Infinite(item) => item.negate(),
        Object::Natural(item) => item.negate(),
        Object::Nexists(item) => item.negate(),
        Object::Tensor(item) => item.negate(),
        Object::Undefined(item) => item.negate(),
        Object::Variable(item) => item.negate()
    })}
    pub fn summation(&self, to: &Object) -> Object {self.genlocale3(to); return self.partial(match self {
        Object::Infinite(item) => item.summation(to),
        Object::Natural(item) => item.summation(to),
        Object::Nexists(item) => item.summation(to),
        Object::Tensor(item) => item.summation(to),
        Object::Undefined(item) => item.summation(to),
        Object::Variable(item) => item.summation(to)
    })}
}

//> OBJECT -> MULTIPLICATION
impl Object {
    pub fn invert(&self) -> Object {self.genlocale4(); return self.partial(match self {
        Object::Infinite(item) => item.invert(),
        Object::Natural(item) => item.invert(),
        Object::Nexists(item) => item.invert(),
        Object::Tensor(item) => item.invert(),
        Object::Undefined(item) => item.invert(),
        Object::Variable(item) => item.invert()
    })}
    pub fn multiplication(&self, to: &Object) -> Object {self.genlocale5(to); return self.partial(match self {
        Object::Infinite(item) => item.multiplication(to),
        Object::Natural(item) => item.multiplication(to),
        Object::Nexists(item) => item.multiplication(to),
        Object::Tensor(item) => item.multiplication(to),
        Object::Undefined(item) => item.multiplication(to),
        Object::Variable(item) => item.multiplication(to),
    })}
} 

//> OBJECT -> REPRESENTATION
impl crate::Display for Object {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {match self {
    Object::Infinite(item) => item.display(formatter),
    Object::Natural(item) => item.display(formatter),
    Object::Nexists(item) => item.display(formatter),
    Object::Tensor(item) => item.display(formatter),
    Object::Undefined(item) => item.display(formatter),
    Object::Variable(item) => item.display(formatter)
}}} impl crate::Debug for Object {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {match self {
    Object::Infinite(item) => item.debug(formatter),
    Object::Natural(item) => item.debug(formatter),
    Object::Nexists(item) => item.debug(formatter),
    Object::Tensor(item) => item.debug(formatter),
    Object::Undefined(item) => item.debug(formatter),
    Object::Variable(item) => item.debug(formatter)
}}}

//> OBJECT -> COMMON
impl Object {
    fn partial<Type: crate::Display + crate::Debug>(&self, value: Type) -> Type {chore(format!(
        "{} > {:?}",
        value, value
    )); return value}
    fn genlocale0(&self, to: &Object) -> () {trace(format!(
        "Checking unequivalency of {} and {}",
        self, to
    )); self.info(); to.info()}
    fn genlocale1(&self, to: &Object) -> () {trace(format!(
        "Checking equivalency of {} and {}",
        self, to
    )); self.info(); to.info()}
    fn genlocale2(&self) -> () {trace(format!(
        "Negating {}",
        self
    )); self.info()}
    fn genlocale3(&self, to: &Object) -> () {trace(format!(
        "Adding {} and {}",
        self, to
    )); self.info(); to.info()}
    fn genlocale4(&self) -> () {trace(format!(
        "Inverting {}",
        self
    )); self.info()}
    fn genlocale5(&self, to: &Object) -> () {trace(format!(
        "Multiplying {} and {}",
        self, to
    )); self.info(); to.info()}
    pub fn info(&self) -> () {match self {
        Object::Infinite(item) => item.info(),
        Object::Natural(item) => item.info(),
        Object::Nexists(item) => item.info(),
        Object::Tensor(item) => item.info(),
        Object::Undefined(item) => item.info(),
        Object::Variable(item) => item.info(),
    }}
    pub fn is(&self, group: Group) -> bool {return match self {
        Object::Infinite(item) => group == Group::Undefined || group == Group::Infinite,
        Object::Natural(item) => group == Group::Undefined || group == Group::Natural,
        Object::Nexists(item) => group == Group::Undefined || group == Group::Nexists,
        Object::Tensor(item) => group == Group::Undefined || group == Group::Tensor,
        Object::Undefined(item) => true,
        Object::Variable(item) => group == Group::Undefined || group == Group::Variable
    }}
}