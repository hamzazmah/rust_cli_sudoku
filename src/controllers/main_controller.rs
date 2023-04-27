/// The function `play_sudoku_game` sets up and plays a game of Sudoku.
mod main_controller {
    //importing the traits
    use rand::Rng;
    use std::io::{self, Stdin, BufRead};
    use std::fs::File;
    use std::path::Path;
    use std::time::{Duration, Instant};

    //importing the models
    pub use crate::models::{Game, Board, User, Difficulty, GameType};

    //importing the custom models traits
    pub use crate::controllers::game_controller::game_controller::IGame;
    pub use crate::controllers::board_controller::board_controller::IBoard;
    pub use crate::controllers::user_controller::user_controller;

    /// Setup the User for the game
    fn setup_user(stdin: &mut Stdin) -> User {
        let mut input = String::new();

        println!("Please enter your name to continue:");
        loop {
            input.clear();
            stdin.read_line(&mut input).unwrap();
            
            input = input.trim().to_string();

            if input.eq("quit") || input.eq("exit") {
                std::process::abort();
            }
            else if input.is_empty() {
                println!("Name cannot be empty. ⛔️");
                continue;
            }
            else if input.len() <= 3 {
                println!("Name is too short. Please input a name longer than 3 characters. ⛔️");
                continue;
            }
            else {
                for c in input.chars() {
                    if !c.is_alphabetic() {
                        println!("Input contains non-alphabetic characters. ⛔️");
                        continue;
                    }
                }
            }

            break;
        }

        // Get the user from file or create a new one
        let user = user_controller::get_user(&input);
        user_controller::add_update_user(&user);

        user
    }

    // Setup the difficulty for the game
    fn setup_difficulty(stdin: &mut Stdin) -> Difficulty {
        let mut input = String::new();

        println!("Now Choose a difficulty level:");
        println!("1. Easy");
        println!("2. Medium");
        println!("3. Hard");
        loop {
            input.clear();
            stdin.read_line(&mut input).unwrap();
    
            match input.trim() {
                "quit" | "q" => std::process::abort(),
                "1" => return Difficulty::Easy,
                "2" => return Difficulty::Medium,
                "3" => return Difficulty::Hard,
                _ => {
                    println!("Invalid input, please try again. ⛔️");
                    continue;
                }
            }
        }
    }

    // Setup the grid size for the game (6, 8, 9, 10, 12, 16)
    fn get_size(stdin: &mut Stdin) -> usize {
        let mut input = String::new();

        println!("Now Choose a Grid Size (Supported sizes are: 6, 8, 9, 10, 12, 16.):");
        println!("Notice! Grid Sizes greater than 9 may take a while to generate.");
        loop {
            input.clear();
            stdin.read_line(&mut input).unwrap();
    
            match input.trim().parse::<usize>() {
                Ok(num) => {
                    match num {
                        6 | 8 | 9 | 10 | 12 | 16 => return num,
                        _ => {
                            println!("Invalid size, please try again. ⛔️");
                            println!("Valid sizes are: 6, 8, 9, 10, 12, 16.");
                            continue;
                        }
                    }
                },
                Err(_) => {
                    match input.trim() {
                        "quit" | "q" => std::process::abort(),
                        _ => {
                            println!("Invalid column, please try again. ⛔️");
                            continue;
                        }
                    }
                }
            };
    
        }
    }

    // Setup the game type (New, NewTimed, Solver, Solved)
    fn setup_type(stdin: &mut Stdin) -> GameType {
        let mut input = String::new();

        println!("Choose what you want to do:");
        println!("1. Play a normal game");
        println!("2. Play a timed game");
        println!("3. Solve a game");
        println!("4. View a solved game");

        loop {
            input.clear();
            stdin.read_line(&mut input).unwrap();
            
            match input.trim() {
                "quit" | "q" => std::process::abort(),
                "1" => return GameType::New,
                "2" => return GameType::NewTimed,
                "3" => return GameType::Solver,
                "4" => return GameType::Solved,
                _ => {
                    println!("Invalid input, please try again. ⛔️");
                    continue;
                }
            };
        }
    }

    // Setup the game
    fn setup_game(game: &mut Game) {
        let mut rng = rand::thread_rng();
        let size = game.board.get_size();

        // Generate a random number of solved cells based on the difficulty and size
        let num_solved_cells = match game.difficulty {
            Difficulty::Easy => match size {
                6 => rng.gen_range(25..=30),
                8 => rng.gen_range(45..=55),
                9 => rng.gen_range(55..=65),
                10 => rng.gen_range(70..=80),
                12 => rng.gen_range(110..=120),
                _ => rng.gen_range(200..=220),
            },
            Difficulty::Medium => match size {
                6 => rng.gen_range(15..=25),
                8 => rng.gen_range(35..=45),
                9 => rng.gen_range(45..=55),
                10 => rng.gen_range(55..=65),
                12 => rng.gen_range(95..=110),
                _ => rng.gen_range(180..=200),
            },
            Difficulty::Hard => match size {
                6 => rng.gen_range(10..=15),
                8 => rng.gen_range(20..=35),
                9 => rng.gen_range(35..=45),
                10 => rng.gen_range(45..=55),
                12 => rng.gen_range(75..=95),
                _ => rng.gen_range(150..=180),
            },
        };

        // create a random solved board
        let mut solved_board = Board::new(game.board.cells.len());
        solved_board.generate_board();
        game.board = solved_board;

        // If user want to view a Solved Board, then we don't need to remove any cells
        if game.game_type == GameType::Solved {
            return;
        } else {
            let mut removed_cells = (size * size) - num_solved_cells;

            while removed_cells > 0 {
                let row = rng.gen_range(0..game.board.get_size());
                let col = rng.gen_range(0..game.board.get_size());

                if let Some(value) = game.board.get_value(row, col) {
                    game.hints.push((row, col, value));                              
                    game.board.set_value(row, col, None);
                    removed_cells -= 1;
                }
            }
        }
    }

