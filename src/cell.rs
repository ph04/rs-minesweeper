#[derive(Debug)]
pub enum Cell {
    Safe,
    Mine,
}

impl Cell {
    pub fn new_random() -> Self {
        Self::Safe // TODO: change to random
    }
}