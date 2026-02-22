//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    FilePath, 
    readFile, 
    writeFile,
    EnumString,
    AsRefStr
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
#[derive(Clone)]
pub struct File (pub FilePath); impl File {
    pub fn read(&self) -> Result<String, Issue> {readFile(&self.0).ok().ok_or_else(|| Issue::FileNotFound(self.0.to_str().unwrap().to_string()))}
    pub fn write(&self, extension: &str, content: String) -> () {
        let mut path = self.0.clone();
        path.set_extension(extension);
        writeFile(path, content).unwrap();
    }
}

//> INPUT -> FLAG
#[derive(EnumString, Clone, Copy, AsRefStr)]
#[strum(serialize_all = "lowercase")]
pub enum Flag {
    Quiet,
    Verbose
}

//> INPUT -> ALIAS
#[derive(Clone, Debug)]
pub struct Alias(pub Vec<char>); impl Alias {pub fn expand(self) -> Result<Vec<Flag>, Issue> {
    let mut converted = Vec::new();
    for letter in &self.0 {converted.push(match letter {
        'q' => Flag::Quiet,
        'v' => Flag::Verbose,
        other => return Err(Issue::UnknownAliasCharacter {
            alias: self, 
            at: converted.len()
        })
    })}
    return Ok(converted);
}}

//> INPUT -> TARGET
#[derive(EnumString, Clone, AsRefStr, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum Target {
    Help,
    Version,
    Tokens,
    Check,
    Ast,
    Latex
}