    // Main Play loop for the game
    fn play(game: &mut Game, stdin: &mut Stdin) {
        let mut input = String::new();
        let start_time = Instant::now();
        loop {
            println!("{}", game.board);

            // Display the timer (if any) in the terminal
            if let Some(duration) = game.timer {
                let duration = duration * 60;
                let elapsed_time = start_time.elapsed().as_secs();
                let remaining_time = duration.checked_sub(elapsed_time).unwrap_or_else(|| Duration::from_secs(0).as_secs());
                println!("Time remaining: {} minutes {} seconds", remaining_time / 60, remaining_time % 60);
                // If the time is up, then the user loses the game. Update the user's stats and break out of the loop
                if elapsed_time >= duration {
                    println!("Time's up! You lost the game 🙁");
                    game.user.lose();
                    user_controller::add_update_user(&game.user);
                    println!("Wins: {}, Losses: {}", game.user.stats.wins, game.user.stats.losses);
                    break;
                }
            }
            
            // Check if the board is valid and no empty cells, if so then the user has solved the puzzle. Update the user's stats and break out of the loop
            if game.board.is_board_valid(false) {
                println!("Congratulations, you solved the puzzle!  🥳 🥳 🥳");
                game.user.win();
                user_controller::add_update_user(&game.user);
                println!("Wins: {}, Losses: {}", game.user.stats.wins, game.user.stats.losses);
                break;
            }
    
            println!("Enter your move as Row Column Value, e.g. '1 2 3', or 'u' to undo a move, or 'r' to redo a previously undone move, or '1 2 h' to get a hint for that Cell:");
    
            input.clear();
            stdin.read_line(&mut input).unwrap();
            
            if input.trim() == "quit" || input.trim() == "q" {
                let remaining_time: Option<u64> = if let Some(duration) = game.timer {
                    let duration = duration * 60;
                    let elapsed_time = start_time.elapsed().as_secs();
                    Some(duration.checked_sub(elapsed_time).unwrap_or_else(|| Duration::from_secs(0).as_secs()) / 60)
                } else {
                    None
                };
                game.user.last_game = Some((game.board.clone(), remaining_time, game.hints.clone()));
                user_controller::add_update_user(&game.user);
                std::process::abort();
            }

            if input.trim() == "r" {
                if game.redo_move() {
                    println!("Redoing last undone move. ↪️");
                } else {
                    println!("No more moves to redo. ⛔️");
                }
                continue;
            }

            if input.trim() == "u" {
                if game.undo_move() {
                    println!("Undoing last move. ↩️");
                } else {
                    println!("No more moves to undo. ⛔️");
                }
                continue;
            }
            
            // Parse the user's input into a Column, Row, and Value
            let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
            if parts.len() != 3 {
                println!("Invalid input, please try again. ⛔️");
                continue;
            }
    
            let row = match parts[0].parse::<usize>() {
                Ok(num) => num - 1,
                Err(_) => {
                    println!("Invalid row, please try again. ⛔️");
                    continue;
                }
            };
    
            let col = match parts[1].parse::<usize>() {
                Ok(num) => num - 1,
                Err(_) => {
                    println!("Invalid column, please try again. ⛔️");
                    continue;
                }
            };
    
            let value = match parts[2].parse::<u32>() {
                Ok(num) => Some(num),
                Err(_) => {
                    // If the user entered a 'h' for value, then they want a hint for that cell
                    if parts[2] == "h" {
                        let hint = game.give_hint(row, col);

                        if hint.is_none() {
                            println!("No hints available for this cell. ⛔️");
                        } else {
                            println!("Hint for Row: {}, Col: {} => {} 🤫", hint.unwrap().0, hint.unwrap().1, hint.unwrap().2);
                        }
                    } else {
                        println!("Invalid value, please try again. ⛔️");
                    }
                    continue;
                }
            };
            
            // Check if the cell already has a value, if so then the user cannot change it
            if game.board.get_value(row, col).is_some() {
                println!("Cell already has a value, please try again. ⛔️");
                continue;
            } 
            if game.make_move(row, col, value) {
                println!("Move is Valid and is now added to the board. ✅");
            } else {
                println!("Invalid move, please try again. ⛔️");
            }
        }
    }

