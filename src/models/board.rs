use serde::{Serialize, Deserialize};

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Board {
    pub cells: [[Option<u8>; 9]; 9],
}

impl Board {
    pub fn new() -> Board {
        Self {
            cells: [[None; 9]; 9],
        }
    }

    pub fn copy (board: &Board) -> Self {
        Self {
            cells: board.cells,
        }
    }

    pub fn get_value(&self, row: usize, col: usize) -> Option<u8> {
        self.cells[row][col]
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: Option<u8>) {
        self.cells[row][col] = value;
    }
}