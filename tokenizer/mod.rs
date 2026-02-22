//^
//^ HEAD
//^

//> HEAD -> MODULES
pub(super) mod token;

//> HEAD -> PRELUDE
use super::prelude::{
    LazyLock, 
    RegexSet
};

//> HEAD -> LOCAL
use self::{
    token::{
        ORDER, 
        BindedToken, 
        Kind
    },
    super::{
        Settings,
        issues::Issue
    }
};


//^
//^ REGEX
//^

//> REGEX -> SET
static REGEXSET: LazyLock<RegexSet> = LazyLock::new(|| RegexSet::new(ORDER.iter().map(|each| each.1.0.as_str())).unwrap());


//^
//^ TOKENIZER
//^

//> TOKENIZER -> MAXLEN
pub static MAXLEN: usize = 0xFFF;

//> TOKENIZER -> POSITION
struct Position {
    column: u32 = 1,
    line: u32 = 1,
    cursor: u64 = 0
}

//> TOKENIZER -> STRUCT
pub(super) struct Tokenizer {} impl Tokenizer {
    pub(super) const fn new() -> Tokenizer {return Self {}}
    pub(super) fn run<'tokenizing>(&self, content: &'tokenizing str, settings: &Settings) -> Result<Vec<BindedToken<'tokenizing>>, Issue> {
        let mut position = Position {..};
        let mut tokens = Vec::with_capacity(MAXLEN);
        while tokens.len() != MAXLEN {
            let (token, length) = self.next(content, position.cursor).ok_or_else(|| Issue::UnknownToken {
                line: position.line, 
                column: position.column, 
                code: content.lines().nth(position.line as usize - 1).unwrap().to_string()
            })?;
            match token.kind {
                Kind::NEWLINES => {position.line += length as u32; position.column = 1},
                Kind::ENDOFFILE => {tokens.push(token); return Ok(tokens)},
                other => position.column += length as u32
            }
            tokens.push(token);
            position.cursor += length as u64;
        };
        return Err(Issue::InputTooLong);
    }
    #[inline(always)]
    fn next<'tokenizing>(&self, content: &'tokenizing str, cursor: u64) -> Option<(BindedToken<'tokenizing>, usize)> {
        let mut best: Option<(Kind, usize)> = None;
        let slice = content[cursor as usize..].as_bytes();
        for chance in REGEXSET.matches_at(slice, 0) {
            let current = ORDER.get_index(chance).unwrap();
            let (kind, length) = (current.0, current.1.0.find_at(slice, 0).unwrap().len());
            if best.is_none() || best.unwrap().1 < length {best = Some((*kind, length))}
        }
        return if let Some(data) = best {Some((BindedToken::new(cursor, str::from_utf8(&slice[..data.1]).unwrap(), data.0), data.1))} else {None}
    }
}