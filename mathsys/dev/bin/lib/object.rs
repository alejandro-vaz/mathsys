//^
//^ OBJECT
//^

//> OBJECT -> ENUM
#[derive(Clone)]
pub enum Object {
    Infinite(crate::Infinite),
    Nexists(crate::Nexists),
    Number(crate::Number),
    Tensor(crate::Tensor),
    Undefined(crate::Undefined),
    Variable(crate::Variable)
} 

//> OBJECT -> EQUIVALENCY
impl Object {
    pub fn unequivalency(&self, to: &Object) -> bool {self.genlocale0(to); return self.partial(match self {
        Object::Infinite(item) => item.unequivalency(to),
        Object::Nexists(item) => item.unequivalency(to),
        Object::Number(item) => item.unequivalency(to),
        Object::Tensor(item) => item.unequivalency(to),
        Object::Undefined(item) => item.unequivalency(to),
        Object::Variable(item) => item.unequivalency(to)
    })}
    pub fn equivalency(&self, to: &Object) -> bool {self.genlocale1(to); return self.partial(match self {
        Object::Infinite(item) => item.equivalency(to),
        Object::Nexists(item) => item.equivalency(to),
        Object::Number(item) => item.equivalency(to),
        Object::Tensor(item) => item.equivalency(to),
        Object::Undefined(item) => item.equivalency(to),
        Object::Variable(item) => item.equivalency(to)
    })}
}

//> OBJECT -> SUMMATION
impl Object {
    pub fn negate(&self) -> Object {self.genlocale2(); return self.partial(match self {
        Object::Infinite(item) => item.negate(),
        Object::Nexists(item) => item.negate(),
        Object::Number(item) => item.negate(),
        Object::Tensor(item) => item.negate(),
        Object::Undefined(item) => item.negate(),
        Object::Variable(item) => item.negate()
    })}
    pub fn summation(&self, to: &Object) -> Object {self.genlocale3(to); return self.partial(match self {
        Object::Infinite(item) => item.summation(to),
        Object::Nexists(item) => item.summation(to),
        Object::Number(item) => item.summation(to),
        Object::Tensor(item) => item.summation(to),
        Object::Undefined(item) => item.summation(to),
        Object::Variable(item) => item.summation(to)
    })}
}

//> OBJECT -> MULTIPLICATION
impl Object {
    pub fn invert(&self) -> Object {self.genlocale4(); return self.partial(match self {
        Object::Infinite(item) => item.invert(),
        Object::Nexists(item) => item.invert(),
        Object::Number(item) => item.invert(),
        Object::Tensor(item) => item.invert(),
        Object::Undefined(item) => item.invert(),
        Object::Variable(item) => item.invert()
    })}
    pub fn multiplication(&self, to: &Object) -> Object {self.genlocale5(to); return self.partial(match self {
        Object::Infinite(item) => item.multiplication(to),
        Object::Nexists(item) => item.multiplication(to),
        Object::Number(item) => item.multiplication(to),
        Object::Tensor(item) => item.multiplication(to),
        Object::Undefined(item) => item.multiplication(to),
        Object::Variable(item) => item.multiplication(to),
    })}
} 

//> OBJECT -> REPRESENTATION
impl crate::Display for Object {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {match self {
    Object::Infinite(item) => item.display(formatter),
    Object::Nexists(item) => item.display(formatter),
    Object::Number(item) => item.display(formatter),
    Object::Tensor(item) => item.display(formatter),
    Object::Undefined(item) => item.display(formatter),
    Object::Variable(item) => item.display(formatter)
}}} impl crate::Debug for Object {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {match self {
    Object::Infinite(item) => item.debug(formatter),
    Object::Nexists(item) => item.debug(formatter),
    Object::Number(item) => item.debug(formatter),
    Object::Tensor(item) => item.debug(formatter),
    Object::Undefined(item) => item.debug(formatter),
    Object::Variable(item) => item.debug(formatter)
}}}

//> OBJECT -> COMMON
impl Object {
    fn partial<Type: crate::Display + crate::Debug>(&self, value: Type) -> Type {crate::stdout::chore(format!(
        "{} > {:?}",
        value, value
    )); return value}
    fn genlocale0(&self, to: &Object) -> () {crate::stdout::trace(format!(
        "Checking unequivalency of {} and {}",
        self, to
    )); self.info(); to.info()}
    fn genlocale1(&self, to: &Object) -> () {crate::stdout::trace(format!(
        "Checking equivalency of {} and {}",
        self, to
    )); self.info(); to.info()}
    fn genlocale2(&self) -> () {crate::stdout::trace(format!(
        "Negating {}",
        self
    )); self.info()}
    fn genlocale3(&self, to: &Object) -> () {crate::stdout::trace(format!(
        "Adding {} and {}",
        self, to
    )); self.info(); to.info()}
    fn genlocale4(&self) -> () {crate::stdout::trace(format!(
        "Inverting {}",
        self
    )); self.info()}
    fn genlocale5(&self, to: &Object) -> () {crate::stdout::trace(format!(
        "Multiplying {} and {}",
        self, to
    )); self.info(); to.info()}
    pub fn info(&self) -> () {match self {
        Object::Infinite(item) => item.info(),
        Object::Nexists(item) => item.info(),
        Object::Number(item) => item.info(),
        Object::Tensor(item) => item.info(),
        Object::Undefined(item) => item.info(),
        Object::Variable(item) => item.info(),
    }}
}