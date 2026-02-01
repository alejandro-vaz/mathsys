//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Regex, LazyLock, IndexMap
};

//> HEAD -> LOCAL
use super::issues::{unknownToken, Issue, inputTooLong};


//^
//^ TOKENS
//^

//> TOKENS -> RESPONSIBILITY
#[derive(PartialEq, Eq)]
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
}

//> TOKENS -> STRUCT
#[derive(Debug)]
pub struct Token {
    start: u32,
    pub kind: Kind
} impl Token {pub fn process(start: u32, kind: Kind) -> Self {return Token {
    start: start,
    kind: kind
}}}

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
pub static MAXLEN: usize = 0xFFF;

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
            if tokens.len() == MAXLEN {return Err(inputTooLong())};
            let (token, length) = self.next()?;
            let kind = token.kind;
            tokens.push(token);
            match kind {
                Kind::NEWLINES => {self.line += length as u16; self.column = 1},
                Kind::ENDOFFILE => break,
                other => self.column += length as u16
            }
            self.cursor += length as u32;
        };
        return Ok(tokens);
    }
    #[inline(always)]
    fn next(&self) -> Result<(Token, usize), Issue> {
        let mut best: Option<(Kind, usize)> = None;
        let slice = &self.content[self.cursor as usize..];
        for (kind, pattern) in ORDER.iter().map(|item| (item.0, &item.1.0)) {
            if let Some(hit) = pattern.find(slice.as_bytes()) {if best.is_none() || best.unwrap().1 < hit.len() {
                best = Some((*kind, hit.len()))
            }}
        };
        let data = best.ok_or_else(|| unknownToken(
            self.line, 
            self.column, 
            self.content.lines().nth((self.line - 1) as usize).unwrap()
        ))?;
        return Ok((Token::process(self.cursor, data.0), data.1));
    }
}