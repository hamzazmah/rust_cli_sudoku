use super::{Board, User};

// Game struct to hold the board, moves, and user
pub struct Game {
    pub board: Board,
    pub hints: Vec<(usize, usize, u32)>,
    pub moves: Vec<(usize, usize, Option<u32>)>,
    pub undone_moves: Vec<(usize, usize, Option<u32>)>,
    pub user: User,
    pub difficulty: Difficulty,
    pub game_type: GameType,
    pub timer: Option<u64>
}

#[derive(Debug)]
pub enum Difficulty {
    Easy,
    Medium,
    Hard
}

#[derive(Debug, PartialEq)]
pub enum GameType {
    New,
    NewTimed,
    Solver,
    Solved
}

impl Game {
    pub fn new(user: User, difficulty: Option<Difficulty>, game_type: GameType, timer: Option<u64>, size: Option<usize>, hints: Option<Vec<(usize, usize, u32)>>) -> Self {
        Self {
            board: Board::new(size.unwrap_or(9)),
            moves: Vec::new(),
            undone_moves: Vec::new(),
            hints: hints.unwrap_or(Vec::new()),
            user,
            difficulty: difficulty.unwrap_or(Difficulty::Easy),
            game_type,
            timer
        }
    }
}