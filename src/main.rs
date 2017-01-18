use std::io;

mod board;
use board::{BoardSquare, initial_board, draw_board};

fn available_moves(board: &[[BoardSquare; 8]; 8], x: usize, y: usize) -> Vec<(usize, usize)> {
    let mut moves = Vec::new();
    let ref piece = board[x][y];
    match *piece {
        BoardSquare::Empty => return moves,
        BoardSquare::ManWhite => {
            if y > 0 {
                match board[x-1][y-1] {
                    BoardSquare::Empty => {
                        moves.push((x-1, y-1))
                    },
                    _ => {
                        // TODO implement later
                    }
                }
            }
            if y < 7 {
                match board[x-1][y+1] {
                    BoardSquare::Empty => {
                        moves.push((x-1, y+1));
                    },
                    _ => {
                        // TODO implement later
                    }
                }
            }
        }
        BoardSquare::ManRed => {
            if y > 0 {
                match board[x+1][y-1] {
                    BoardSquare::Empty => {
                        moves.push((x+1, y-1));
                    },
                    _ => {
                        // TODO implement later
                    }
                }
            }
            if y < 7 {
                match board[x+1][y+1] {
                    BoardSquare::Empty => {
                        moves.push((x+1, y+1));
                    },
                    _ => {
                        // TODO implement later
                    }
                }
            }
        }

    }

    return moves;
}

fn update_board(board: &mut [[BoardSquare; 8]; 8], start_coords: (usize, usize), end_coords: (usize, usize) ) {
    let start_piece = board[start_coords.0][start_coords.1];
    board[start_coords.0][start_coords.1] = BoardSquare::Empty;
    board[end_coords.0][end_coords.1] = start_piece;
}

fn main() {

    let mut board = board::one_piece_board();

    loop {

        draw_board(&board);

        let mut who_to_move = String::new();
        println!("> who do you want to move?");
        io::stdin()
            .read_line(&mut who_to_move)
            .expect("Failed to read line");

        let coordinates = who_to_move.split_whitespace()
            .map(|x| x.parse::<usize>().expect("should be an int"))
            .collect::<Vec<usize>>();

        let x = coordinates[0];
        let y = coordinates[1];

        println!("> who's at {} {}?", x, y);
        let moves = available_moves(&board, x, y);
        if (moves.len() == 0) {
            println!("> no legal moves!");
            continue;
        }
        println!("> legal moves are {:?}", moves);
        println!("> which move are you gonna make?");
        for (i, item) in moves.iter().enumerate() {
            println!("{}> {:?}", i, item);
        }
        let mut where_to_move = String::new();
        io::stdin()
            .read_line(&mut where_to_move)
            .expect("failed to read line");

        let move_index = where_to_move.trim().parse::<usize>().expect("should be int");
        println!("you picked {:?}!", moves[move_index]);

        update_board(&mut board, (x, y), moves[move_index]);
    }

}
