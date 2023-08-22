use std::{cell::RefCell, rc::Rc};

use super::{Input, Selection};

#[derive(Clone)]
pub enum Modes {
    Explore,
    Move,
    Input(Rc<RefCell<Input>>),
    Select(Rc<RefCell<Selection>>),
    Quit,
}

impl ToString for Modes {
    fn to_string(&self) -> String {
        match self {
            Modes::Explore => "EXPLORE".to_string(),
            Modes::Move => "MOVE".to_string(),
            Modes::Input(_) => "INPUT".to_string(),
            Modes::Select(_) => "SELECT".to_string(),
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

impl Mode {
    pub fn get(&self) -> &Modes {
        &self.mode
    }

    pub fn switch(&mut self, mode: Modes) {
        self.mode = mode;
    }
}
