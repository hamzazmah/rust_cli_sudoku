pub mod board_controller {
    use crate::models::Board;
    use rand::{seq::SliceRandom, thread_rng};

    pub trait IBoard {
        fn is_move_valid(&self, row: usize, col: usize, value: u32) -> bool;
        fn is_board_valid(&self, skip_empty: bool) -> bool;
        fn generate_board(&mut self) -> bool;
    }

    impl IBoard for Board {    
        fn is_move_valid(&self, row: usize, col: usize, value: u32) -> bool {
            if self.get_value(row, col).is_some() {
                return false;
            }
        
            let size = self.get_size();

            if (value as usize) > size {
                return false;
            }

            let (subgrid_size_row, subgrid_size_col) = self.get_subgrid();
        
            // Check row
            for i in 0..size {
                if self.get_value(row, i) == Some(value) {
                    return false;
                }
            }
            // Check column
            for i in 0..size {
                if self.get_value(i, col) == Some(value) {
                    return false;
                }
            }
            // Check sub-grid
            let row_start = (row / subgrid_size_row) * subgrid_size_row;
            let col_start = (col / subgrid_size_col) * subgrid_size_col;
            for i in row_start..(row_start + subgrid_size_row) {
                for j in col_start..(col_start + subgrid_size_col) {
                    if self.get_value(i, j) == Some(value) {
                        return false;
                    }
                }
            }
            true
        }
        
        fn generate_board(&mut self) -> bool {
            let mut rng = thread_rng();

            let size = self.get_size();
            
            // Find an empty cell
            let mut row = 0;
            let mut col = 0;

            let mut found_empty_cell = false;
            for i in 0..size {
                for j in 0..size {
                    if self.get_value(i,j).is_none() {
                        row = i;
                        col = j;
                        found_empty_cell = true;
                        break;
                    }
                }
                if found_empty_cell {
                    break;
                }
            }
        
            if !found_empty_cell {
                return true; // Board is filled, solution found
            }
        
            let mut values: Vec<u32> = (1..=size as u32).collect();
            values.shuffle(&mut rng);
        
            for value in values {
                // Check if the value is valid in the current cell
                if self.is_move_valid(row, col, value) {
                    self.set_value(row, col, Some(value));
        
                    // Recursively generate the rest of the board
                    if self.generate_board() {
                        return true;
                    }
        
                    self.set_value(row, col, None); // Undo the current cell
                }
            }
        
            false // No valid value found for the current cell
        }

        fn is_board_valid(&self, skip_empty: bool) -> bool {
            let board_size = self.get_size();
            let (subgrid_size_row, subgrid_size_col) = self.get_subgrid();
        
            for row in 0..board_size {
                for col in 0..board_size {
                    let cell = self.get_value(row, col);
                    if skip_empty && cell.is_none() {
                        continue;
                    } else if !skip_empty && cell.is_none() {
                        return false;
                    }
        
                    for c in 0..board_size {
                        if c != col && self.get_value(row,c) == cell {
                            return false;
                        }
                    }
        
                    for r in 0..board_size {
                        if r != row && self.get_value(r,col) == cell {
                            return false;
                        }
                    }
        
                    let block_row = row / subgrid_size_row;
                    let block_col = col / subgrid_size_col;
                    for r in block_row * subgrid_size_row..(block_row + 1) * subgrid_size_row {
                        for c in block_col * subgrid_size_col..(block_col + 1) * subgrid_size_col {
                            if (r != row || c != col) && self.get_value(r,c) == cell {
                                return false;
                            }
                        }
                    }
                }
            }
            true
        }
    }
}