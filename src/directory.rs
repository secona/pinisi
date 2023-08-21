use crate::item::Item;
use std::{
    env, fs,
    path::{Path, PathBuf},
};

pub struct Directory {
    pub path: PathBuf,
    pub count: usize,
    pub items: Vec<Item>,
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
        let entries = fs::read_dir(&self.path).unwrap().collect::<Vec<_>>();
        self.count = entries.len();
        self.items = Vec::new();

        for entry in entries {
            self.items.push(Item::from(entry.unwrap()));
        }

        self.items.sort_by_key(|item| !item.meta.is_dir());
    }

    pub fn cd(&mut self, path: &Path) {
        self.path = PathBuf::from(path);
        self.refresh();
    }

    pub fn item_at(&self, index: usize) -> Option<&Item> {
        self.items.get(index)
    }

    pub fn delete_item(&mut self, index: &usize) {
        let item = self.item_at(*index).unwrap();
        if !item.meta.is_dir() {
            fs::remove_file(item.entry.path()).unwrap();
        }
        self.refresh();
    }

    pub fn rename_item(&mut self, index: &usize, to: String) {
        let item = self.item_at(*index).unwrap();
        let old_name = item.entry.path();

        let mut new_name = PathBuf::new();
        new_name.push(old_name.parent().unwrap().to_str().unwrap());
        new_name.push(to);

        fs::rename(old_name.clone(), new_name).unwrap();
        self.refresh();
    }
}
