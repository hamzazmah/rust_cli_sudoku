use std::fmt;

use crate::models::board::Board;

impl fmt::Display for Board {
    // Display the board in a nice format.
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut s = String::new();

        let size = self.get_size();
        let (subgrid_size_row, subgrid_size_col) = self.get_subgrid();

        let padding = (size * 3) + subgrid_size_row - 1;

        s += &format!("┏{}┓\n", "━".repeat(padding));

        let header_text = "Rust Sudoku Cli";
        let header_padding = (padding - header_text.len())/2;

        if (padding - header_text.len()) % 2 != 0 {
            s += &format!("┃{}{}{}┃\n", " ".repeat(header_padding), header_text, " ".repeat(header_padding + 1));
        } else {
            s += &format!("┃{}{}{}┃\n", " ".repeat(header_padding), header_text, " ".repeat(header_padding));
        }

        // Loop through each row and column and print the value
        for i in 0..size as usize {
            if i % subgrid_size_row == 0 {
                s += &format!("┣{}┫\n", "━".repeat(padding));
            }

            for j in 0..size as usize {
                if j % subgrid_size_col == 0 {
                    s += "┃";
                }

                // If value is double digit convert to letter for formatting
                if let Some(value) = self.get_value(i, j) {
                    let alph_val = match value {
                        10 => "A",
                        11 => "B",
                        12 => "C",
                        13 => "D",
                        14 => "E",
                        15 => "F",
                        16 => "G",
                        _ => "",
                    };

                    if value > 9 {
                        s += &format!(" {} ", alph_val);
                    } else {
                        s += &format!(" {} ", value);
                    }
                } else {
                    s += "   ";
                }
            }

            s += "┃\n";
        }

        s += &format!("┗{}┛\n", "━".repeat(padding)); 
        write!(f, "{}", s)
    }
}