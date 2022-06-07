use crate::cell::Cell;

#[derive(Debug)]
pub struct Board {
    width: usize,
    height: usize,
    raw_board: Vec<Vec<Cell>>,
}

impl Board {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            raw_board: (0..height).map(|_|
                (0..width).map(|_| Cell::new_random()).collect()
            ).collect()
        }
    }
}