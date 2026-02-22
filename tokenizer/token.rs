//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    LazyLock, 
    IndexMap, 
    Regex
};

//^ 
//^ TOKEN
//^ 

//> TOKEN -> RESPONSIBILITY
pub(crate) enum Responsibility {
    Null,
    Structural,
    Total
}

//> TOKEN -> KIND
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub(crate) enum Kind {
    IDENTIFIER,
    MODULE,
    NUMBER,
    OPERATOR,
    COMMENT,
    RATIONAL,
    SIGN,
    DEFINITION,
    CLOSE,
    COMMA,
    ENTER,
    EQUALITY,
    EXIT,
    EXPONENTIATION,
    INFINITE,
    LIMIT,
    NEWLINES,
    OF,
    OPEN,
    PIPE,
    SPACES,
    TO,
    UNDEFINED,
    USE,
    ENDOFFILE
}

//> TOKEN -> ORDER
pub(crate) static ORDER: LazyLock<IndexMap<Kind, (Regex, Responsibility)>> = LazyLock::new(|| {[
    (Kind::UNDEFINED, (Regex::new(r#"^\?"#).unwrap(), Responsibility::Structural)),
    (Kind::LIMIT, (Regex::new("^lim").unwrap(), Responsibility::Structural)),
    (Kind::PIPE, (Regex::new(r#"^\|"#).unwrap(), Responsibility::Structural)),
    (Kind::TO, (Regex::new("^->").unwrap(), Responsibility::Structural)),
    (Kind::OF, (Regex::new("^of").unwrap(), Responsibility::Structural)),
    (Kind::INFINITE, (Regex::new("^inf").unwrap(), Responsibility::Structural)),
    (Kind::USE, (Regex::new("^use").unwrap(), Responsibility::Structural)),
    (Kind::IDENTIFIER, (Regex::new("^[A-Za-zº$%]+").unwrap(), Responsibility::Total)),
    (Kind::EXPONENTIATION, (Regex::new(r#"^\^"#).unwrap(), Responsibility::Structural)),
    (Kind::RATIONAL, (Regex::new(r#"^[0-9]*\.[0-9]+"#).unwrap(), Responsibility::Total)),
    (Kind::NUMBER, (Regex::new("^[0-9]+").unwrap(), Responsibility::Total)),
    (Kind::DEFINITION, (Regex::new("^:=").unwrap(), Responsibility::Structural)),
    (Kind::EQUALITY, (Regex::new("^=").unwrap(), Responsibility::Structural)),
    (Kind::OPERATOR, (Regex::new(r#"^[\*\/]"#).unwrap(), Responsibility::Total)),
    (Kind::SIGN, (Regex::new("^[+-]").unwrap(), Responsibility::Total)),
    (Kind::OPEN, (Regex::new(r#"^\("#).unwrap(), Responsibility::Structural)),
    (Kind::CLOSE, (Regex::new(r#"^\)"#).unwrap(), Responsibility::Structural)),
    (Kind::ENTER, (Regex::new(r#"^\["#).unwrap(), Responsibility::Structural)),
    (Kind::COMMA, (Regex::new(r#"^,"#).unwrap(), Responsibility::Structural)),
    (Kind::EXIT, (Regex::new(r#"^\]"#).unwrap(), Responsibility::Structural)),
    (Kind::SPACES, (Regex::new("^ +").unwrap(), Responsibility::Structural)),
    (Kind::NEWLINES, (Regex::new(r#"^\n+"#).unwrap(), Responsibility::Structural)),
    (Kind::MODULE, (Regex::new(r#"^"[a-z]+\.msm""#).unwrap(), Responsibility::Total)),
    (Kind::COMMENT, (Regex::new(r"^\#[^\n]*").unwrap(), Responsibility::Null)),
    (Kind::ENDOFFILE, (Regex::new("^$").unwrap(), Responsibility::Structural))
].into_iter().collect()});


//^
//^ INSTANCES
//^

//> INSTANCES -> BINDED
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub(crate) struct BindedToken<'processing> {
    start: u64,
    pub(crate) value: &'processing str,
    pub(crate) kind: Kind
} impl<'processing> BindedToken<'processing> {
    #[inline(always)]
    pub(super) fn new(start: u64, value: &'processing str, kind: Kind) -> Self {return BindedToken {
        start: start,
        value: value,
        kind: kind
    }}
    #[inline(always)]
    pub(crate) fn fixate(self) -> ShallowToken {return ShallowToken {
        start: self.start,
        kind: self.kind
    }}
}

//> INSTANCES -> SHALLOW
#[derive(Debug)]
pub struct ShallowToken {
    start: u64,
    kind: Kind
}