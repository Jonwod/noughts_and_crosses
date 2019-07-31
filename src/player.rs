use super::board::*;
use super::vector2::*;
use rand::Rng;
use std::io;


pub enum PlayerType {
    Human,
    AI,
}


pub struct Player {
    pub name: String,
    pub token: CellValue,
    pub controller: PlayerType,
}


impl Player {
    pub fn decide_move(&self, board: &[[CellValue; 3]; 3]) -> Vector2<u8> {
        match self.controller {
            PlayerType::Human => {
                self.player_turn(board)
            },
            PlayerType::AI => {
                self.ai_turn(board)
            },
        }
    }


    fn player_turn(&self, board: &[[CellValue; 3]; 3]) -> Vector2<u8> {
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


    fn ai_turn(&self, board: &[[CellValue; 3]; 3]) -> Vector2<u8> {
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