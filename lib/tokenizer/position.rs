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
    pub line: NonZero<usize> = unsafe {NonZero::new_unchecked(1)},
    pub column: NonZero<usize> = unsafe {NonZero::new_unchecked(1)}
}