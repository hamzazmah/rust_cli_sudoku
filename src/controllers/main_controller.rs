mod main_controller {
    //importing the traits
    use rand::Rng;
    use std::io::{self, Stdin, BufRead};
    use std::fs::File;
    use std::thread;
    use std::time::{Duration, Instant};

    //importing the models
    pub use crate::models::{Game, Board, User, Difficulty, GameType};

    //importing the custom models traits
    pub use crate::controllers::game_controller::game_controller::IGame;
    pub use crate::controllers::board_controller::board_controller::IBoard;
    pub use crate::controllers::user_controller::user_controller;

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
                println!("Name cannot be empty.");
                continue;
            }
            else if input.len() <= 3 {
                println!("Name is too short. Please input a name longer than 3 characters.");
                continue;
            }
            else {
                for c in input.chars() {
                    if !c.is_alphabetic() {
                        println!("Input contains non-alphabetic characters.");
                        continue;
                    }
                }
            }

            break;
        }

        let user = user_controller::get_user(&input);
        user_controller::add_update_user(&user);

        user
    }

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
                    println!("Invalid input, please try again.");
                    continue;
                }
            }
        }
    }

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
                    println!("Invalid input, please try again.");
                    continue;
                }
            };
        }
    }

    fn setup_game(game: &mut Game) {
        let mut rng = rand::thread_rng();

        let num_solved_cells = match game.difficulty {
            Difficulty::Easy => rng.gen_range(50..=60),
            Difficulty::Medium => rng.gen_range(40..50),
            Difficulty::Hard => rng.gen_range(20..=35),
        };

        // create a random solved board
        let mut solved_board = Board::new();
        solved_board.generate_board();

        game.board = solved_board;

        if game.game_type == GameType::Solved {
            return;
        } else {
            let mut removed_cells = 81 - num_solved_cells;

            while removed_cells > 0 {
                let row = rng.gen_range(0..9);
                let col = rng.gen_range(0..9);

                if let Some(value) = game.board.get_value(row, col) {
                    game.hints.push((row, col, value));                              
                    game.board.set_value(row, col, None);
                    removed_cells -= 1;
                }
            }
        }
    }

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
                if elapsed_time >= duration {
                    println!("Time's up!");
                    game.user.lose();
                    user_controller::add_update_user(&game.user);
                    break;
                }
            }
    
            if game.board.is_board_valid(false) {
                println!("Congratulations, you solved the puzzle!");
                game.user.win();
                user_controller::add_update_user(&game.user);
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
                game.user.last_game = Some((game.board.clone(), remaining_time));
                user_controller::add_update_user(&game.user);
                std::process::abort();
            }

            if input.trim() == "r" {
                if game.redo_move() {
                    println!("Redoing last undone move.");
                } else {
                    println!("No more moves to redo.");
                }
                continue;
            }

            if input.trim() == "u" {
                if game.undo_move() {
                    println!("Undoing last move.");
                } else {
                    println!("No more moves to undo.");
                }
                continue;
            }
    
            let parts: Vec<&str> = input.trim().split_whitespace().collect();
    
            if parts.len() != 3 {
                println!("Invalid input, please try again.");
                continue;
            }
    
            let row = match parts[0].parse::<usize>() {
                Ok(num) => num - 1,
                Err(_) => {
                    println!("Invalid row, please try again.");
                    continue;
                }
            };
    
            let col = match parts[1].parse::<usize>() {
                Ok(num) => num - 1,
                Err(_) => {
                    println!("Invalid column, please try again.");
                    continue;
                }
            };
    
            let value = match parts[2].parse::<u8>() {
                Ok(num) => Some(num),
                Err(_) => {
                    if parts[2] == "h" {
                        let hint = game.give_hint(row, col);

                        if hint.is_none() {
                            println!("No hints available for this cell.");
                        } else {
                            println!("Hint for Row: {}, Col: {} => {}", hint.unwrap().0, hint.unwrap().1, hint.unwrap().2);
                        }
                    } else {
                        println!("Invalid value, please try again.");
                    }
                    continue;
                }
            };
            
            if game.board.get_value(row, col).is_some() {
                println!("Cell already has a value, please try again.");
                continue;
            } else if !game.make_move(row, col, value) {
                println!("Invalid move, please try again.");
            }
        }
    }

    fn read_board_from_file(filename: &str) -> Result<Board, io::Error> {
        let mut board = Board::new();
    
        let file = File::open(filename)?;
        let lines = io::BufReader::new(file).lines();
    
        for (i, line) in lines.enumerate() {
            let line = line?;
            let values: Vec<u8> = line
                .trim()
                .split_whitespace()
                .map(|x| x.parse().unwrap_or(0))
                .collect();
    
            if values.len() != 9 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidData,
                    format!(
                        "Invalid number of values in line {}: expected 9, got {}",
                        i + 1,
                        values.len()
                    ),
                ));
            }
    
            for (j, &value) in values.iter().enumerate() {
                if value > 0 && value <= 9 {
                    board.set_value(i, j, Some(value));
                }
            }
        }
    
        Ok(board)
    }

    fn sudoku_solver() {
        let mut input = String::new();
        let stdin = io::stdin();

        println!("Please enter the path to the input file:");
        loop {
            input.clear();
            stdin.read_line(&mut input).unwrap();

            let result = read_board_from_file(input.trim());

            let mut board = match result {
                Ok(board) => board,
                Err(err) => {
                    println!("Error reading input file: {}", err);
                    continue;
                }
            };

            if !board.is_board_valid(true) {
                println!("Board is not valid! Please check the input file and try again.");
            }
            else if board.generate_board() {
                // Print the solved board
                println!("Here is the solved board:");
                println!("{}", board);
                break;
            } else {
                println!("Could not solve the input board");
                break;
            }
        }
    }

    pub fn play_game() {
        println!("Welcome to Rust Sudoku.");

        let user = setup_user(&mut io::stdin());

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
                        let (board, timer) = user.last_game.unwrap();
                        let game_type = if timer.is_some() { GameType::NewTimed } else { GameType::New };
                        let mut game = Game::new(user.clone(), None, game_type, timer);
                        game.board = board;
                        play(&mut game, &mut io::stdin());
                        break;
                    },
                    "n" | "no" => break,
                    _ => println!("Invalid input, please try again.")
                }
            }
        }

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
    
                    let mut game = Game::new(user.clone(), difficulty, game_type, timer);
            
                    println!("Welcome to Rust Sudoku, {}!", game.user.name);
                    println!("Wins: {}, Losses: {}", game.user.stats.wins, game.user.stats.losses);
            
                    setup_game(&mut game);
    
                    if game.game_type == GameType::Solved {
                        println!("Here is a random solved board:");
                        println!("{}", game.board);
                    } else {
                        play(&mut game, &mut io::stdin());
                    }
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