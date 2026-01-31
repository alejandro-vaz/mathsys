//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Regex, LazyLock, HashMap, IndexMap
};

//> HEAD -> LOCAL
use super::issues::{unknownToken, Issue, inputTooLong};


//^
//^ TOKENS
//^

//> TOKENS -> RESPONSIBILITY
#[derive(Clone, Copy, PartialEq, Eq)]
pub enum Responsibility {
    Null,
    Structural,
    Total
}

//> TOKENS -> KIND
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum Kind {
    IDENTIFIER,
    MODULE,
    NUMBER,
    OPERATOR,
    COMMENT,
    RATIONAL,
    SIGN,
    GROUP,
    BINDING,
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
} impl Kind {pub fn byname(name: &str) -> Kind {return match name {
    "IDENTIFIER" => Kind::IDENTIFIER,
    "MODULE" => Kind::BINDING,
    "NUMBER" => Kind::NUMBER,
    "OPERATOR" => Kind::OPERATOR,
    "COMMENT" => Kind::COMMENT,
    "RATIONAL" => Kind::RATIONAL,
    "SIGN" => Kind::SIGN,
    "GROUP" => Kind::GROUP,
    "BINDING" => Kind::BINDING,
    "CLOSE" => Kind::CLOSE,
    "COMMA" => Kind::COMMA,
    "ENTER" => Kind::ENTER,
    "EQUALITY" => Kind::EQUALITY,
    "EXIT" => Kind::EXIT,
    "EXPONENTIATION" => Kind::EXPONENTIATION,
    "INFINITE" => Kind::INFINITE,
    "LIMIT" => Kind::LIMIT,
    "NEWLINES" => Kind::NEWLINES,
    "OF" => Kind::OF,
    "OPEN" => Kind::OPEN,
    "PIPE" => Kind::PIPE,
    "SPACES" => Kind::SPACES,
    "TO" => Kind::TO,
    "UNDEFINED" => Kind::UNDEFINED,
    "USE" => Kind::USE,
    "ENDOFFILE" => Kind::ENDOFFILE,
    other => panic!("{other}")
}}}

//> TOKENS -> STRUCT
#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Token {
    start: u32,
    length: u8,
    pub kind: Kind
}

//> TOKENS -> ORDER
pub static ORDER: LazyLock<IndexMap<Kind, (Regex, Responsibility)>> = LazyLock::new(|| {[
    (Kind::UNDEFINED, (Regex::new(r#"^\?"#).unwrap(), Responsibility::Structural)),
    (Kind::LIMIT, (Regex::new("^lim").unwrap(), Responsibility::Structural)),
    (Kind::PIPE, (Regex::new(r#"^\|"#).unwrap(), Responsibility::Structural)),
    (Kind::TO, (Regex::new("^->").unwrap(), Responsibility::Structural)),
    (Kind::OF, (Regex::new("^of").unwrap(), Responsibility::Structural)),
    (Kind::INFINITE, (Regex::new("^inf").unwrap(), Responsibility::Structural)),
    (Kind::USE, (Regex::new("^use").unwrap(), Responsibility::Structural)),
    (Kind::GROUP, (Regex::new(r#"^\@(Infinite|Integer|Natural|Nexists|Rational|Tensor|Undefined|Variable|Whole)"#).unwrap(), Responsibility::Total)),
    (Kind::IDENTIFIER, (Regex::new("^[A-Za-zº$%]+").unwrap(), Responsibility::Total)),
    (Kind::EXPONENTIATION, (Regex::new(r#"^\^"#).unwrap(), Responsibility::Structural)),
    (Kind::RATIONAL, (Regex::new(r#"^[0-9]*\.[0-9]+"#).unwrap(), Responsibility::Total)),
    (Kind::NUMBER, (Regex::new("^[0-9]+").unwrap(), Responsibility::Total)),
    (Kind::BINDING, (Regex::new("^==").unwrap(), Responsibility::Structural)),
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
    (Kind::MODULE, (Regex::new(r#"^"[a-z]+""#).unwrap(), Responsibility::Total)),
    (Kind::COMMENT, (Regex::new(r"^\#[^\n]*").unwrap(), Responsibility::Null)),
    (Kind::ENDOFFILE, (Regex::new("^$").unwrap(), Responsibility::Structural))
].into_iter().collect()});



//^
//^ TOKENIZER
//^

//> TOKENIZER -> MAXLEN
pub static MAXLEN: usize = 0x4096;

//> TOKENIZER -> STRUCT
pub struct Tokenizer {
    content: String,
    column: u16,
    line: u16,
    cursor: u32
} impl Tokenizer {
    pub fn new() -> Tokenizer {return Self {
        content: String::new(),
        column: 1,
        line: 1,
        cursor: 0
    }}
    fn reset(&mut self, parsing: String) -> Vec<Token> {
        self.content = parsing;
        self.column = 1;
        self.line = 1;
        self.cursor = 0;
        return Vec::with_capacity(MAXLEN);
    }
    pub fn run(&mut self, content: String) -> Result<Vec<Token>, Issue> {
        let mut tokens = self.reset(content);
        loop {
            let token = self.next()?;
            let length = token.length;
            let kind = token.kind;
            tokens.push(token);
            match kind {
                Kind::NEWLINES => {
                    self.line += length as u16;
                    self.column = 1;
                },
                Kind::ENDOFFILE => {
                    break;
                },
                other => {
                    self.column += length as u16;
                }
            }
            self.cursor += length as u32;
        };
        let total = tokens.len();
        return if total > MAXLEN {Err(inputTooLong(total))} else {Ok(tokens)};
    }
    #[inline(always)]
    fn next(&self) -> Result<Token, Issue> {
        let mut best: Option<Token> = None;
        let slice = &self.content[self.cursor as usize..];
        for (kind, pattern) in ORDER.iter().map(|item| (item.0, &item.1.0)) {if let Some(hit) = pattern.find(slice.as_bytes()) {
            if let None = best {best = Some(Token {
                start: self.cursor + 1,
                length: hit.len() as u8,
                kind: *kind
            }); continue}
            else if let Some(token) = &best && token.length < (hit.len() as u8) {best = Some(Token {
                start: self.cursor + 1,
                length: hit.len() as u8,
                kind: *kind
            })}
        }};
        return best.ok_or_else(|| unknownToken(
            self.line, 
            self.column, 
            self.content.lines().nth((self.line - 1) as usize).unwrap()
        ));
    }
}