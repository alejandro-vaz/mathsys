//^
//^ NOISE
//^

//> NOISE -> DEFINITION
pub enum Noise {
    Debug,
    Verbose,
    Normal,
    Quiet,
    Zero
} impl Noise {
    pub(super) fn change(&mut self, shift: bool) -> () {*self = match self {
        Noise::Debug => if shift {Noise::Debug} else {Noise::Verbose},
        Noise::Normal => if shift {Noise::Verbose} else {Noise::Quiet},
        Noise::Quiet => if shift {Noise::Normal} else {Noise::Zero},
        Noise::Verbose => if shift {Noise::Debug} else {Noise::Normal},
        Noise::Zero => if shift {Noise::Quiet} else {Noise::Zero}
    }}
    pub fn verbose(&self) -> bool {match self {
        Noise::Debug | Noise::Verbose => true,
        other => false
    }}
    pub fn quiet(&self) -> bool {match self {
        Noise::Quiet | Noise::Zero => true,
        other => false
    }}
    pub fn debug(&self) -> bool {if let Noise::Debug = self {true} else {false}}
    pub fn zero(&self) -> bool {if let Noise::Zero = self {true} else {false}}
}