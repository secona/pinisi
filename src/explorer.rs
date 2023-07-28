use std::{fs, path::PathBuf};

pub struct Explorer {
    path: PathBuf,
}

impl Default for Explorer {
    fn default() -> Self {
        Self {
            path: PathBuf::from("."),
        }
    }
}

impl Explorer {
    pub fn print_items(&self) {
        let items = fs::read_dir(&self.path).unwrap();
        for item in items {
            let item = item.unwrap();
            let is_dir = item.metadata().unwrap().is_dir();
            println!(
                "{} {}",
                if is_dir { "DIR" } else { "   " },
                item.path().display()
            );
        }
    }
}
