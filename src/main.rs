pub mod models;
pub mod controllers;
pub mod views;

use controllers::play_sudoku_game;

fn main() {
    play_sudoku_game();
}