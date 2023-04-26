pub mod game_controller {
    use rand::{seq::SliceRandom, thread_rng};

    use crate::models::Game;
    pub use crate::controllers::board_controller::board_controller::IBoard;

    pub trait IGame {
        fn make_move(&mut self, row: usize, col: usize, value: Option<u8>) -> bool;
        fn redo_move(&mut self) -> bool;
        fn undo_move(&mut self) -> bool;
        fn give_hint(&mut self, row: usize, col: usize) -> Option<(usize, usize, u8)>;
    }

    impl IGame for Game {    
        fn make_move(&mut self, row: usize, col: usize, value: Option<u8>) -> bool {
            if self.board.is_move_valid(row, col, value.unwrap()) {
                self.board.set_value(row, col, value);
                self.moves.push((row, col, value));
                true
            } else {
                false
            }
        }
    
        fn redo_move(&mut self) -> bool {
            if let Some((row, col, value)) = self.undone_moves.pop() {
                self.board.set_value(row, col, value);
                self.moves.push((row, col, value));
                true
            } else {
                false
            }
        }

        fn undo_move(&mut self) -> bool {
            if let Some((row, col, value)) = self.moves.pop() {
                self.board.set_value(row, col, None);
                self.undone_moves.push((row, col, value));
                true
            } else {
                false
            }
        }

        fn give_hint(&mut self, row: usize, col: usize) -> Option<(usize, usize, u8)> {
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