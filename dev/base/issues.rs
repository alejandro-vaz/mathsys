//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    exit, Error, Display, Formatter, Rst, Colored, Flag, currentDir, readDir, Alias
};

//> HEAD -> LOCAL
use super::tokenizer::MAXLEN;
use super::super::{TARGETS, FLAGLIASES};


//^
//^ DEFAULTS
//^

//> DEFAULTS -> STYLE
enum Style {
    Code,
    Place,
    Object,
    Target,
    Flag,
    Alias
}

//> DEFAULTS -> COLOR
fn color(string: impl Colored, style: Style) -> String {return (match style {
    Style::Code => string.truecolor(0xAA, 0xAA, 0xAA),
    Style::Place => string.cyan(),
    Style::Object => string.purple().bold(),
    Style::Target => string.green(),
    Style::Flag => string.blue(),
    Style::Alias => string.yellow()
}).to_string()}


//^
//^ ISSUES
//^

//> ISSUES -> NEWIS
#[derive(Debug)]
pub enum Newis {
    UnknownToken {
        line: u32,
        column: u32,
        code: String
    }
} impl Newis {
    pub fn consume(self) -> ! {println!("{self:?}"); exit(&self as *const Newis as i32)}
    fn content(&self) -> String {match self {
        Newis::UnknownToken {line, column, code} => format!(
            "Unknown token at line {},\n\n    {}\n    {}",
            color(&line.to_string() as &str, Style::Place),
            color(code as &str, Style::Code),
            " ".repeat(*column as usize - 1) + &color("^", Style::Place)
        )
    }}
} impl Display for Newis {fn fmt(&self, formatter: &mut Formatter<'_>) -> Rst {
    let base = format!("{} {} {}{}{}{}", "Raised".red(), "issue", "issue:".red(), "\n>\n> ".red(), self.content().replace("\n", &"\n> ".red().to_string()), "\n>\n".red());
    write!(formatter, "{base}")
}} impl Error for Newis {}


//> ISSUES -> ISSUE
#[derive(Debug)]
pub struct Issue {
    name: &'static str,
    content: String
} impl Issue {
    pub fn is(name: &'static str, content: String) -> Self {Self {name: name, content: content}}
    pub fn consume(self) -> ! {println!("{self}"); exit(self.name.as_ptr() as usize as i32)}
} 
impl Display for Issue {fn fmt(&self, formatter: &mut Formatter<'_>) -> Rst {
    let base = format!("{} {} {}{}{}{}", "Raised".red(), color(self.name, Style::Object), "issue:".red(), "\n>\n> ".red(), self.content.replace("\n", &"\n> ".red().to_string()), "\n>\n".red());
    write!(formatter, "{base}")
}}
impl Error for Issue {}

//> ISSUES -> UNKNOWN TOKEN
pub fn UnknownToken(line: u32, column: u32, code: &str) -> Issue {Issue::is("unknownToken", format!(
    "Unknown token at line {},\n\n    {}\n    {}",
    color(line.to_string().as_str(), Style::Place),
    color(code, Style::Code),
    " ".repeat((column - 1) as usize) + &color("^", Style::Place)
))}

//> ISSUES -> UNKNOWN TARGET
pub fn UnknownTarget(unknown: &str) -> Issue {Issue::is("unknownTarget", format!(
    "Unknown target {}.\n\nAvailable targets:\n{}",
    color(unknown, Style::Target),
    TARGETS.map(|target| "- ".to_string() + &color(target.0, Style::Target) + ", " + target.1).join("\n")
))}

//> ISSUES -> NO FILE PROVIDED
pub fn MissingFile() -> Issue {Issue::is("noFileProvided" ,format!(
    "Missing input file. Files available in {}:\n{}",
    color(currentDir().unwrap().to_str().unwrap(), Style::Place),
    readDir(currentDir().unwrap()).unwrap().into_iter().filter_map(|entry| if let Ok(thing) = entry && thing.path().is_file() {Some(thing.path())} else {None}).filter_map(|file| if let Some(extension) = file.extension() && extension.to_str().unwrap().starts_with("ms") {Some("- ".to_string() + &color(file.strip_prefix(currentDir().unwrap()).unwrap().to_str().unwrap(), Style::Place))} else {None}).collect::<Vec<String>>().join("\n")
))}

//> ISSUES -> INPUT TOO LONG
pub fn InputTooLong() -> Issue {Issue::is("inputTooLong", format!(
    "Input size exceeds limit of {MAXLEN}."
))}

//> ISSUES -> UNKNOWN FLAG
pub fn UnknownFlag(flag: &Flag) -> Issue {Issue::is("unknownFlag", format!(
    "Unknown flag provided: {}\n\nValid flags and aliases:\n{}",
    color(&("--".to_string() + &flag.value) as &str, Style::Flag),
    FLAGLIASES.map(|tuple| "- ".to_string() + &color(&('-'.to_string() + &tuple.0.to_string()) as &str, Style::Alias) + ", " + &color(&("--".to_string() + tuple.1) as &str, Style::Flag) + ": " + tuple.2).join("\n")
))}

//> ISSUES -> UNKNOWN ALIAS CHARACTER
pub fn UnknownAliasCharacter(alias: &Alias, index: usize) -> Issue {Issue::is("unknownAliasCharacter", format!(
    "Unknown alias character provided: {}\n{}\n\nValid flags and aliases:\n{}",
    color(&('-'.to_string() + &alias.letters.iter().collect::<String>()) as &str, Style::Alias),
    " ".repeat(35 + index) + &color("^", Style::Place),
    FLAGLIASES.map(|tuple| "- ".to_string() + &color(&('-'.to_string() + &tuple.0.to_string()) as &str, Style::Alias) + ", " + &color(&("--".to_string() + tuple.1) as &str, Style::Flag) + ": " + tuple.2).join("\n")
))}

//> ISSUES -> HELP
pub fn GetHelp() -> Issue {Issue::is("getHelp", format!(
    "Available targets:\n{}\n\nAvailable flags and aliases:\n{}",
    TARGETS.map(|target| "- ".to_string() + &color(target.0, Style::Target) + ": " + target.1).join("\n"),
    FLAGLIASES.map(|tuple| "- ".to_string() + &color(&('-'.to_string() + &tuple.0.to_string()) as &str, Style::Alias) + ", " + &color(&("--".to_string() + tuple.1) as &str, Style::Flag) + ": " + tuple.2).join("\n")
))}

//> ISSUES -> SYNTAX ERROR
pub fn SyntaxError() -> Issue {Issue::is("syntaxError", format!(
    "Syntax error"
))}