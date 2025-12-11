//^
//^ CONTEXT
//^

//> CONTEXT -> VALUE
pub trait Value: crate::Display + crate::Debug {
    fn data(&self) -> () {crate::stdout::debug(format!(
        "{} > {:?}",
        self, self
    ))}
    fn partial(&self, value: Object) -> Object {crate::stdout::chore(format!(
        "{} > {:?}",
        value, value
    )); return value}
}

//> CONTEXT -> OBJECT
#[derive(Clone)]
pub enum Object {
    Infinite(crate::Infinite),
    Nexists(crate::Nexists),
    Number(crate::Number),
    Tensor(crate::Tensor),
    Undefined(crate::Undefined),
    Variable(crate::Variable)
} impl Object {
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
    pub fn unequivalency(&self, to: &Object) -> bool {self.genlocale0(to); return match self {
        Object::Infinite(item) => item.unequivalency(to),
        Object::Nexists(item) => item.unequivalency(to),
        Object::Number(item) => item.unequivalency(to),
        Object::Tensor(item) => item.unequivalency(to),
        Object::Undefined(item) => item.unequivalency(to),
        Object::Variable(item) => item.unequivalency(to)
    }}
    pub fn equivalency(&self, to: &Object) -> bool {self.genlocale1(to); return match self {
        Object::Infinite(item) => item.equivalency(to),
        Object::Nexists(item) => item.equivalency(to),
        Object::Number(item) => item.equivalency(to),
        Object::Tensor(item) => item.equivalency(to),
        Object::Undefined(item) => item.equivalency(to),
        Object::Variable(item) => item.equivalency(to)
    }}
    pub fn negate(&self) -> Object {self.genlocale2(); return match self {
        Object::Infinite(item) => item.negate(),
        Object::Nexists(item) => item.negate(),
        Object::Number(item) => item.negate(),
        Object::Tensor(item) => item.negate(),
        Object::Undefined(item) => item.negate(),
        Object::Variable(item) => item.negate()
    }}
    pub fn summation(&self, to: &Object) -> Object {self.genlocale3(to); return match self {
        Object::Infinite(item) => item.summation(to),
        Object::Nexists(item) => item.summation(to),
        Object::Number(item) => item.summation(to),
        Object::Tensor(item) => item.summation(to),
        Object::Undefined(item) => item.summation(to),
        Object::Variable(item) => item.summation(to)
    }}
    pub fn invert(&self) -> Object {self.genlocale4(); return match self {
        Object::Infinite(item) => item.invert(),
        Object::Nexists(item) => item.invert(),
        Object::Number(item) => item.invert(),
        Object::Tensor(item) => item.invert(),
        Object::Undefined(item) => item.invert(),
        Object::Variable(item) => item.invert()
    }}
    pub fn multiplication(&self, to: &Object) -> Object {self.genlocale5(to); return match self {
        Object::Infinite(item) => item.multiplication(to),
        Object::Nexists(item) => item.multiplication(to),
        Object::Number(item) => item.multiplication(to),
        Object::Tensor(item) => item.multiplication(to),
        Object::Undefined(item) => item.multiplication(to),
        Object::Variable(item) => item.multiplication(to),
    }}
} impl crate::Display for Object {fn fmt(&self, formatter: &mut crate::Formatter<'_>) -> crate::Result {match self {
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

//> CONTEXT -> STRUCT
pub struct Context {
    cache: Vec<Object>,
    pub mutable: crate::HashMap<String, Object>,
    pub immutable: crate::HashMap<String, Object>,
    pub types: crate::HashMap<String, u8>
}

//> CONTEXT -> IMPLEMENTATION
impl Context {
    pub fn new(length: usize) -> Self {
        let mut instance = Context {
            cache: Vec::with_capacity(length),
            mutable: crate::HashMap::new(),
            immutable: crate::HashMap::new(),
            types: crate::HashMap::new()
        };
        for index in 0..(length) {instance.cache.push(Object::Nexists(crate::Nexists {}))};
        return instance;
    }
    fn set(&mut self, id: u32, value: Object) {self.cache[(id as usize) - 1] = value}
    pub fn read(&self, id: u32) -> Object {
        if id == 0 {return Object::Nexists(crate::Nexists {})}
        return self.cache[(id as usize) - 1].clone();
    }
    pub fn process(&mut self, id: u32, memory: &Vec<Box<dyn crate::reparser::Class>>) -> () {
        let item = &memory[(id as usize) - 1];
        let output = item.evaluate(self, id, memory);
        self.set(id, output);
    }
    pub fn quick(&mut self, memory: &Vec<Box<dyn crate::reparser::Class>>) -> () {
        self.process(memory.len() as u32, &memory);
    }
}