/// This code defines a module called `game_controller` that contains a trait called `IGame` and an
/// implementation of that trait for the `Game` struct. The `IGame` trait defines four methods:
/// `make_move`, `redo_move`, `undo_move`, and `give_hint`. The implementation of `IGame` for `Game`
/// provides the actual implementation of these methods for the `Game` struct.
pub mod game_controller {
    use crate::models::Game;
    pub use crate::controllers::board_controller::board_controller::IBoard;

    pub trait IGame {
        fn make_move(&mut self, row: usize, col: usize, value: Option<u32>) -> bool;
        fn redo_move(&mut self) -> bool;
        fn undo_move(&mut self) -> bool;
        fn give_hint(&mut self, row: usize, col: usize) -> Option<(usize, usize, u32)>;
    }

    impl IGame for Game {    
        // Make a move on the board 
        fn make_move(&mut self, row: usize, col: usize, value: Option<u32>) -> bool {
            if self.board.is_move_valid(row, col, value.unwrap()) {
                self.board.set_value(row, col, value);
                self.moves.push((row, col, value));
                true
            } else {
                false
            }
        }
        
        // Redo a move. Used for undoing an undo.
        fn redo_move(&mut self) -> bool {
            if let Some((row, col, value)) = self.undone_moves.pop() {
                self.board.set_value(row, col, value);
                self.moves.push((row, col, value));
                true
            } else {
                false
            }
        }

        // Undo a move on the board
        fn undo_move(&mut self) -> bool {
            if let Some((row, col, value)) = self.moves.pop() {
                self.board.set_value(row, col, None);
                self.undone_moves.push((row, col, value));
                true
            } else {
                false
            }
        }

        // Give a hint for a move
        fn give_hint(&mut self, row: usize, col: usize) -> Option<(usize, usize, u32)> {
            if self.board.get_value(row, col).is_some() {
                return None;
            }

            if let Some((_, _, value)) = self.hints.iter().find(|&&(r, c, _)| r == row && c == col) {
                return Some((row + 1, col + 1, *value));
            } 

            None
        }
    }
}