//^
//^ HEAD
//^

//> HEAD -> FLAGS
#![allow(unused_variables)]
#![allow(nonstandard_style)]
#![feature(try_trait_v2)]
#![feature(default_field_values)]

//> HEAD -> MODULES
mod prelude;
mod issues;
mod entry;
mod tokenizer {
    pub mod tokenizer;
}
mod parser {
    pub mod grammar;
    pub mod parser;
}
mod solver {
    pub mod nonterminal;
    pub mod solver;
}
mod syntax {
    pub mod level1;
    pub mod level2;
    pub mod level3;
    pub mod level4;
    pub mod level5;
    pub mod start;
}
mod backends {
    pub mod latex;
    pub mod traits;
}

//> HEAD -> PRELUDE
use crate::prelude::{
    ARCH, OS, Time, rustcv, getArguments
};

//> HEAD -> LOCAL
use self::{
    syntax::start::Start,
    tokenizer::tokenizer::{ShallowToken, Tokenizer, MAXLEN},
    parser::parser::Parser,
    solver::solver::Solver,
    backends::traits::Backends,
    issues::Issue,
    entry::{Argument, File, Flag, Target, Alias}
};

//> HEAD -> VERSION
pub static VERSION: usize = 7;


//^
//^ PIPELINE
//^

//> PIPELINE -> TRANSFORMERS
pub struct Transformers {
    tokenizer: Tokenizer,
    parser: Parser,
    solver: Solver,
    settings: Settings,
    time: Time
} impl Transformers {pub fn new(arguments: &[Argument]) -> Result<Self, Issue> {return Ok(Transformers {
    tokenizer: Tokenizer::new(),
    parser: Parser::new(),
    solver: Solver::new(),
    settings: Settings::set(arguments)?,
    time: Time::now()
})}}

//> PIPELINE -> HELP
pub fn help(transformers: &mut Transformers) -> Result<(), Issue> {return Err(Issue::GetHelp)}

//> PIPELINE -> VERSION
pub fn version(transformers: &mut Transformers) -> Result<usize, Issue> {return Ok(VERSION)}

//> PIPELINE -> TOKENS
pub fn tokens(transformers: &mut Transformers) -> Result<Vec<ShallowToken>, Issue> {
    let content = transformers.settings.file.as_ref().ok_or(Issue::MissingFile)?.read()?;
    let tokens = transformers.tokenizer.run(&content, &transformers.settings)?;
    return Ok(tokens.into_iter().map(|token| token.fixate()).collect());
}

//> PIPELINE -> LENGTH
pub fn length(transformers: &mut Transformers) -> Result<usize, Issue> {
    let content = transformers.settings.file.as_ref().ok_or(Issue::MissingFile)?.read()?;
    let tokens = transformers.tokenizer.run(&content, &transformers.settings)?;
    return Ok(tokens.len());
}

//> PIPELINE -> CHECK
pub fn check(transformers: &mut Transformers) -> Result<(), Issue> {
    let content = transformers.settings.file.as_ref().ok_or(Issue::MissingFile)?.read()?;
    let tokens = transformers.tokenizer.run(&content, &transformers.settings)?;
    let pool = transformers.parser.run(&tokens, &transformers.settings);
    let start = transformers.solver.run(&pool)?;
    return Ok(());
}

//> PIPELINE -> AST
pub fn ast(transformers: &mut Transformers) -> Result<Start, Issue> {
    let content = transformers.settings.file.as_ref().ok_or(Issue::MissingFile)?.read()?;
    let tokens = transformers.tokenizer.run(&content, &transformers.settings)?;
    let pool = transformers.parser.run(&tokens, &transformers.settings);
    let start = transformers.solver.run(&pool)?;
    return Ok(start);
}

//> PIPELINE -> LATEX
pub fn latex(transformers: &mut Transformers) -> Result<String, Issue> {
    let content = transformers.settings.file.as_ref().ok_or(Issue::MissingFile)?.read()?;
    let tokens = transformers.tokenizer.run(&content, &transformers.settings)?;
    let pool = transformers.parser.run(&tokens, &transformers.settings);
    let start = transformers.solver.run(&pool)?;
    return Ok(start.latex());
}


//^
//^ TARGETS
//^

