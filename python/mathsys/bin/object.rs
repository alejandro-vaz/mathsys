//
//  OBJECT
//

// OBJECT -> FACTOR
pub struct Factor {
    pub sign: *mut u8,
    pub precision: *mut u8,
    pub value: *mut u8
}

// OBJECT -> EXPRESSION
pub struct Expression {
    pub left: *mut u8,
    pub operator: *mut u8,
    pub right: *mut u8
}

// OBJECT -> PLACEHOLDER
pub struct Placeholder {}

// OBJECT -> NODE
pub enum Node {
    Factor(Factor),
    Expression(Expression),
    Placeholder(Placeholder)
}