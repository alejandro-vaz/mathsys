//^
//^ CONTEXT
//^

//> CONTEXT -> VALUE
pub trait Value {
    fn id(&self) -> &'static str;
    fn ctrlcv(&self) -> crate::Box<dyn Value>;
    fn equiv(&self, to: crate::Box<dyn Value>) -> bool;
    fn summation(&mut self, to: crate::Box<dyn Value>, inverse: bool, selfinverse: bool) -> crate::Box<dyn Value>;
    fn locale(&self, code: u8) -> ();
    fn genlocale(&self, code: u8) -> () {match code {
        0 => crate::stdout::trace(&crate::format!(
            "Copying element of type {}",
            self.id()
        )),
        1 => crate::stdout::trace(&crate::format!(
            "Checking equivalency of an element of type {}",
            self.id()
        )),
        2 => crate::stdout::trace(&crate::format!(
            "Adding an element of type {}",
            self.id()
        )),
        other => crate::stdout::crash(crate::stdout::Code::LocaleNotFound)
    }}
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
    pub immutable: crate::Vec<(crate::String, crate::Box<dyn Value>)>
}

//> CONTEXT -> IMPLEMENTATION
impl<'a> Context<'a> {
    pub fn new(size: usize, memory: &'a crate::Vec<crate::Box <dyn crate::converter::Class>>) -> Self {
        let mut instance = Context {
            cache: crate::Vec::with_capacity(size),
            memory: memory,
            mutable: crate::Vec::new(),
            immutable: crate::Vec::new()
        };
        for index in 0..size {instance.cache.push(crate::Box::new(crate::_Nexists {}))};
        return instance;
    }
    fn set(&mut self, id: u32, value: crate::Box<dyn Value>) {self.cache[(id as usize) - 1] = value}
    fn get(&self, id: u32) -> &dyn Value {
        if id == 0 {return &crate::_Nexists {}}
        crate::stdout::trace(&crate::format!(
            "Retrieving object with ID {}",
            id
        ));
        return &*self.cache[(id as usize) - 1]
    }
    pub fn read(&self, id: u32) -> crate::Box<dyn Value> {
        if id == 0 {return (crate::_Nexists {}).ctrlcv()}
        crate::stdout::trace(&crate::format!(
            "Reading object with ID {}",
            id
        ));
        return self.cache[(id as usize) - 1].ctrlcv();
    }
    pub fn process(&mut self, id: u32) -> () {
        let item = &self.memory[(id as usize) - 1];
        crate::stdout::space(&crate::format!(
            "[{}] Processing ID {}",
            item.name(),
            id
        ));
        let output = item.evaluate(self);
        self.set(id, output);
    }
    pub fn quick(&mut self) -> &dyn Value {
        for element in self.memory.iter().rev().take(1) {
            if element.as_ref().name() == "Start" {
                let index = self.memory.len() as u32;
                self.process(index);
                return self.get(index);
            }
        }
        crate::stdout::crash(crate::stdout::Code::StartNotFound);
    }
}


//^
//^ DOWNCASTING
//^

//> DOWNCASTING -> IMMUTABLE FUNCTION
pub fn downcast<Type: Id>(value: &dyn Value) -> &Type {
    crate::stdout::trace(&crate::format!(
        "Downcasting a {}",
        Type::ID
    ));
    if value.id() != Type::ID {crate::stdout::crash(crate::stdout::Code::FailedDowncast)} else {
        return unsafe {&*(value as *const dyn Value as *const Type)}
    }
}

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