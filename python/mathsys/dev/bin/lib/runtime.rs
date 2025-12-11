//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::object::Object;
use crate::class::Class;


//^
//^ CONTEXT
//^

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
    pub fn process(&mut self, id: u32, memory: &Vec<Class>) -> () {
        let item = &memory[(id as usize) - 1];
        let output = item.evaluate(self, id, memory);
        self.set(id, output);
    }
    pub fn quick(&mut self, memory: &Vec<Class>) -> () {
        self.process(memory.len() as u32, &memory);
    }
}