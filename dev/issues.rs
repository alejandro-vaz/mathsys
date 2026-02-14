//^
//^ HEAD
//^

//> HEAD -> PRELUDE
use crate::prelude::{
    exit, Error, Display, Formatter, Rst, Colored, Flag, currentDir, readDir, Alias, AsRefStr, Debug
};

//> HEAD -> LOCAL
use super::{
    TARGETS,
    FLAGLIASES,
    tokenizer::tokenizer::MAXLEN
};


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
    Alias,
    Issue
}

//> DEFAULTS -> COLOR
fn color(string: impl Colored, style: Style) -> String {return (match style {
    Style::Code => string.truecolor(0xAA, 0xAA, 0xAA),
    Style::Place => string.cyan(),
    Style::Object => string.purple().bold(),
    Style::Target => string.green(),
    Style::Flag => string.blue(),
    Style::Alias => string.yellow(),
    Style::Issue => string.red()
}).to_string()}


//^
//^ ISSUES
//^

//> ISSUES -> NEWIS
#[derive(AsRefStr)]
pub enum Issue {
    UnknownToken {
        line: u32,
        column: u32,
        code: String
    },
    UnknownTarget(String),
    MissingFile,
    InputTooLong,
    UnknownFlag(Flag),
    UnknownAliasCharacter {
        alias: Alias,
        at: usize
    },
    GetHelp,
    SyntaxError
} impl Issue {
    pub fn consume(self) -> ! {println!("{self}"); exit(&self as *const Issue as i32)}
    fn content(&self) -> String {match self {
        Issue::UnknownToken {line, column, code} => format!(
            "Unknown token at line {},\n\n    {}\n    {}",
            color(&line.to_string() as &str, Style::Place),
            color(code as &str, Style::Code),
            " ".repeat(*column as usize - 1) + &color("^", Style::Place)
        ),
        Issue::UnknownTarget(target) => format!(
            "Unknown target {}.\n\nAvailable targets:\n{}",
            color(target as &str, Style::Target),
            TARGETS.map(|target| "- ".to_string() + &color(target.0, Style::Target) + ", " + target.1).join("\n")
        ),
        Issue::MissingFile => format!(
            "Missing input file. Files available in {}:\n{}",
            color(currentDir().unwrap().to_str().unwrap(), Style::Place),
            readDir(currentDir().unwrap()).unwrap().into_iter().filter_map(|entry| if let Ok(thing) = entry && thing.path().is_file() {Some(thing.path())} else {None}).filter_map(|file| if let Some(extension) = file.extension() && extension.to_str().unwrap() == "msm" {Some("- ".to_string() + &color(file.strip_prefix(currentDir().unwrap()).unwrap().to_str().unwrap(), Style::Place))} else {None}).collect::<Vec<String>>().join("\n")
        ),
        Issue::InputTooLong => format!(
            "Input size exceeds limit of {MAXLEN}."
        ),
        Issue::UnknownFlag(flag) => format!(
            "Unknown flag provided: {}\n\nValid flags and aliases:\n{}",
            color(&("--".to_string() + &flag.value) as &str, Style::Flag),
            FLAGLIASES.map(|tuple| "- ".to_string() + &color(&('-'.to_string() + &tuple.0.to_string()) as &str, Style::Alias) + ", " + &color(&("--".to_string() + tuple.1) as &str, Style::Flag) + ": " + tuple.2).join("\n")
        ),
        Issue::UnknownAliasCharacter {alias, at} => format!(
            "Unknown alias character provided: {}\n{}\n\nValid flags and aliases:\n{}",
            color(&('-'.to_string() + &alias.letters.iter().collect::<String>()) as &str, Style::Alias),
            " ".repeat(35 + at) + &color("^", Style::Place),
            FLAGLIASES.map(|tuple| "- ".to_string() + &color(&('-'.to_string() + &tuple.0.to_string()) as &str, Style::Alias) + ", " + &color(&("--".to_string() + tuple.1) as &str, Style::Flag) + ": " + tuple.2).join("\n")
        ),
        Issue::GetHelp => format!(
            "Available targets:\n{}\n\nAvailable flags and aliases:\n{}",
            TARGETS.map(|target| "- ".to_string() + &color(target.0, Style::Target) + ": " + target.1).join("\n"),
            FLAGLIASES.map(|tuple| "- ".to_string() + &color(&('-'.to_string() + &tuple.0.to_string()) as &str, Style::Alias) + ", " + &color(&("--".to_string() + tuple.1) as &str, Style::Flag) + ": " + tuple.2).join("\n")
        ),
        Issue::SyntaxError => format!(
            "Syntax error."
        )
    }}
} impl Display for Issue {fn fmt(&self, formatter: &mut Formatter<'_>) -> Rst {write!(formatter, "{}", format!(
    "{} {} {}{}{}{}", 
    color("Raised", Style::Issue), 
    color(self.as_ref(), Style::Object),
    color("issue:", Style::Issue), 
    color("\n>\n> ", Style::Issue), 
    self.content().replace("\n", &color("\n> ", Style::Issue)), 
    color("\n>\n", Style::Issue)
))}} impl Debug for Issue {fn fmt(&self, formatter: &mut Formatter<'_>) -> Rst {Display::fmt(&self, formatter)}} impl Error for Issue {}