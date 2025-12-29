//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    HashMap,
    Object,
    Group,
    crash,
    Code,
    Pointer,
    Nexists,
    Class
};


//^
//^ RUNTIME
//^

//> RUNTIME -> STRUCT
pub struct Runtime {
    pub mutable: HashMap<String, Object>,
    pub immutable: HashMap<String, Object>,
    pub types: HashMap<String, Group>
}

//> RUNTIME -> IMPLEMENTATION
impl Runtime {
    pub fn new() -> Self {return Runtime {
        mutable: HashMap::new(),
        immutable: HashMap::new(),
        types: HashMap::new()
    }}
    pub fn get(&mut self, id: Pointer, memory: &Vec<Class>) -> Object {
        if id.0 == 0 {return Object::Nexists(Nexists {})}
        if id.0 > memory.len() as u32 {crash(Code::RuntimeHigherObject)}
        let item = &memory[(id.0 as usize) - 1];
        return item.evaluate(self, id, memory);
    }
    pub fn start(&mut self, memory: &Vec<Class>) -> Object {self.get(Pointer(memory.len() as u32), memory)}
}