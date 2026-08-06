//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod expression;
pub mod factor;
pub mod statement;
pub mod term;
pub mod value;

//> HEAD -> STATEMENT
use statement::Statement;


//^
//^ START
//^

//> START -> STRUCT
pub struct Start<'valid> {
    pub statements: Vec<Statement<'valid>>
}