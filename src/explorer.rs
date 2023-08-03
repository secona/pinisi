use crate::{cursor::Cursor, directory::Directory};

pub struct Explorer {
    directory: Directory,
    cursor: Cursor,
}

impl Default for Explorer {
    fn default() -> Self {
        let directory = Directory::new();
        Self {
            cursor: Cursor::from(&directory),
            directory,
        }
    }
}

impl Explorer {
    pub fn print_items(&mut self) {
        self.directory.refresh();
        for item in &self.directory.items {
            let meta = item.metadata().unwrap();
            println!(
                "{} {}",
                if meta.is_dir() { "DIR" } else { "   " },
                item.path().display()
            );
        }
    }
}
