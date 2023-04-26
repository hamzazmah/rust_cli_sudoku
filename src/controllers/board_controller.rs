pub mod board_controller {
    use crate::models::Board;
    use rand::{seq::SliceRandom, thread_rng};

    pub trait IBoard {
        fn is_move_valid(&self, row: usize, col: usize, value: u8) -> bool;
        fn is_board_valid(&self, skip_empty: bool) -> bool;
        fn generate_board(&mut self) -> bool;
    }

    impl IBoard for Board {    
        fn is_move_valid(&self, row: usize, col: usize, value: u8) -> bool {
            if self.get_value(row, col).is_some() {
                return false;
            }
    
            for i in 0..9 {
                if self.get_value(row, i) == Some(value) {
                    return false;
                }
                if self.get_value(i, col) == Some(value) {
                    return false;
                }
            }
    
            let box_row = (row / 3) * 3;
            let box_col = (col / 3) * 3;
    
            for i in box_row..(box_row + 3) {
                for j in box_col..(box_col + 3) {
                    if self.get_value(i, j) == Some(value) {
                        return false;
                    }
                }
            }
    
            true
        }
        
        fn generate_board(&mut self) -> bool {
            let mut rng = thread_rng();

            // Find an empty cell
            let mut row = 0;
            let mut col = 0;

            let mut found_empty_cell = false;
            for i in 0..9 {
                for j in 0..9 {
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
        
            let mut values: Vec<u8> = (1..=9).collect();
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
            for row in 0..9 {
                for col in 0..9 {
                    let cell = self.get_value(row, col);
                    if skip_empty && cell.is_none() {
                        continue;
                    } else if !skip_empty && cell.is_none() {
                        return false;
                    }

                    for c in 0..9 {
                        if c != col && self.get_value(row,c) == cell {
                            return false;
                        }
                    }

                    for r in 0..9 {
                        if r != row && self.get_value(r,col) == cell {
                            return false;
                        }
                    }

                    let block_row = row / 3;
                    let block_col = col / 3;
                    for r in block_row * 3..(block_row + 1) * 3 {
                        for c in block_col * 3..(block_col + 1) * 3 {
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