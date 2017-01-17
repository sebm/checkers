use std::io;

mod board;
use board::{BoardSquare, initial_board, draw_board};

fn available_moves(board: &[[BoardSquare; 8]; 8], x: usize, y: usize) {
    let ref piece = board[x][y];
    match *piece {
        BoardSquare::Empty => println!("> nobody there!"),
        BoardSquare::ManWhite => {
            println!("> white man");
        }
        BoardSquare::ManRed => {
            println!("> red man");
            if (y > 0) {
                println!("> gonna check {} {}", x + 1, y - 1);
            }
            if (y < 7) {
                println!("> gonna check {} {}", x + 1, y + 1);
            }
        }

    }
}

fn main() {

    let board = initial_board();

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
        available_moves(&board, x, y);
    }

}
