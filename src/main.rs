pub mod models;
pub mod controllers;
pub mod views;
pub mod tests;

use controllers::play_sudoku_game;

fn main() {
    play_sudoku_game();
}