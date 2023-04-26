use super::{Board, User};

pub struct Game {
    pub board: Board,
    pub hints: Vec<(usize, usize, u8)>,
    pub moves: Vec<(usize, usize, Option<u8>)>,
    pub undone_moves: Vec<(usize, usize, Option<u8>)>,
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
    pub fn new(user: User, difficulty: Option<Difficulty>, game_type: GameType, timer: Option<u64>) -> Self {
        Self {
            board: Board::new(),
            moves: Vec::new(),
            undone_moves: Vec::new(),
            hints: Vec::new(),
            user,
            difficulty: difficulty.unwrap_or(Difficulty::Easy),
            game_type,
            timer
        }
    }
}