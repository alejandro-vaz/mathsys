//^
//^ HEAD
//^

//> HEAD -> CROSS-SCOPE TRAIT
use crate::object::Object;
use crate::class::Class;
use crate::group::Group;


//^
//^ RUNTIME
//^

//> RUNTIME -> STRUCT
pub struct Runtime {
    pub mutable: crate::HashMap<String, Object>,
    pub immutable: crate::HashMap<String, Object>,
    pub types: crate::HashMap<String, Group>
}

//> RUNTIME -> IMPLEMENTATION
impl Runtime {
    pub fn new() -> Self {return Runtime {
        mutable: crate::HashMap::new(),
        immutable: crate::HashMap::new(),
        types: crate::HashMap::new()
    }}
    pub fn get(&mut self, id: u32, memory: &Vec<Class>) -> Object {
        if id <= 0 {return Object::Nexists(crate::Nexists {})}
        let item = &memory[(id as usize) - 1];
        return item.evaluate(self, id, memory);
    }
    pub fn start(&mut self, memory: &Vec<Class>) -> Object {self.get(memory.len() as u32, memory)}
}