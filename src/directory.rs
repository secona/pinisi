use std::{
    env,
    fs::{self, DirEntry},
    path::{Path, PathBuf},
};

pub struct Directory {
    pub path: PathBuf,
    pub count: usize,
    pub items: Vec<DirEntry>,
}

impl Directory {
    pub fn new() -> Self {
        let path = env::current_dir().unwrap();
        Self {
            path,
            count: 0,
            items: Vec::new(),
        }
    }

    pub fn refresh(&mut self) {
        let items = fs::read_dir(&self.path).unwrap().collect::<Vec<_>>();
        self.count = items.len();
        self.items = Vec::new();

        for item in items {
            let item = item.unwrap();
            self.items.push(item);
        }

        self.items
            .sort_by_key(|item| !item.metadata().unwrap().is_dir());
    }

    pub fn cd(&mut self, path: &Path) {
        self.path = PathBuf::from(path);
        self.refresh();
    }

    pub fn item_at(&self, index: usize) -> Option<&DirEntry> {
        self.items.get(index)
    }
}
