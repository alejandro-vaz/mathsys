//^
//^ TYPES
//^

//> TYPES -> POINTER
#[derive(Clone, Copy)]
pub struct Pointer(pub u32);
impl crate::Display for Pointer {fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> crate::Result {write!(formatter,
    "{}",
    self.0
)}}
impl crate::Debug for Pointer {fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> crate::Result {write!(formatter,
    "{}",
    self.0
)}}