//^ 
//^ HEAD
//^

//> HEAD -> LOCAL
use super::start::Start;
use super::level2::Level2;


//^
//^ 1ºLEVEL
//^

//> 1ºLEVEL -> NAMESPACE
pub enum Level1 {
    Declaration(Declaration),
    Definition(Definition),
    Annotation(Annotation),
    Node(Node),
    Equation(Equation),
    Use(Use)
}

//> 1ºLEVEL -> DECLARATION
pub struct Declaration {
} impl Declaration {
    pub fn summon() -> Self {return Declaration {}}
}

//> 1ºLEVEL -> DEFINITION
pub struct Definition {
} impl Definition {
    pub fn summon() -> Self {return Definition {}}
}

//> 1ºLEVEL -> ANNOTATION
pub struct Annotation {
    
} impl Annotation {
    pub fn summon() -> Self {return Annotation {}}
}

//> 1ºLEVEL -> NODE
pub struct Node {
} impl Node {
    pub fn summon() -> Self {return Node {}}
}

//> 1ºLEVEL -> EQUATION
pub struct Equation {
} impl Equation {
    pub fn summon() -> Self {return Equation {}}
}

//> 1ºLEVEL -> USE
pub struct Use {
    start: Option<Start>
} impl Use {
    pub fn summon() -> Self {return Use {
        start: None
    }}
}