//^
//^ HEAD
//^

//> HEAD -> LOCAL
use super::level1::Level1;


//^
//^ START
//^

//> START -> CLASS
pub struct Start {
    stream: Vec<Level1>
} impl Start {
    pub fn summon(items: Vec<Level1>) -> Self {return Start {
        stream: items
    }}
    pub fn merge(&mut self, with: Start) -> () {self.stream.extend(with.stream)}
}