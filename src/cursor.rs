use crate::directory::Directory;

pub struct Cursor {
    pub position: usize,
    pub max: usize,
}

impl From<&Directory> for Cursor {
    fn from(directory: &Directory) -> Self {
        Self {
            position: 0,
            max: directory.count - 1,
        }
    }
}

impl Cursor {
    pub fn mut_move_relative(&mut self, value: isize) {
        let value = self
            .position
            .saturating_add_signed(value)
            .clamp(0, self.max);

        self.position = value;
    }
}
