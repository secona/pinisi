#![allow(dead_code)]

use std::{cell::RefCell, rc::Rc};

#[derive(Clone)]
pub struct Selection {
    pub start: usize,
    pub selected: Vec<usize>,
}

impl Selection {
    pub fn new(start: usize) -> Self {
        Self {
            start,
            selected: vec![start],
        }
    }

    pub fn set(&mut self, selected: Vec<usize>) {
        self.selected = selected;
    }
}

#[derive(Clone)]
pub enum Modes {
    Explore,
    Move,
    Input,
    Select(Rc<RefCell<Selection>>),
    Quit,
}

impl ToString for Modes {
    fn to_string(&self) -> String {
        match self {
            Modes::Explore => "EXPLORE".to_string(),
            Modes::Move => "MOVE".to_string(),
            Modes::Input => "INPUT".to_string(),
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
