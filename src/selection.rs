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

    pub fn update(&mut self, cursor_position: &usize) {
        if self.start < *cursor_position {
            self.set((self.start..=*cursor_position).collect());
        } else {
            self.set((*cursor_position..=self.start).collect());
        }
    }
}
