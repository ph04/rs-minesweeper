use crate::board::Board;

const BOARD_WIDTH: usize = 10;
const BOARD_HEIGHT: usize = 10;

pub struct Minesweeper {
    board: Board,
}

impl Minesweeper {
    pub fn new() -> Self {
        Self {
            board: Board::new(BOARD_WIDTH, BOARD_HEIGHT),
        }
    }
}