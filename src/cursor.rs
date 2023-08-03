use crate::directory::Directory;

pub struct Cursor {
    pub position: usize,
    pub max: i32,
}

impl From<&Directory> for Cursor {
    fn from(directory: &Directory) -> Self {
        Self {
            position: 0,
            max: directory.count - 1,
        }
    }
}
