pub struct Input {
    callback: Box<dyn FnMut(Option<String>)>,
}

impl Input {
    pub fn new(callback: impl FnMut(Option<String>) + 'static) -> Self {
        Self {
            callback: Box::new(callback),
        }
    }
}
