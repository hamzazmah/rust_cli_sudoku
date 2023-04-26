use std::fmt;

use crate::models::board::Board;

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        s += "┏━━━━━━━━━┳━━━━━━━━━┳━━━━━━━━━┓\n";
        s += "┃       Rust Sudoku Cli       ┃\n";

        for i in 0..9 {
            if i % 3 == 0 {
                s += "┣━━━━━━━━━╋━━━━━━━━━╋━━━━━━━━━┫\n";
            }

            for j in 0..9 {
                if j % 3 == 0 {
                    s += "┃";
                }

                if let Some(value) = self.cells[i][j] {
                    s += &format!(" {} ", value);
                } else {
                    s += "   ";
                }
            }

            s += "┃\n";
        }

        s += "┗━━━━━━━━━┻━━━━━━━━━┻━━━━━━━━━┛\n";
        write!(f, "{}", s)
    }
}