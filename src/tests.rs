#[cfg(test)]
mod tests {
    use crate::controllers::IGame;
    pub use crate::models::{Board, Game, User, Difficulty, GameType};
    pub use crate::controllers::IBoard;
    pub use crate::controllers::{add_update_user, get_user};

    fn create_board() -> Game {
        let user = User::new("Test");
        Game::new(user, None, GameType::New, None, Some(9), None)
    }

    #[test]
    fn create_solve_display_sudoku_board_success() {
        let mut game = create_board(); // Create a new game
        game.board.generate_board(); // Generate a new Solved board
        print!("{}", game.board); // Print the board
        assert!(game.board.is_board_valid(false)); // Check if the board is valid
    }

    #[test]
    fn make_redo_undo_move_success() {
        let mut game = create_board();
        assert!(game.make_move(1, 1, Some(1))); // Make a new Valid move
        assert!(game.undo_move()); // Undo the newly made move
        assert!(game.redo_move()); // Redo the undone move
    }

    #[test]
    fn get_update_user_success() {
        let mut user = get_user("Test");
        let mut user_updated = user.clone(); // Clone the user
        user.name = "Test2".to_string(); // Update the user name
        add_update_user(&user); // Update the user in file
        user_updated = get_user("Test2"); // Get the updated user
        assert!(user.name == user_updated.name);
    }
}