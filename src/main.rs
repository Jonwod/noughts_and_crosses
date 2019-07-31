use std::io;
use rand::Rng;
use std::fmt;
mod vector2;
use vector2::Vector2;


#[derive(Debug, Copy, Clone, PartialEq)]
enum CellValue {
    Empty,
    Nought,
    Cross
}


impl CellValue {
    fn to_char(&self) -> char {
        match self {
            CellValue::Empty => ' ',
            CellValue::Nought => 'o',
            CellValue::Cross => 'x',
        }
    }


    fn player_name(&self) -> String {
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


const COLUMN_LETTERS: [char; 3] = ['A', 'B', 'C'];


/*

 |A|B|C|
1| | | |
2| | | |
3| | | |

*/
fn print_board(board: &[[CellValue; 3]; 3]) {
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
                    // TODO: read board
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


fn process_row_input(row_num: char) -> Option<u8> {
    let as_num = match row_num.to_digit(10) {
        Some(n) => n,
        None => return None,
    };

    if as_num > 0 && as_num <=3 {
        Some((as_num - 1) as u8)
    }
    else {
        None
    }
}


fn process_column_input(letter: char) -> Option<u8> {
    for (i, l) in COLUMN_LETTERS.iter().enumerate() {
        if letter == *l {
            return Some(i as u8)
        }
    }
    None
}


fn process_input(input: &String) -> Option<Vector2<u8>> {
    let cleaned = input.trim().to_uppercase();
    if cleaned.len() == 2 {
        let column_in: char = cleaned.chars().nth(0).unwrap();
        let row_in: char = cleaned.chars().nth(1).unwrap();
        let mut out_coords = Vector2 {x: 0u8, y: 0u8};

        match process_column_input(column_in) {
            Some(x) => out_coords.x = x,
            None => return None
        }

        match process_row_input(row_in) {
            Some(y) => out_coords.y = y,
            None => return None
        }

        return Some(out_coords)
    }
    None
}


// In this first iteration, player is always noughts
fn player_turn(board: &[[CellValue; 3]; 3]) -> Vector2<u8> {
    println!("Your turn.");

    return loop {
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to get input!");
        match process_input(&input) {
            Some(coords) => {
                match board[coords.x as usize][coords.y as usize] {
                    CellValue::Empty => break coords,
                    _ => println!("Cell already occupied. Please select another cell."),
                }
            }
            None => {
                println!("Input format invalid. Please enter in the format 'A1'.");
            }
        }
    }
}


// In this first iteration, player is always crosses
fn ai_turn(board: &[[CellValue; 3]; 3], _token_type: CellValue) -> Vector2<u8> {
    let mut valid_moves: Vec<Vector2<u8>> = Vec::new();
    for u in 0..3 {
        for v in 0..3 {
            match board[u][v] {
                CellValue::Empty => valid_moves.push(Vector2 {x: u as u8, y: v as u8}),
                _ => (),
            }
        }
    }

    if valid_moves.len() == 0 {
        panic!("AI being asked to make a move, but there are no valid moves!");
    }

    let move_index = rand::thread_rng().gen_range(0, valid_moves.len());

    valid_moves[move_index]
}



fn ai_token(player_token: &CellValue) -> CellValue {
    match player_token {
        CellValue::Cross => CellValue::Nought,
        _ => CellValue::Cross,
    }
}


// Returns true if there is a line of the specified CellValue
// type between the specified indices.
fn is_line(board: &[[CellValue; 3]; 3], start: Vector2<u8>, end: Vector2<u8>) -> bool {
    let start_i = Vector2 {x: start.x as usize, y: start.y as usize};
    let end_i = Vector2 {x: end.x as usize, y: end.y as usize};
    let mid_point_coords = {
        let sum = start_i + end_i;
        Vector2 {x: sum.x / 2, y: sum.y / 2}
    };

    let start_value = board[start_i.x][start_i.y];
    if start_value == board[end_i.x][end_i.y]  &&  start_value  ==  board[mid_point_coords.x][mid_point_coords.y] {
        return true;
    }

    false
}


fn winner(board: &[[CellValue; 3]; 3]) -> CellValue {
    for x in 0..3u8 {
        if board[x as usize][0] != CellValue::Empty  &&  is_line(&board, Vector2 {x: x, y: 0}, Vector2 {x: x, y: 2}) {
            return board[x as usize][0];
        }
    }

    for y in 0..3u8 {
        if board[0][y as usize] != CellValue::Empty  && is_line(&board, Vector2 {x: 0, y: y}, Vector2 {x: 2, y: y}) {
            return board[0][y as usize];
        }
    }

    if is_line(&board, Vector2 {x: 0, y: 0}, Vector2 {x: 2, y: 2}) {
        return board[0][0];
    }

    if is_line(&board, Vector2 {x: 2, y: 0}, Vector2 {x: 0, y: 2}) {
        return board[2][0];
    }

    CellValue::Empty
}


fn board_full(board: &[[CellValue; 3]; 3]) -> bool {
    for x in 0..3 {
        for y in 0..3 {
            if board[x][y] == CellValue::Empty {
                return false;
            }
        }
    }

    true
}


fn announce_winner(winner: CellValue, is_human: bool) {
    if is_human {
        println!("You Win! Thanks for playing!");
    } else if winner == CellValue::Empty {
        println!("It's a Draw!");
    } else {
        println!("{} wins.", winner.player_name());
    }
}



fn main() {
    println!("Welcome to Noughts and Crosses!");
    println!("You are playing as crosses.");
    println!("On your turn, just enter the coordinates that you'd like to place a token at.");

    let mut board_state = [ [CellValue::Empty; 3]; 3 ];

    let player_token = CellValue::Cross;

    print_board(&board_state);

    if rand::thread_rng().gen_range(0, 2) == 0 {
        println!("You go first!");
        let player_move = player_turn(&board_state);
        board_state[player_move.x as usize][player_move.y as usize] = player_token;
        print_board(&board_state);
    }
    else {
        println!("The AI goes first.");
    }


    let victor = loop {
        let ai_move = ai_turn(&board_state, ai_token(&player_token));
        board_state[ai_move.x as usize][ai_move.y as usize] = ai_token(&player_token);
        print_board(&board_state);
        // TODO: Remove duplication
        let vic = winner(&board_state);
        if vic != CellValue::Empty {
            break vic;
        }
        else if board_full(&board_state) {
            break CellValue::Empty;
        }

        let player_move = player_turn(&board_state);
        board_state[player_move.x as usize][player_move.y as usize] = player_token;
        print_board(&board_state);
        // TODO: Remove duplication
        let vic = winner(&board_state);
        if vic != CellValue::Empty {
            break vic;
        }
        else if board_full(&board_state) {
            break CellValue::Empty;
        }
    };

    announce_winner(victor, victor == player_token);
}
