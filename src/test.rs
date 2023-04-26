// pub fn set_difficulty(&mut self, difficulty: Difficulty) {
//     let num_solved_cells = match difficulty {
//         Difficulty::Easy => 35,
//         Difficulty::Medium => 45,
//         Difficulty::Hard => 55,
//     };

//     let mut rng = rand::thread_rng();

//     // create a random solved board
//     let mut solved_board = Board::new();
//     let mut stack = Vec::new();
//     let mut row = 0;
//     let mut col = 0;

//     loop {
//         if row == 9 {
//             break;
//         }

//         if solved_board.cells[row][col].fixed {
//             if col == 8 {
//                 row += 1;
//                 col = 0;
//             } else {
//                 col += 1;
//             }
//             continue;
//         }

//         let mut valid_values = Vec::new();

//         for value in 1..=9 {
//             if solved_board.is_valid_move(row, col, value) {
//                 valid_values.push(value);
//             }
//         }

//         if valid_values.is_empty() {
//             let (prev_row, prev_col) = stack.pop().unwrap();
//             row = prev_row;
//             col = prev_col;
//             solved_board.set_value(row, col, None);
//             if col == 0 {
//                 if let Some(sub_row) = row.checked_sub(1) {
//                     row = sub_row;
//                 } else {
//                     print!("");
//                 }
//                 col = 8;
//             } else {
//                 col -= 1;
//             }
//             continue;
//         }

//         let value = valid_values[rng.gen_range(0..valid_values.len())];

//         solved_board.set_value(row, col, Some(value));
//         stack.push((row, col));
//         if col == 8 {
//             row += 1;
//             col = 0;
//         } else {
//             col += 1;
//         }
//     }

//     // copy the solved board to the game board and remove some cells
//     self.board = solved_board;

//     let mut removed_cells = 81 - num_solved_cells;

//     while removed_cells > 0 {
//         let row = rng.gen_range(0..9);
//         let col = rng.gen_range(0..9);

//         if self.board.cells[row][col].fixed {
//             continue;
//         }

//         let value = self.board.cells[row][col].value;

//         self.board.set_value(row, col, None);
//         self.moves.push((row, col, value));
//         removed_cells -= 1;
//     }
// }

// pub fn play(&mut self) {
//     let stdin = io::stdin();
//     let mut stdout = io::stdout();
//     let mut input = String::new();
//     let mut difficulty = None;

//     loop {
//         println!("Welcome to Rust Sudoku, {}!", self.user.name);
//         println!("Wins: {}, Losses: {}", self.user.stats.wins, self.user.stats.losses);
//         println!("Choose a difficulty level:");

//         println!("1. Easy");
//         println!("2. Medium");
//         println!("3. Hard");

//         input.clear();
//         stdin.read_line(&mut input).unwrap();

//         match input.trim() {
//             "1" => {
//                 self.set_difficulty(Difficulty::Easy);
//                 difficulty = Some(Difficulty::Easy);
//                 break;
//             }
//             "2" => {
//                 self.set_difficulty(Difficulty::Medium);
//                 difficulty = Some(Difficulty::Medium);
//                 break;
//             }
//             "3" => {
//                 self.set_difficulty(Difficulty::Hard);
//                 difficulty = Some(Difficulty::Hard);
//                 break;
//             }
//             _ => {
//                 println!("Invalid input, please try again.");
//             }
//         }
//     }

//     loop {
//         println!("{}", self.board);

//         if self.board.is_solved() {
//             println!("Congratulations, you solved the puzzle!");
//             self.user.stats.wins += 1;
//             break;
//         }

//         println!("Enter your move as row column value, e.g. '4 2 7', or 'r' to redo a move:");

//         input.clear();
//         stdin.read_line(&mut input).unwrap();

//         if input.trim() == "r" {
//             if self.redo_move() {
//                 println!("Redoing last move.");
//             } else {
//                 println!("No more moves to redo.");
//             }
//             continue;
//         }

//         let parts: Vec<&str> = input.trim().split_whitespace().collect();

//         if parts.len() != 3 {
//             println!("Invalid input, please try again.");
//             continue;
//         }

//         let row = match parts[0].parse::<usize>() {
//             Ok(num) => num - 1,
//             Err(_) => {
//                 println!("Invalid row, please try again.");
//                 continue;
//             }
//         };