    // Function to read a board from a file
    fn read_board_from_file(filename: &str) -> Result<Board, io::Error> {    
        // Open the file and get Siz and read the lines
        let file = File::open(filename)?;
        let size = get_size(&mut io::stdin());
        let lines = io::BufReader::new(file).lines();

        let mut board = Board::new(size);
        
        // Iterate through the lines and parse the values into the board
        for (i, line) in lines.enumerate() {
            let line = line?;
            let values: Vec<u32> = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap_or(0))
                .collect();
            
            // Check if the number of values in the line is equal to the size of the board
            if values.len() != size {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Invalid number of values in line {}: expected {}, got {} ⛔️",
                        i + 1,
                        size,
                        values.len()
                    ),
                ));
            }
            
            // Iterate through the values and set them in the board
            for (j, &value) in values.iter().enumerate() {
                if value > 0 && value <= 9 {
                    board.set_value(i, j, Some(value));
                }
            }
        }
    
        Ok(board)
    }

    // Function to solve a board read from a file
    fn sudoku_solver() {
        let mut input = String::new();
        let stdin = io::stdin();

        loop {
            println!("Please enter the path to the input file (Currently only supports 9x9):");
            println!("View the example 'board.txt' file for an example.");

            input.clear();
            stdin.read_line(&mut input).unwrap();

            if input.trim() == "quit" || input.trim() == "q" {
                std::process::abort();
            }

            let path = Path::new(input.trim());
            if let Some("txt") = path.extension().and_then(|ext| ext.to_str()) {
                let result = read_board_from_file(input.trim());

                let mut board = match result {
                    Ok(board) => board,
                    Err(err) => {
                        println!("Error reading input file: {} ⛔️", err);
                        continue;
                    }
                };

                if !board.is_board_valid(true) {
                    println!("Board is not valid! Please check the input file and try again. ⛔️");
                }
                else if board.generate_board() {
                    // Print the solved board
                    println!("Here is the solved board:");
                    println!("{}", board);
                    break;
                } else {
                    println!("Could not solve the input board 🙁");
                    break;
                }
            } else {
                println!("File is not a .txt file! ⛔️");
            }
        }
    }

    // Main function to run the game
    pub fn play_game() {
        println!("Welcome to Rust Sudoku. 👋");

        let user = setup_user(&mut io::stdin());
        let cloned_user = user.clone();

        // Check if the user has a last game saved and ask if they want to resume it
        if user.last_game.is_some() {
            println!("Welcome back to Rust Sudoku, {}!", user.name);
            println!("Wins: {}, Losses: {}", user.stats.wins, user.stats.losses);
            println!("Would you like to resume your last game? (y/n)");
            let mut input = String::new();
            let stdin = io::stdin();
            loop {
                input.clear();
                stdin.read_line(&mut input).unwrap();
                match input.trim().to_lowercase().as_str() {
                    "y" | "yes" => {
                        let (board, timer, hints) = cloned_user.last_game.unwrap();
                        let game_type = if timer.is_some() { GameType::NewTimed } else { GameType::New };
                        let mut game = Game::new(user.clone(), None, game_type, timer, Some(board.cells.len()), Some(hints));
                        game.board = board;
                        play(&mut game, &mut io::stdin());
                        break;
                    },
                    "n" | "no" => break,
                    _ => println!("Invalid input, please try again. ⛔️")
                }
            }
        }
        println!("Enter 'q' or 'quit' at any time to quit the game.");
        // Main game loop
        loop {
            let game_type= setup_type(&mut io::stdin());
            match game_type {
                GameType::Solver => sudoku_solver(),
                _ => {
                    let mut difficulty = None;
                    let mut timer = None;
    
                    if game_type == GameType::New || game_type == GameType::NewTimed {
                        difficulty = Some(setup_difficulty(&mut io::stdin()));

                        if game_type == GameType::NewTimed {
                            timer = setup_timer(&difficulty);
                        }
                    }

                    let size = Some(get_size(&mut io::stdin())); 
    
                    let mut game = Game::new(user.clone(), difficulty, game_type, timer, size, None);
            
                    println!("Welcome to Rust Sudoku, {}!", game.user.name);
                    println!("Wins: {}, Losses: {}", game.user.stats.wins, game.user.stats.losses);
            
                    setup_game(&mut game);
    
                    if game.game_type == GameType::Solved {
                        println!("Here is a random solved board: 🔢");
                        println!("{}", game.board);
                    } else {
                        play(&mut game, &mut io::stdin());
                    }

                    game.user = user_controller::get_user(&game.user.name); // Update the user in case they changed their stats
                }
            }
        }
    }

    fn setup_timer(difficulty: &Option<Difficulty>) -> Option<u64> {
        match difficulty {
            Some(Difficulty::Easy) => Some(25),
            Some(Difficulty::Medium) => Some(15),
            Some(Difficulty::Hard) => Some(10),
            _ => None
        }
    }  
}

pub fn play_sudoku_game() {
    super::main_controller::main_controller::play_game();
}