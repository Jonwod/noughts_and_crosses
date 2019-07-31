use std::fmt;


#[derive(Debug, Copy, Clone, PartialEq)]
pub enum CellValue {
    Empty,
    Nought,
    Cross
}


impl CellValue {
    pub fn to_char(&self) -> char {
        match self {
            CellValue::Empty => ' ',
            CellValue::Nought => 'o',
            CellValue::Cross => 'x',
        }
    }


    pub fn player_name(&self) -> String {
        match self {
            CellValue::Empty => String::from("None"),
            CellValue::Nought => String::from("Noughts"),
            CellValue::Cross => String::from("Crosses"),
        }
    }
}


impl fmt::Display for CellValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_char())
    }
}



pub fn board_full(board: &[[CellValue; 3]; 3]) -> bool {
    for x in 0..3 {
        for y in 0..3 {
            if board[x][y] == CellValue::Empty {
                return false;
            }
        }
    }

    true
}


pub const COLUMN_LETTERS: [char; 3] = ['A', 'B', 'C'];


/*

 |A|B|C|
1| | | |
2| | | |
3| | | |

*/
pub fn print_board(board: &[[CellValue; 3]; 3]) {
    println!("");

    for y in 0..4 {
        let mut row = String::new();
        let left_margin: char = if y == 0 {' '}
        else {
            std::char::from_digit(y, 10).expect("print_board() does not yet support grid size larger than 9")
        };

        row.push(left_margin);

        for x in 1..8 {
            if x % 2 == 0 {
                let x_coord: usize = (x/2) - 1;
                if y == 0 {
                    row.push(COLUMN_LETTERS[x_coord]);
                }
                else {
                    let y_coord: usize = y as usize - 1;
                    row.push(board[x_coord][y_coord].to_char());
                }
            } else {
                row.push('|');
            }
        }
        println!("{}", row);
    }

    println!("");
}