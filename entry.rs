//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    FilePath, VERSION, read, write
};


//^
//^ ARGUMENT
//^

//> ARGUMENT -> ENUM
pub enum Argument {
    File(File),
    Flag(Flag),
    Alias(Alias),
    Target(Target)
}


//^
//^ INPUT
//^

//> INPUT -> FILE
#[derive(Clone)]
pub struct File {
    pub name: FilePath
} impl File {
    pub fn read(&self) -> String {read(&self.name).unwrap()}
    pub async fn write(&self, extension: &str, content: String) -> () {
        let mut path = self.name.clone();
        path.set_extension(extension);
        write(path, content).unwrap();
    }
    pub fn version(&self) -> usize {
        let after = self.name.to_str().unwrap()[2..].to_string();
        match after.chars().last().unwrap() {
            'd' => VERSION + 1,
            'r' => VERSION,
            other => after.parse().unwrap()
        }
    }
}

//> INPUT -> FLAG
#[derive(Clone)]
pub struct Flag {
    pub value: String
}

//> INPUT -> ALIAS
#[derive(Clone)]
pub struct Alias {
    pub letters: Vec<char>
}

//> INPUT -> TARGET
#[derive(Clone)]
pub struct Target {
    pub name: String
}