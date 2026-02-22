//^
//^ HEAD
//^

//> HEAD -> MODULES
pub mod noise;

//> HEAD -> PRELUDE
use super::prelude::{
    getArguments
};

//> HEAD -> LOCAL
use self::{
    super::{
        entry::{
            Argument,
            Flag,
            File,
            Target,
            Alias,
        },
        issues::Issue
    },
    noise::Noise
};


//^ 
//^ SETTINGS
//^ 

//> SETTINGS -> SETTINGS
pub struct Settings {
    pub(super) files: Vec<File> = Vec::new(),
    pub targets: Vec<Target> = Vec::new(),
    pub noise: Noise = Noise::Normal
} impl Settings {
    pub fn cli() -> Result<Settings, Issue> {return Settings::set(getArguments().skip(1).map(|argument| match &argument {
        file if file.split(".").last().unwrap() == "msm" => Ok(Argument::File(File(argument.into()))),
        flag if flag.starts_with("--") => Ok(Argument::Flag(Flag::try_from(&argument as &str).ok().ok_or(Issue::UnknownFlag(argument))?)),
        alias if alias.starts_with("-") => Ok(Argument::Alias(Alias(argument.chars().skip(1).collect()))),
        process if let Ok(target) = argument.parse::<Target>() => Ok(Argument::Target(target)),
        other => Err(Issue::UnknownArgument(argument))
    }).collect::<Result<Vec<Argument>, Issue>>()?)}
    pub fn set(arguments: Vec<Argument>) -> Result<Settings, Issue> {
        let mut settings = Settings {..};
        for argument in arguments {settings.apply(argument)?}
        return Ok(settings);
    }
    fn apply(&mut self, argument: Argument) -> Result<(), Issue> {return Ok(match argument {
        Argument::Alias(alias) => for flag in alias.expand()? {self.apply(Argument::Flag(flag))?},
        Argument::File(file) => self.files.push(file),
        Argument::Flag(flag) => match flag {
            Flag::Quiet => self.noise.change(false),
            Flag::Verbose => self.noise.change(true)
        },
        Argument::Target(target) => self.targets.push(target)
    })}
}