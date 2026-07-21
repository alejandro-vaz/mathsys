//^
//^ HEAD
//^

//> HEAD -> CORE
use core::num::NonZero;


//^
//^ POSITION
//^

//> POSITION -> STRUCT
#[derive(Clone, Copy)]
pub struct Position {
    pub cursor: usize = 0,
    pub line: NonZero<usize> = NonZero::new(1).unwrap(),
    pub column: NonZero<usize> = NonZero::new(1).unwrap()
}