//> TARGETS -> NOISE
enum Noise {
    Debug,
    Verbose,
    Normal,
    Quiet,
    Zero
} impl Noise {
    fn change(&mut self, shift: bool) -> () {*self = match self {
        Noise::Debug => if shift {Noise::Debug} else {Noise::Verbose},
        Noise::Normal => if shift {Noise::Verbose} else {Noise::Quiet},
        Noise::Quiet => if shift {Noise::Normal} else {Noise::Zero},
        Noise::Verbose => if shift {Noise::Debug} else {Noise::Normal},
        Noise::Zero => if shift {Noise::Quiet} else {Noise::Zero}
    }}
    fn verbose(&self) -> bool {match self {
        Noise::Debug | Noise::Verbose => true,
        other => false
    }}
    fn quiet(&self) -> bool {match self {
        Noise::Quiet | Noise::Zero => true,
        other => false
    }}
    fn debug(&self) -> bool {if let Noise::Debug = self {true} else {false}}
    fn zero(&self) -> bool {if let Noise::Zero = self {true} else {false}}
}

//> TARGETS -> SETTINGS
pub struct Settings {
    file: Option<File> = None,
    target: Option<Target> = None,
    noise: Noise = Noise::Normal
} impl Settings {
    pub fn set(arguments: &[Argument]) -> Result<Settings, Issue> {
        let mut settings = Settings {..};
        for argument in arguments {settings.apply(argument)?}
        return Ok(settings);
    }
    pub fn apply(&mut self, argument: &Argument) -> Result<(), Issue> {return Ok(match argument {
        Argument::Alias(alias) => for (index, letter) in alias.letters.iter().enumerate() {if let Some((character, aliasing)) = FLAGLIASES.iter().find_map(|(key, second, third)| if key == letter {Some((key, second))} else {None}) {
            self.apply(&Argument::Flag(Flag {value: aliasing.to_string()}))?;
        } else {return Err(Issue::UnknownAliasCharacter {
            alias: alias.clone(), 
            at: index
        })}}
        Argument::File(file) => if self.file.is_none() {self.file = Some(file.clone())},
        Argument::Flag(flag) => match &flag.value.to_lowercase() as &str {
            name if name == FLAGLIASES[0].1 => return Err(Issue::GetHelp),
            name if name == FLAGLIASES[1].1 => self.noise.change(false),
            name if name == FLAGLIASES[2].1 => self.noise.change(true),
            other => return Err(Issue::UnknownFlag(flag.clone()))
        },
        Argument::Target(target) => if self.target.is_none() {self.target = Some(target.clone())}
    })}
}

//> TARGETS -> WRAPPER
pub fn main() -> Result<(), Issue> {
    let mut transformers = Transformers::new(&getArguments().skip(1).map(|argument| match &argument {
        file if file.split(".").last().unwrap() == "msm" => Argument::File(File {
            name: argument.into()
        }),
        flag if flag.starts_with("--") => Argument::Flag(Flag {
            value: argument.chars().skip(2).collect()
        }),
        alias if alias.starts_with("-") => Argument::Alias(Alias {
            letters: argument.chars().skip(1).collect()
        }),
        target => Argument::Target(Target {
            name: argument
        }),
    }).collect::<Vec<Argument>>())?;
    match &transformers.settings.target.clone().unwrap_or(Target {name: String::from("help")}).name as &str {
        target if target == TARGETS[0].0 => help(&mut transformers)?,
        target if target == TARGETS[1].0 => println!("Running Mathsys v{} on {}/{}/{}", version(&mut transformers)?, OS, ARCH, rustcv()),
        target if target == TARGETS[2].0 => println!("{:#?}", tokens(&mut transformers)?),
        target if target == TARGETS[3].0 => {let len = length(&mut transformers)?; println!("Token length: {len} / {MAXLEN} ({}%)", len as f32 / MAXLEN as f32 * 100.0)},
        target if target == TARGETS[4].0 => println!("{}", {check(&mut transformers)?; "Valid"}),
        target if target == TARGETS[5].0 => println!("{:#?}", ast(&mut transformers)?),
        target if target == TARGETS[6].0 => println!("{}", latex(&mut transformers)?),
        other => return Err(Issue::UnknownTarget(other.to_string()))
    };
    if transformers.settings.noise.verbose() {println!("Execution time: {:?}", transformers.time.elapsed())}
    return Ok(());
}

//> TARGETS -> LIST
pub static TARGETS: [(&'static str, &'static str); 7] = [
    ("help", "show this informative menu"),
    ("version", "check current Mathsys version"),
    ("tokens", "tokenize the input file"),
    ("length", "show length of token stream"),
    ("check", "validate the semantics"),
    ("ast", "build the abstract syntax tree"),
    ("latex", "show the latex program equivalency")
];

//> TARGETS -> FLAGS AND ALIASES
pub static FLAGLIASES: [(char, &'static str, &'static str); 3] = [
    ('h', "help", "show help menu"),
    ('q', "quiet", "silence the output"),
    ('v', "verbose", "increase program verbosity")
];