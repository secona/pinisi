use std::{
    fs::{self, DirEntry},
    path::PathBuf,
};

pub struct Directory {
    pub path: PathBuf,
    pub count: usize,
    pub items: Vec<DirEntry>,
}

impl Directory {
    pub fn new() -> Self {
        Self {
            path: PathBuf::from("."),
            count: 0,
            items: Vec::new(),
        }
    }

    pub fn refresh(&mut self) {
        let items = fs::read_dir(&self.path).unwrap().collect::<Vec<_>>();
        self.count += &items.len();
        for item in items {
            let item = item.unwrap();
            self.items.push(item);
        }
    }
}
