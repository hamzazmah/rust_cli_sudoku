use serde::{Serialize, Deserialize};
use super::Board;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct User {
    pub name: String,
    pub stats: Stats,
    pub last_game: Option<(Board, Option<u64>)>
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize)]
pub struct Stats {
    pub wins: u32,
    pub losses: u32
}

impl User {
    pub fn new(name: &str) -> User {
        User {
            name: String::from(name),
            stats: Stats {
                wins: 0,
                losses: 0
            },
            last_game: None
        }
    }

    pub fn win(&mut self) {
        self.stats.wins += 1;
        self.last_game = None;
    }

    pub fn lose(&mut self) {
        self.stats.losses += 1;
        self.last_game = None;
    }
}