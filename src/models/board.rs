use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
// Board struct to hold the cells and size of the board
pub struct Board {
    pub cells: Vec<Vec<Option<u32>>>,
    pub size: usize
}

// Board implementation to get and set values
impl Board {
    pub fn new(size: usize) -> Board {
        Self {
            cells: vec![vec![None; size]; size],
            size: size
        }
    }

    pub fn get_value(&self, row: usize, col: usize) -> Option<u32> {
        self.cells[row][col]
    }

    pub fn set_value(&mut self, row: usize, col: usize, value: Option<u32>) {
        self.cells[row][col] = value;
    }

    pub fn get_size(&self) -> usize {
        self.size
    }

    // Get the size of the subgrid
    pub fn get_subgrid(&self) -> (usize, usize) {
        match self.get_size() {
            6 => (2, 3),
            8 => (2, 4),
            10 => (2, 5),
            12 => (3, 4),
            _ => {
                let root = (self.get_size() as f64).sqrt() as usize;
                (root, root)
            }
        }
    }
}