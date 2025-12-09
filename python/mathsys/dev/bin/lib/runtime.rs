//^
//^ CONTEXT
//^

//> CONTEXT -> VALUE
pub trait Value: crate::Display + crate::Debug + crate::Any {
    fn id(&self) -> &'static str;
    fn info(&self) -> () {crate::stdout::debug(format!(
        "{} > {:?}",
        self, self
    ))}
    fn ctrlcv(&self) -> Box<dyn Value>;
    fn unequivalency(&self, to: &Box<dyn Value>) -> bool;
    fn equivalency(&self, to: &Box<dyn Value>) -> bool;
    fn negate(&self) -> Box<dyn Value>;
    fn summation(&self, to: &Box<dyn Value>) -> Box<dyn Value>;
    fn invert(&self) -> Box<dyn Value>;
    fn multiplication(&self, to: &Box<dyn Value>) -> Box<dyn Value>;
    fn partial(&self, value: Box<dyn Value>) -> Box<dyn Value> {
        crate::stdout::chore(format!(
            "{} > {:?}",
            value, value
        ));
        return value;
    }
    fn genlocale0(&self, to: &Box<dyn Value>) -> () {crate::stdout::trace(format!(
        "Checking unequivalency of {} and {}",
        self, to
    )); self.info(); to.info()}
    fn genlocale1(&self, to: &Box<dyn Value>) -> () {crate::stdout::trace(format!(
        "Checking equivalency of {} and {}",
        self, to
    )); self.info(); to.info()}
    fn genlocale2(&self) -> () {crate::stdout::trace(format!(
        "Negating {}",
        self
    )); self.info()}
    fn genlocale3(&self, to: &Box<dyn Value>) -> () {crate::stdout::trace(format!(
        "Adding {} and {}",
        self, to
    )); self.info(); to.info()}
    fn genlocale4(&self) -> () {crate::stdout::trace(format!(
        "Inverting {}",
        self
    )); self.info()}
    fn genlocale5(&self, to: &Box<dyn Value>) -> () {crate::stdout::trace(format!(
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
    cache: Vec<Box<dyn Value>>,
    pub mutable: crate::HashMap<String, Box<dyn Value>>,
    pub immutable: crate::HashMap<String, Box<dyn Value>>,
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
        for index in 0..(length) {instance.cache.push(Box::new(crate::Nexists {}))};
        return instance;
    }
    fn set(&mut self, id: u32, value: Box<dyn Value>) {self.cache[(id as usize) - 1] = value}
    pub fn read(&self, id: u32) -> Box<dyn Value> {
        if id == 0 {return Box::new(crate::Nexists {})}
        return self.cache[(id as usize) - 1].ctrlcv();
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


//^
//^ DOWNCASTING
//^

//> DOWNCASTING -> MUTABLE FUNCTION
pub fn mutcast<Type: 'static>(value: &mut dyn Value) -> &mut Type {
    return (value as &mut dyn crate::Any).downcast_mut::<Type>().unwrap_or_else(
        || crate::stdout::crash(crate::stdout::Code::FailedMutcast)
    )
}

//> DOWNCASTING -> STATIC FUNCTION
pub fn downcast<Type: 'static>(value: &dyn Value) -> &Type {
    return (value as &dyn crate::Any).downcast_ref::<Type>().unwrap_or_else(
        || crate::stdout::crash(crate::stdout::Code::FailedDowncast)
    )
}