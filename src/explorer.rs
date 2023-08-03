use crate::directory::Directory;

pub struct Explorer {
    directory: Directory,
}

impl Default for Explorer {
    fn default() -> Self {
        Self {
            directory: Directory::new(),
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