//         let col = match parts[1].parse::<usize>() {
//             Ok(num) => num - 1,
//             Err(_) => {
//                 println!("Invalid column, please try again.");
//                 continue;
//             }
//         };

//         let value = match parts[2].parse::<u8>() {
//             Ok(num) => Some(num),
//             Err(_) => {
//                 println!("Invalid value, please try again.");
//                 continue;
//             }
//         };

//         if !self.make_move(row, col, value) {
//             println!("Invalid move, please try again.");
//         }
//     }
// }

// pub fn make_move(&mut self, row: usize, col: usize, value: Option<u8>) -> bool {
//     if self.board.is_valid_move(row, col, value.unwrap()) {
//         self.board.set_value(row, col, value);
//         self.moves.push((row, col, value));
//         true
//     } else {
//         false
//     }
// }

// pub fn redo_move(&mut self) -> bool {
//     if let Some((row, col, value)) = self.moves.pop() {
//         self.board.set_value(row, col, value);
//         true
//     } else {
//         false
//     }
// }

// pub fn is_valid_move(&self, row: usize, col: usize, value: u8) -> bool {
//     if let Some(_) = self.cells[row][col].value {
//         return false;
//     }

//     for i in 0..9 {
//         if self.cells[row][i].value == Some(value) {
//             return false;
//         }
//         if self.cells[i][col].value == Some(value) {
//             return false;
//         }
//     }

//     let box_row = (row / 3) * 3;
//     let box_col = (col / 3) * 3;

//     for i in box_row..(box_row + 3) {
//         for j in box_col..(box_col + 3) {
//             if self.cells[i][j].value == Some(value) {
//                 return false;
//             }
//         }
//     }

//     true
// }

// pub fn is_solved(&self) -> bool {
//     for row in self.cells.iter() {
//         for cell in row.iter() {
//             if cell.value.is_none() {
//                 return false;
//             }
//         }
//     }

//     for i in 0..9 {
//         for j in 0..9 {
//             let value = self.cells[i][j].value.unwrap();

//             for k in (i + 1)..9 {
//                 if self.cells[k][j].value == Some(value) {
//                     return false;
//                 }
//             }

//             for k in (j + 1)..9 {
//                 if self.cells[i][k].value == Some(value) {
//                     return false;
//                 }
//             }

//             let box_row = (i / 3) * 3;
//             let box_col = (j / 3) * 3;

//             for r in box_row..(box_row + 3) {
//                 for c in box_col..(box_col + 3) {
//                     if (r != i || c != j) && self.cells[r][c].value == Some(value) {
//                         return false;
//                     }
//                 }
//             }
//         }
//     }

//     true
// }


// pub mod board_controller {
//     use crate::models::Board;

//     pub trait IBoard {
//         fn solve(&mut self) -> bool;
//         fn is_solved(&self) -> bool;
//         fn is_valid_move(&self, row: usize, col: usize, value: u8) -> bool;
//         fn is_valid_in_row(&self, row: usize, col: usize, value: u8) -> bool;
//         fn is_valid_in_column(&self, row: usize, col: usize, value: u8) -> bool;
//         fn is_valid_in_block(&self, row: usize, col: usize, value: u8) -> bool;
//         fn is_valid(&self) -> bool;
//     }

//     impl IBoard for Board {    
//         fn is_valid_move(&self, row: usize, col: usize, value: u8) -> bool {
//             if let Some(_) = self.get_value(row, col) {
//                 return false;
//             }
    
//             for i in 0..9 {
//                 if self.get_value(row, i) == Some(value) {
//                     return false;
//                 }
//                 if self.get_value(i, col) == Some(value) {
//                     return false;
//                 }
//             }
    
//             let box_row = (row / 3) * 3;
//             let box_col = (col / 3) * 3;
    
//             for i in box_row..(box_row + 3) {
//                 for j in box_col..(box_col + 3) {
//                     if self.get_value(i, j) == Some(value) {
//                         return false;
//                     }
//                 }
//             }
    
//             true
//         }
    
//         fn is_solved(&self) -> bool {
//             for row in self.cells.iter() {
//                 for cell in row.iter() {
//                     if cell.is_none() {
//                         return false;
//                     }
//                 }
//             }
    
