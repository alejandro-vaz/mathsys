//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    Regex, LazyLock, IndexMap, RegexSet
};

//> HEAD -> LOCAL
use super::super::Settings;
use super::issues::{UnknownToken, Issue, InputTooLong};


//^
//^ TOKENS
//^

//> TOKENS -> RESPONSIBILITY
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

//> TOKENS -> BINDED
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct BindedToken<'processing> {
    start: u64,
    value: &'processing str,
    pub kind: Kind
} impl<'processing> BindedToken<'processing> {
    #[inline(always)]
    fn new(start: u64, value: &'processing str, kind: Kind) -> Self {return BindedToken {
        start: start,
        value: value,
        kind: kind
    }}
    #[inline(always)]
    pub fn fixate(self) -> ShallowToken {return ShallowToken {
        start: self.start,
        kind: self.kind
    }}
}

//> TOKENS -> SHALLOW
#[derive(Debug)]
pub struct ShallowToken {
    start: u64,
    kind: Kind
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

//> TOKENS -> REGEXSET
static REGEXSET: LazyLock<RegexSet> = LazyLock::new(|| RegexSet::new(ORDER.iter().map(|each| each.1.0.as_str())).unwrap());


//^
//^ TOKENIZER
//^

//> TOKENIZER -> MAXLEN
pub static MAXLEN: usize = 0xFFF;

//> TOKENIZER -> STRUCT
pub struct Tokenizer {
    column: u32 = 1,
    line: u32 = 1,
    cursor: u64 = 0
} impl Tokenizer {
    pub fn new() -> Tokenizer {return Self {..}}
    pub fn run<'tokenizing>(&mut self, content: &'tokenizing str, settings: &Settings) -> Result<Vec<BindedToken<'tokenizing>>, Issue> {
        self.column = 1;
        self.line = 1;
        self.cursor = 0;
        let mut tokens = Vec::with_capacity(MAXLEN);
        while tokens.len() != MAXLEN {
            let (token, length) = self.next(content).ok_or_else(|| UnknownToken(
                self.line, 
                self.column, content.lines().nth(self.line as usize - 1).unwrap()
            ))?;
            match token.kind {
                Kind::NEWLINES => {self.line += length as u32; self.column = 1},
                Kind::ENDOFFILE => {tokens.push(token); return Ok(tokens)},
                other => self.column += length as u32
            }
            tokens.push(token);
            self.cursor += length as u64;
        };
        return Err(InputTooLong());
    }
    #[inline(always)]
    fn next<'tokenizing>(&self, content: &'tokenizing str) -> Option<(BindedToken<'tokenizing>, usize)> {
        let mut best: Option<(Kind, usize)> = None;
        let slice = content[self.cursor as usize..].as_bytes();
        for chance in REGEXSET.matches_at(slice, 0) {
            let current = ORDER.get_index(chance).unwrap();
            let (kind, length) = (current.0, current.1.0.find_at(slice, 0).unwrap().len());
            if best.is_none() || best.unwrap().1 < length {best = Some((*kind, length))}
        }
        return if let Some(data) = best {Some((BindedToken::new(self.cursor, str::from_utf8(&slice[..data.1]).unwrap(), data.0), data.1))} else {None}
    }
}