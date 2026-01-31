//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    HashMap, Object, Group, stdout, Code, Pointer, Class, HashSet
};


//^
//^ RUNTIME
//^

//> RUNTIME -> STRUCT
pub struct Runtime {
    cycle: HashSet<Pointer>,
    pub mutable: HashMap<String, Object>,
    pub immutable: HashMap<String, Object>,
    pub types: HashMap<String, Group>
}

//> RUNTIME -> IMPLEMENTATION
impl Runtime {
    pub fn new() -> Self {return Runtime {
        cycle: HashSet::new(),
        mutable: HashMap::new(),
        immutable: HashMap::new(),
        types: HashMap::new()
    }}
    pub fn get(&mut self, id: Pointer, memory: &Vec<Class>) -> Object {
        if id.0 >= memory.len() as u32 {stdout.crash(Code::RuntimeHigherObject)}
        if self.cycle.contains(&id) {stdout.crash(Code::CyclicEvaluation)};
        self.cycle.insert(id);
        let result = memory[id.0 as usize].evaluate(self, id, memory);
        self.cycle.remove(&id);
        return result;
    }
    pub fn start(&mut self, memory: &Vec<Class>) -> Object {self.get(Pointer(memory.len() as u32 - 1), memory)}
}