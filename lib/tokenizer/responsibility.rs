//^
//^ HEAD
//^

//> HEAD -> ENUM_AS_INNER
use enum_as_inner::EnumAsInner;


//^
//^ RESPONSIBILITY
//^

//> RESPONSIBILITY -> ENUM
#[derive(EnumAsInner)]
pub enum Responsibility {
    Null,
    Structural,
    Full
}