//             for i in 0..9 {
//                 for j in 0..9 {
//                     let value = self.get_value(i, j).unwrap();
    
//                     for k in (i + 1)..9 {
//                         if self.get_value(k,j) == Some(value) {
//                             return false;
//                         }
//                     }
    
//                     for k in (j + 1)..9 {
//                         if self.get_value(i,k) == Some(value) {
//                             return false;
//                         }
//                     }
    
//                     let box_row = (i / 3) * 3;
//                     let box_col = (j / 3) * 3;
    
//                     for r in box_row..(box_row + 3) {
//                         for c in box_col..(box_col + 3) {
//                             if (r != i || c != j) && self.get_value(r, c) == Some(value) {
//                                 return false;
//                             }
//                         }
//                     }
//                 }
//             }
    
//             true
//         }

//         fn solve(& mut self) -> bool {
//             for i in 0..9 {
//                 for j in 0..9 {
//                     if self.get_value(i, j).is_none() {
//                         for value in 1..=9 {
//                             if <Board as IBoard>::is_valid_in_row(self, i, j, value)
//                             && <Board as IBoard>::is_valid_in_column(self, i, j, value)
//                             && <Board as IBoard>::is_valid_in_block(self, i, j, value)
//                             {
//                                 self.set_value(i, j, Some(value));
//                                 if <Board as IBoard>::solve(self) {
//                                     return true;
//                                 }
//                             }
//                         }
//                         self.set_value(i, j, None);
//                         return false;
//                     }
//                 }
//             }
//             true
//         }

//         fn is_valid(&self) -> bool {
//             for i in 0..9 {
//                 for j in 0..9 {
//                     let cell = &self.cells[i][j];
//                     if cell.is_none() {
//                         continue;
//                     }
//                     if !<Board as IBoard>::is_valid_in_row(self, i, j, cell.unwrap())
//                     || !<Board as IBoard>::is_valid_in_column(self, i, j, cell.unwrap())
//                     || !<Board as IBoard>::is_valid_in_block(self, i, j, cell.unwrap())
//                     {
//                         return false;
//                     }
//                 }
//             }
//             true
//         }
        
//         fn is_valid_in_row(&self, row: usize, col: usize, value: u8) -> bool {
//             for j in 0..9 {
//                 if j != col && self.get_value(row,j) == Some(value) {
//                     return false;
//                 }
//             }
//             true
//         }
        
//         fn is_valid_in_column(&self, row: usize, col: usize, value: u8) -> bool {
//             for i in 0..9 {
//                 if i != row && self.get_value(i,col) == Some(value) {
//                     return false;
//                 }
//             }
//             true
//         }
        
//         fn is_valid_in_block(&self, row: usize, col: usize, value: u8) -> bool {
//             let block_row = row / 3;
//             let block_col = col / 3;
//             for i in block_row * 3..(block_row + 1) * 3 {
//                 for j in block_col * 3..(block_col + 1) * 3 {
//                     if i != row && j != col && self.get_value(i,j) == Some(value) {
//                         return false;
//                     }
//                 }
//             }
//             true
//         }
//     }
// }

// fn is_valid_move(&self, row: usize, col: usize, value: u8) -> bool {
//     if self.get_value(row, col).is_some() {
//         return false;
//     }

//     for i in 0..9 {
//         if self.get_value(row, i) == Some(value) {
//             return false;
//         }
//         if self.get_value(i, col) == Some(value) {
//             return false;
//         }
//     }

//     let box_row = (row / 3) * 3;
//     let box_col = (col / 3) * 3;

//     for i in box_row..(box_row + 3) {
//         for j in box_col..(box_col + 3) {
//             if self.get_value(i, j) == Some(value) {
//                 return false;
//             }
//         }
//     }

//     true
// }

// fn is_solved(&self) -> bool {
//     for row in self.cells.iter() {
//         for cell in row.iter() {
//             if cell.is_none() {
//                 return false;
//             }
//         }
//     }

//     for i in 0..9 {
//         for j in 0..9 {
//             let value = self.get_value(i, j).unwrap();

//             for k in (i + 1)..9 {
//                 if self.get_value(k,j) == Some(value) {
//                     return false;
//                 }
//             }

