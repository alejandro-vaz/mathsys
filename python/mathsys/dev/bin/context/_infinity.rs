//^
//^ INFINITY
//^

//> INFINITY -> STRUCT
#[derive(Clone, Copy)]
pub struct _Infinity {}

//> INFINITY -> IMPLEMENTATION
impl crate::runtime::Value for _Infinity {
    fn id(&self) -> &'static str {"Infinity"}
    fn ctrlcv(&self) -> crate::Box<dyn crate::runtime::Value> {return crate::Box::new(self.clone())}
} 
impl crate::runtime::Id for _Infinity {const ID: &'static str = "Infinity";} 
impl _Infinity {}