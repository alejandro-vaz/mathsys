//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::{
    failure::Failure,
    tokenizer::token::Token
};


//^
//^ STATE
//^

//> STATE -> STRUCT
pub struct State<'valid> {
    tokens: Vec<Token<'valid>>,
    index: usize
}

//> STATE -> IMPLEMENTATION
impl<'valid> State<'valid> {
    pub fn advance<
        Return: 'valid,
        Mapping: FnOnce(&Token<'valid>) -> Option<Return>
    >(&mut self, mapping: Mapping) -> Result<Return, Failure<'valid>> {return match mapping(
        self.tokens.get(self.index).ok_or(Failure::TokenStreamDepleted)?
    ) {
        None => Err(Failure::TokenNotFound),
        Some(data) => {
            self.index += 1;
            Ok(data)
        }
    }}
    pub fn optional<Return: 'valid>(
        &mut self, 
        call: fn(&mut Self) -> Result<Return, Failure<'valid>>
    ) -> Option<Return> {
        let checkpoint = self.index;
        return match call(self) {
            Ok(value) => Some(value),
            Err(_) => {
                self.index = checkpoint;
                None
            }
        }
    }
    pub fn multiple<Return: 'valid>(
        &mut self,
        call: fn(&mut Self) -> Result<Return, Failure<'valid>>
    ) -> Vec<Return> {
        let mut items = Vec::new();
        return loop {match self.optional(call) {
            Some(value) => items.push(value),
            None => break items
        }}
    }
    pub fn more<Return: 'valid>(
        &mut self,
        call: fn(&mut Self) -> Result<Return, Failure<'valid>>
    ) -> Result<Vec<Return>, Failure<'valid>> {
        let items = self.multiple(call); 
        return match items.len() {
            1.. => Ok(items),
            0 => Err(Failure::CouldntParseMore)
        }
    }
}

//> STATE -> FROM TOKENS
impl<'valid> From<Vec<Token<'valid>>> for State<'valid> {
    fn from(value: Vec<Token<'valid>>) -> Self {return Self {
        tokens: value,
        index: 0
    }}
}