//             for k in (j + 1)..9 {
//                 if self.get_value(i,k) == Some(value) {
//                     return false;
//                 }
//             }

//             let box_row = (i / 3) * 3;
//             let box_col = (j / 3) * 3;

//             for r in box_row..(box_row + 3) {
//                 for c in box_col..(box_col + 3) {
//                     if (r != i || c != j) && self.get_value(r, c) == Some(value) {
//                         return false;
//                     }
//                 }
//             }
//         }
//     }

//     true
// }

// fn random_solved_board(&mut self) {
//     let mut rng = rand::thread_rng();

//     let mut stack = Vec::new();
//     let mut row = 0;
//     let mut col = 0;

//     loop {
//         if row == 9 {
//             break;
//         }

//         let mut valid_values = Vec::new();

//         for value in 1..=9 {
//             if self.is_valid_move(row, col, value) {
//                 valid_values.push(value);
//             }
//         }

//         if valid_values.is_empty() {
//             let (prev_row, prev_col) = stack.pop().unwrap_or((0, 0));
//             row = prev_row;
//             col = prev_col;
//             self.set_value(row, col, None);
//             if col == 0 {
//                 if let Some(sub_row) = row.checked_sub(1) {
//                     row = sub_row;
//                 } else {
//                     print!("");
//                 }
//                 col = 8;
//             } else {
//                 col -= 1;
//             }
//             continue;
//         }

//         let value = valid_values[rng.gen_range(0..valid_values.len())];

//         self.set_value(row, col, Some(value));
//         stack.push((row, col));
//         if col == 8 {
//             row += 1;
//             col = 0;
//         } else {
//             col += 1;
//         }
//     }
// }

// fn solve(& mut self) -> bool {
//     for i in 0..9 {
//         for j in 0..9 {
//             if self.get_value(i, j).is_none() {
//                 for value in 1..=9 {
//                     if <Board as IBoard>::is_valid_in_row(self, i, j, value)
//                     && <Board as IBoard>::is_valid_in_column(self, i, j, value)
//                     && <Board as IBoard>::is_valid_in_block(self, i, j, value)
//                     {
//                         self.set_value(i, j, Some(value));
//                         if <Board as IBoard>::solve(self) {
//                             return true;
//                         }
//                     }
//                 }
//                 self.set_value(i, j, None);
//                 return false;
//             }
//         }
//     }
//     true
// }

// fn solve_helper(&mut self) -> bool {
//     // Find the first empty cell
//     if let Some((row, col)) = self.find_empty_cell() {
//         // Generate a random permutation of the numbers 1 to 9
//         let mut nums: Vec<u8> = (1..=9).collect();
//         nums.shuffle(&mut thread_rng());
//         // Try each number in the empty cell, in random order
//         for num in nums {
//             if self.is_valid_board(row, col, num) {
//                 self.set_value(row, col, Some(num));
//                 // Recursively try to solve the rest of the board
//                 if self.solve_helper() {
//                     return true;
//                 }
//                 // If we reach here, we couldn't solve the rest of the board
//                 self.set_value(row, col, None);
//             }
//         }
//         // If we've tried all numbers and none worked, backtrack
//         false
//     } else {
//         // If there are no empty cells, we've solved the board
//         true
//     }
// }

// fn find_empty_cell(&self) -> Option<(usize, usize)> {
//     // Find the first empty cell
//     for row in 0..9 {
//         for col in 0..9 {
//             if self.get_value(row, col).is_none() {
//                 return Some((row, col));
//             }
//         }
//     }
//     None
// }

// fn random_solved_board(&mut self) {
//     let mut rng = rand::thread_rng();

//     let mut stack = Vec::new();
//     let mut row = 0;
//     let mut col = 0;

//     loop {
//         if row == 9 {
//             break;
//         }

//         let mut valid_values = Vec::new();

//         for value in 1..=9 {
//             if self.is_valid_move(row, col, value) {
//                 valid_values.push(value);
//             }
//         }

//         if valid_values.is_empty() {
//             let (prev_row, prev_col) = stack.pop().unwrap_or((0, 0));
//             row = prev_row;
//             col = prev_col;
//             self.set_value(row, col, None);
//             if col == 0 {
//                 if let Some(sub_row) = row.checked_sub(1) {
//                     row = sub_row;
//                 } else {
//                     print!("");
//                 }
//                 col = 8;
//             } else {
//                 col -= 1;
//             }
//             continue;
//         }

