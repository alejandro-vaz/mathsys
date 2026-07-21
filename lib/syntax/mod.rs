//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod level1;
pub mod level2;
pub mod level3;
pub mod level4;
pub mod level5;

//> HEAD -> LEVEL1
use level1::Level1;


//^
//^ START
//^

//> START -> STRUCT
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Start<'valid> {
    pub stream: Vec<Level1<'valid>>
}