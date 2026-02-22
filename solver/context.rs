//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Set
};

//> HEAD -> LOCAL
use super::super::syntax::level5::Variable;


//^
//^ CONTEXT
//^

//> CONTEXT -> STRUCT
pub(crate) struct Context {
    functions: Set<String>
} impl Context {
    pub(super) fn new() -> Self {return Context {
        functions: Set::new()
    }}
    pub(crate) fn registerFn(&mut self, variable: &Variable) -> () {self.functions.insert(variable.name.clone());}
    pub(crate) fn isFn(&self, variable: &Variable) -> bool {self.functions.contains(&variable.name)}
}