//         let value = valid_values[rng.gen_range(0..valid_values.len())];

//         self.set_value(row, col, Some(value));
//         stack.push((row, col));
//         if col == 8 {
//             row += 1;
//             col = 0;
//         } else {
//             col += 1;
//         }
//     }
// }

// fn solve(& mut self) -> bool {
//     for i in 0..9 {
//         for j in 0..9 {
//             if self.get_value(i, j).is_none() {
//                 for value in 1..=9 {
//                     if <Board as IBoard>::is_valid_in_row(self, i, j, value)
//                     && <Board as IBoard>::is_valid_in_column(self, i, j, value)
//                     && <Board as IBoard>::is_valid_in_block(self, i, j, value)
//                     {
//                         self.set_value(i, j, Some(value));
//                         if <Board as IBoard>::solve(self) {
//                             return true;
//                         }
//                     }
//                 }
//                 self.set_value(i, j, None);
//                 return false;
//             }
//         }
//     }
//     true
// }


        
// fn is_valid_in_row(&self, row: usize, col: usize, value: u8) -> bool {
//     for c in 0..9 {
//         if c != col && self.get_value(row,c) == Some(value) {
//             return false;
//         }
//     }
//     true
// }

// fn is_valid_in_column(&self, row: usize, col: usize, value: u8) -> bool {
//     for r in 0..9 {
//         if r != row && self.get_value(r,col) == Some(value) {
//             return false;
//         }
//     }
//     true
// }

// fn is_valid_in_block(&self, row: usize, col: usize, value: u8) -> bool {
//     let block_row = row / 3;
//     let block_col = col / 3;
//     for r in block_row * 3..(block_row + 1) * 3 {
//         for c in block_col * 3..(block_col + 1) * 3 {
//             if r != row && c != col && self.get_value(r,c) == Some(value) {
//                 return false;
//             }
//         }
//     }
//     true
// }

//             // Check if there are any duplicates in rows, columns and blocks
//             for i in 0..9 {
//                 for j in 0..9 {
//                     let value = self.get_value(i, j).unwrap();
                    
//                     // Check row
//                     for k in (i + 1)..9 {
//                         if self.get_value(k,j) == Some(value) {
//                             return false;
//                         }
//                     }
                    
//                     // Check column
//                     for k in (j + 1)..9 {
//                         if self.get_value(i,k) == Some(value) {
//                             return false;
//                         }
//                     }
                    
//                     // Check block (3x3)
//                     let box_row = (i / 3) * 3;
//                     let box_col = (j / 3) * 3;
    
//                     for r in box_row..(box_row + 3) {
//                         for c in box_col..(box_col + 3) {
//                             if (r != i || c != j) && self.get_value(r, c) == Some(value) {
//                                 return false;
//                             }
//                         }
//                     }
//                 }
//             }


//             fn is_board_solved(&self) -> bool {
//                 // First check if there is empty values
//                 for row in self.cells.iter() {
//                     for cell in row.iter() {
//                         if cell.is_none() {
//                             return false;
//                         }
//                     }
//                 }
                
//                 // Check if there are any duplicates in rows, columns and blocks
//                 for i in 0..9 {
//                     for j in 0..9 {
//                         let value = self.get_value(i, j).unwrap();
                        
//                         // Check row
//                         for k in (i + 1)..9 {
//                             if self.get_value(k,j) == Some(value) {
//                                 return false;
//                             }
//                         }
                        
//                         // Check column
//                         for k in (j + 1)..9 {
//                             if self.get_value(i,k) == Some(value) {
//                                 return false;
//                             }
//                         }
                        
//                         // Check block (3x3)
//                         let box_row = (i / 3) * 3;
//                         let box_col = (j / 3) * 3;
        
//                         for r in box_row..(box_row + 3) {
//                             for c in box_col..(box_col + 3) {
//                                 if (r != i || c != j) && self.get_value(r, c) == Some(value) {
//                                     return false;
//                                 }
//                             }
//                         }
//                     }
//                 }
        
//                 true
//             }