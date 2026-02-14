//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    FilePath, readFile, writeFile
};

//> HEAD -> LOCAL
use super::issues::Issue;


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
#[derive(Clone, Debug)]
pub struct File {
    pub name: FilePath
} impl File {
    pub fn read(&self) -> Result<String, Issue> {readFile(&self.name).ok().ok_or_else(|| Issue::FileNotFound(self.name.to_str().unwrap().to_string()))}
    pub async fn write(&self, extension: &str, content: String) -> () {
        let mut path = self.name.clone();
        path.set_extension(extension);
        writeFile(path, content).unwrap();
    }
}

//> INPUT -> FLAG
#[derive(Clone, Debug)]
pub struct Flag {
    pub value: String
}

//> INPUT -> ALIAS
#[derive(Clone, Debug)]
pub struct Alias {
    pub letters: Vec<char>
}

//> INPUT -> TARGET
#[derive(Clone, Debug)]
pub struct Target {
    pub name: String
}