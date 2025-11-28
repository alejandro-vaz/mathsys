//^
//^ CONTEXT
//^

//> CONTEXT -> VALUE
pub trait Value: crate::Display + crate::Debug {
    fn id(&self) -> &'static str;
    fn info(&self) -> () {crate::stdout::debug(crate::format!(
        "{} > {:?}",
        self, self
    ))}
    fn ctrlcv(&self) -> crate::Box<dyn Value>;
    fn unequivalency(&self, to: &crate::Box<dyn Value>) -> bool;
    fn equivalency(&self, to: &crate::Box<dyn Value>) -> bool;
    fn negate(&self) -> crate::Box<dyn Value>;
    fn summation(&self, to: &crate::Box<dyn Value>) -> crate::Box<dyn Value>;
    fn invert(&self) -> crate::Box<dyn Value>;
    fn multiplication(&self, to: &crate::Box<dyn Value>) -> crate::Box<dyn Value>;
    fn partial(&self, value: crate::Box<dyn Value>) -> crate::Box<dyn Value> {
        crate::stdout::chore(crate::format!(
            "{} > {:?}",
            value, value
        ));
        return value;
    }
    fn genlocale0(&self, to: &crate::Box<dyn Value>) -> () {crate::stdout::trace(crate::format!(
        "Checking unequivalency of {} and {}",
        self, to
    )); self.info(); to.info()}
    fn genlocale1(&self, to: &crate::Box<dyn Value>) -> () {crate::stdout::trace(crate::format!(
        "Checking equivalency of {} and {}",
        self, to
    )); self.info(); to.info()}
    fn genlocale2(&self) -> () {crate::stdout::trace(crate::format!(
        "Negating {}",
        self
    )); self.info()}
    fn genlocale3(&self, to: &crate::Box<dyn Value>) -> () {crate::stdout::trace(crate::format!(
        "Adding {} and {}",
        self, to
    )); self.info(); to.info()}
    fn genlocale4(&self) -> () {crate::stdout::trace(crate::format!(
        "Inverting {}",
        self
    )); self.info()}
    fn genlocale5(&self, to: &crate::Box<dyn Value>) -> () {crate::stdout::trace(crate::format!(
        "Multiplying {} and {}",
        self, to
    )); self.info(); to.info()}
}

//> CONTEXT -> ID
pub trait Id {
    const ID: &'static str;
}

//> CONTEXT -> STRUCT
pub struct Context {
    cache: crate::Vec<crate::Box<dyn Value>>,
    pub mutable: crate::Vec<(crate::String, crate::Box<dyn Value>)>,
    pub immutable: crate::Vec<(crate::String, crate::Box<dyn Value>)>,
    pub types: crate::Vec<(crate::String, u8)>
}

//> CONTEXT -> IMPLEMENTATION
impl Context {
    pub fn new(length: usize) -> Self {
        let mut instance = Context {
            cache: crate::Vec::with_capacity(length),
            mutable: crate::Vec::new(),
            immutable: crate::Vec::new(),
            types: crate::Vec::new()
        };
        for index in 0..(length) {instance.cache.push(crate::Box::new(crate::Nexists {}))};
        return instance;
    }
    fn set(&mut self, id: u32, value: crate::Box<dyn Value>) {self.cache[(id as usize) - 1] = value}
    pub fn read(&self, id: u32) -> crate::Box<dyn Value> {
        if id == 0 {return crate::Box::new(crate::Nexists {})}
        return self.cache[(id as usize) - 1].ctrlcv();
    }
    pub fn process(&mut self, id: u32, memory: &crate::Vec<crate::Box<dyn crate::converter::Class>>) -> () {
        let item = &memory[(id as usize) - 1];
        let output = item.evaluate(self, id, memory);
        //self.set(id, output);
    }
    pub fn quick(&mut self, memory: &crate::Vec<crate::Box<dyn crate::converter::Class>>) -> () {
        self.process(memory.len() as u32, &memory);
    }
}


//^
//^ DOWNCASTING
//^

//> DOWNCASTING -> MUTABLE FUNCTION
pub fn mutcast<Type: Id>(value: &mut dyn Value) -> &mut Type {
    if value.id() != Type::ID {crate::stdout::crash(crate::stdout::Code::FailedMutcast)} else {
        return unsafe {&mut *(value as *mut dyn Value as *mut Type)}
    }
}

//> DOWNCASTING -> STATIC FUNCTION
pub fn downcast<Type: Id>(value: &dyn Value) -> &Type {
    if value.id() != Type::ID {crate::stdout::crash(crate::stdout::Code::FailedDowncast)} else {
        return unsafe {&*(value as *const dyn Value as *const Type)}
    }
}