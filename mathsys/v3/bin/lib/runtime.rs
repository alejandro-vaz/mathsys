//^
//^ CONTEXT
//^

//> CONTEXT -> VALUE
pub trait Value {
    fn id(&self) -> &'static str;
    fn info(&self) -> ();
    fn ctrlcv(&self) -> crate::Box<dyn Value>;
    fn equiv(&self, to: crate::Box<dyn Value>) -> bool;
    fn summation(&mut self, to: crate::Box<dyn Value>, inverse: bool, selfinverse: bool) -> crate::Box<dyn Value>;
    fn locale(&self, code: u8) -> ();
    fn genlocale(&self, code: u8) -> () {match code {
        0 => crate::stdout::trace(&crate::format!(
            "Copying {}",
            self.id()
        )),
        1 => crate::stdout::trace(&crate::format!(
            "Checking equivalency of {}",
            self.id()
        )),
        2 => crate::stdout::trace(&crate::format!(
            "Adding {}",
            self.id()
        )),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }; self.info()}
}

//> CONTEXT -> ID
pub trait Id {
    const ID: &'static str;
}

//> CONTEXT -> STRUCT
pub struct Context<'a> {
    cache: crate::Vec<crate::Box<dyn Value>>,
    memory: &'a crate::Vec<crate::Box <dyn crate::converter::Class>>,
    pub mutable: crate::Vec<(crate::String, crate::Box<dyn Value>)>,
    pub immutable: crate::Vec<(crate::String, crate::Box<dyn Value>)>,
    pub types: crate::Vec<(crate::String, u8)>
}

//> CONTEXT -> IMPLEMENTATION
impl<'a> Context<'a> {
    pub fn new(memory: &'a crate::Vec<crate::Box <dyn crate::converter::Class>>) -> Self {
        let mut instance = Context {
            cache: crate::Vec::with_capacity(memory.len()),
            memory: memory,
            mutable: crate::Vec::new(),
            immutable: crate::Vec::new(),
            types: crate::Vec::new()
        };
        for index in 0..(memory.len()) {instance.cache.push(crate::Box::new(crate::Nexists {}))};
        return instance;
    }
    fn set(&mut self, id: u32, value: crate::Box<dyn Value>) {self.cache[(id as usize) - 1] = value}
    pub fn read(&self, id: u32) -> crate::Box<dyn Value> {
        crate::stdout::trace(&crate::format!(
            "Reading object with ID {}",
            id
        ));
        if id == 0 {return crate::Box::new(crate::Nexists {})}
        return self.cache[(id as usize) - 1].ctrlcv();
    }
    pub fn process(&mut self, id: u32) -> () {
        let item = &self.memory[(id as usize) - 1];
        let output = item.evaluate(self, id);
        self.set(id, output);
    }
    pub fn quick(&mut self) -> crate::Box<dyn Value> {
        for element in self.memory.iter().rev().take(1) {
            if element.as_ref().name() == "_Start" {
                let index = self.memory.len() as u32;
                self.process(index);
                crate::stdout::space("{RUNTIME} Shutdown");
                return self.read(index);
            }
        }
        crate::stdout::crash(crate::stdout::Code::StartNotFound);
    }
}


//^
//^ DOWNCASTING
//^

//> DOWNCASTING -> MUTABLE FUNCTION
pub fn mutcast<Type: Id>(value: &mut dyn Value) -> &mut Type {
    crate::stdout::trace(&crate::format!(
        "Mutcasting a {}",
        Type::ID
    ));
    if value.id() != Type::ID {crate::stdout::crash(crate::stdout::Code::FailedMutcast)} else {
        return unsafe {&mut *(value as *mut dyn Value as *mut Type)}
    }
}