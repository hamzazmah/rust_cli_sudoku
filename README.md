# Rust CLI Sudoku Game

## How to setup
> Make sure [Rust & Cargo](https://www.rust-lang.org/tools/install) are installed and setup properly

Run these inside a terminal
- `git clone https://github.com/hamzazmah/rust_cli_sudoku.git`
- `cd rust_cli_sudoku`
- `cargo run`

This will create an .exe file at `./target/debug/rust_cli_sudoku.exe` which you can use to run the application or use `cargo run`.

## Main Features

 1. Basic Authentication: The application prompts the user for their name and retrieves their details from a JSON file, if found else adds a new user to the JSON file.
 2. Replay last game: The application saves a game when the user quits mid-way and prompts them if they would like to replay the saved game the next time they enter the game. 
 3. Difficulty Settings: The application allows the user to choose the difficulty level of the Sudoku game. This is used to hide a number of cells from the user, easy difficulty means the board will have fewer empty cells and hence easier to solve.
 4. Board Size Options: The application currently supports board sizes of 6, 8, 9, 10, 12, and 16 and allows the user to choose a board size before game starts. 
 5. User Input Validation: Basic validation checks are performed on user input to ensure that it meets the requirements of the game.
 6. Board Generation: The application generates a solved board at the start of each game. 
 7. Hint Feature: The application provides the user with hints if requested. 
 8. Undo/Redo Functionality: The application provides the user with the ability to undo or redo a move. 
 9. Win/Loss Tracking: The application keeps track of the user wins and losses. 
 10. Timer Functionality: The application allows the user to play against a timer. This is also based on the difficulty setting, easy difficulty means more time to solve. 
 11. View Solved Board: The application provides the user with the option to view a random solved board. 
 12. Board Solving Functionality from a file: The application allows the user to solve a given Sudoku board. The application prompts the user to input a file name and then it performs a validation check to see if it is solvable and then prints out the solved board.

## How to play

 1. Start the game and Enter your name (this will be used to track your stats)
 2. Choose a Normal or Timed Game
 3. Choose Difficulty
 4. Choose Board Size
 5. Enter a move in the following order `row column value` `1 2 3`
 6. `u` to Undo a Move
 7. `r` to Redo a undone move
 8. `row column h` `1 2 h` to get a hint for that column
 
 ## how to Solve a board in a .txt file
 
 1. Add a file inside the root directory if running `cargo run` or if running from an .exe inside the directory where the .exe is located. Check the example `board.txt` file for reference layout
 2. Choose option 3
 3.  input the file name with the extension `board.txt`
