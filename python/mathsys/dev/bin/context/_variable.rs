//
//  VARIABLE
//

// VARIABLE -> STRUCT
#[derive(Clone)]
pub struct _Variable {
    pub name: crate::String
}

// VARIABLE -> IMPLEMENTATION
impl crate::runtime::Value for _Variable {fn id(&self) -> &'static str {"Variable"}} 
impl crate::runtime::Id for _Variable {const ID: &'static str = "Variable";} 
impl _Variable {
    pub fn set(
        &self, 
        value: crate::Box<dyn crate::runtime::Value>, 
        mutable: bool, 
        context: &mut crate::runtime::Context
    ) -> () {
        for (key, _data) in &context.immutable {if key == &self.name {crate::stdout::crash(3)}}
        if mutable {
            for (key, data) in &mut context.mutable {if *key == self.name {*data = value; return}}
            context.mutable.push((self.name.clone(), value));
        } else {
            context.immutable.push((self.name.clone(), value));
        }
    }
    pub fn get<'a>(&self, context: &'a crate::runtime::Context) -> &'a dyn crate::runtime::Value {
        for (key, value) in &context.immutable {if key == &self.name {return &**value}}
        for (key, value) in &context.mutable {if key == &self.name {return &**value}}
        crate::ALLOCATOR.tempSpace(|| {
            crate::stdout::alert(&crate::format!(
                "Variable \"{}\" is not defined",
                &self.name
            ))
        });
        return &crate::_Undefined {};
    }
}