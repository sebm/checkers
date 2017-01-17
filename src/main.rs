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

fn main() {

    let board = board::one_piece_board();

    draw_board(&board);

    loop {
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
        println!("> legal moves are {:?}", moves);
    }

}
