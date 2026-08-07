//^
//^ HEAD
//^

//> HEAD -> CRATE
use crate::failure::Failure;


//^
//^ STATE
//^

//> STATE -> STRUCT
pub struct State<'valid> {
    input: &'valid [u8],
    index: usize
}

//> STATE -> IMPLEMENTATION
impl<'valid> State<'valid> {
    pub fn advance(
        &mut self, 
        mapping: fn(u8) -> bool
    ) -> Result<u8, Failure<'valid>> {
        let byte = *self.input.get(self.index).ok_or(Failure::TokenStreamDepleted)?;
        return match mapping(byte) {
            false => Err(Failure::TokenNotFound),
            true => {
                self.index += 1;
                Ok(byte)
            }
        }
    }
    pub fn record(
        &mut self,
        filter: fn(u8) -> bool
    ) -> Result<&'valid str, Failure<'valid>> {
        let position = self.index;
        while let Some(&byte) = self.input.get(self.index) && filter(byte) {self.index += 1}
        return match self.index == position {
            false => Ok(str::from_utf8(
                &self.input[position..self.index]
            ).map_err(|_| Failure::NonUtf8Sequence)?),
            true => Err(Failure::TokenNotFound)
        }
    }
    pub fn skip(&mut self, byte: u8) -> () {
        while let Some(&next) = self.input.get(self.index) && next == byte {self.index += 1}
    }
    pub fn depleted<Return: 'valid>(
        &self, 
        value: Return
    ) -> Result<Return, Failure<'valid>> {return match self.input.len() == self.index {
        true => Ok(value),
        false => Err(Failure::UnfinishedInputParse)
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
impl<'valid> From<&'valid [u8]> for State<'valid> {
    fn from(value: &'valid [u8]) -> Self {return Self {
        input: value,
        index: 0
    }}
}