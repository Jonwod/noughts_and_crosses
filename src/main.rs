use std::io;
use rand::Rng;
mod vector2;
use vector2::Vector2;
mod board;
use board::*;
mod player;
use player::*;


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


fn announce_winner(winner: Option<&Player>) {
    match winner {
        Some(w) => {
            println!("{} wins.", w.name);
            if let PlayerType::Human = w.controller {
                println!("Congratulations!");
            }
        },
        None => println!("It's a draw!"),
    }
}



fn main() {
    println!("Welcome to Noughts and Crosses!");
    println!("You are playing as crosses.");
    println!("On your turn, just enter the coordinates that you'd like to place a token at.");

    let mut board_state = [ [CellValue::Empty; 3]; 3 ];

    let player_token = CellValue::Cross;

    let players = [
        Player{name: String::from("Player 1"), token: player_token, controller: PlayerType::Human},
        Player{name: String::from("Computer"), token: player_token.opposite(), controller: PlayerType::AI},
    ];

    print_board(&board_state);

    let mut current_turn = match rand::thread_rng().gen_range(0, 2) == 0 {
        false =>{
            println!("You go first!");
            0 as usize
        },
        true => {
            println!("The AI goes first.");
            1 as usize
        },
    };


    let victor = loop {
        let mov = players[current_turn].decide_move(&board_state);

        board_state[mov.x as usize][mov.y as usize] = players[current_turn].token;
        print_board(&board_state);

        if winner(&board_state) != CellValue::Empty {
            break Option::Some(&players[current_turn]);
        }
        else if board_full(&board_state) {
            break None;
        }

        current_turn = if current_turn == 0 { 1 } else { 0 };
    };

    announce_winner(victor);
}
