#![allow(dead_code)]

enum Modes {
    Explore,
    Move,
    Input,
    Select,
    Quit,
}

impl ToString for Modes {
    fn to_string(&self) -> String {
        match self {
            Modes::Explore => "EXPLORE".to_string(),
            Modes::Move => "MOVE".to_string(),
            Modes::Input => "INPUT".to_string(),
            Modes::Select => "SELECT".to_string(),
            Modes::Quit => "QUIT".to_string(),
        }
    }
}

pub struct Mode {
    mode: Modes,
}

impl Default for Mode {
    fn default() -> Self {
        Self {
            mode: Modes::Explore,
        }
    }
}

impl std::fmt::Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "-- {} --", self.mode.to_string())
    }
}
