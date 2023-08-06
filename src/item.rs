use std::fs::{DirEntry, Metadata};

pub struct Item {
    pub entry: DirEntry,
    pub meta: Metadata,
}

impl From<DirEntry> for Item {
    fn from(entry: DirEntry) -> Self {
        Self {
            meta: entry.metadata().unwrap(),
            entry,
        }
    }
}
