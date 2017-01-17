use std::{fmt, io};

enum BoardSquare {
    Empty,
    ManWhite,
    ManRed, /* KingWhite,
             * KingRed */
}

impl fmt::Display for BoardSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match *self {
            BoardSquare::ManWhite => "w",
            BoardSquare::ManRed => "r",
            BoardSquare::Empty => " ",
        };

        write!(f, "{}", x)
    }
}

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

fn draw_board(board: &[[BoardSquare; 8]; 8]) {
    print!("╔");
    for _ in 0..7 {
        print!("═══╦");
    }
    print!("═══╗\n");
    for row in board.iter() {
        for square in row.iter() {
            print!("║ {} ", square);
        }
        print!("║\n");
    }
    print!("╚");
    for _ in 0..7 {
        print!("═══╩");
    }
    print!("═══╝\n");

}

fn main() {

    let board = [[BoardSquare::Empty,
                  BoardSquare::ManRed,
                  BoardSquare::Empty,
                  BoardSquare::ManRed,
                  BoardSquare::Empty,
                  BoardSquare::ManRed,
                  BoardSquare::Empty,
                  BoardSquare::ManRed],
                 [BoardSquare::ManRed,
                  BoardSquare::Empty,
                  BoardSquare::ManRed,
                  BoardSquare::Empty,
                  BoardSquare::ManRed,
                  BoardSquare::Empty,
                  BoardSquare::ManRed,
                  BoardSquare::Empty],
                 [BoardSquare::Empty,
                  BoardSquare::ManRed,
                  BoardSquare::Empty,
                  BoardSquare::ManRed,
                  BoardSquare::Empty,
                  BoardSquare::ManRed,
                  BoardSquare::Empty,
                  BoardSquare::ManRed],
                 [BoardSquare::Empty,
                  BoardSquare::Empty,
                  BoardSquare::Empty,
                  BoardSquare::Empty,
                  BoardSquare::Empty,
                  BoardSquare::Empty,
                  BoardSquare::Empty,
                  BoardSquare::Empty],
                 [BoardSquare::Empty,
                  BoardSquare::Empty,
                  BoardSquare::Empty,
                  BoardSquare::Empty,
                  BoardSquare::Empty,
                  BoardSquare::Empty,
                  BoardSquare::Empty,
                  BoardSquare::Empty],
                 [BoardSquare::ManWhite,
                  BoardSquare::Empty,
                  BoardSquare::ManWhite,
                  BoardSquare::Empty,
                  BoardSquare::ManWhite,
                  BoardSquare::Empty,
                  BoardSquare::ManWhite,
                  BoardSquare::Empty],
                 [BoardSquare::Empty,
                  BoardSquare::ManWhite,
                  BoardSquare::Empty,
                  BoardSquare::ManWhite,
                  BoardSquare::Empty,
                  BoardSquare::ManWhite,
                  BoardSquare::Empty,
                  BoardSquare::ManWhite],
                 [BoardSquare::ManWhite,
                  BoardSquare::Empty,
                  BoardSquare::ManWhite,
                  BoardSquare::Empty,
                  BoardSquare::ManWhite,
                  BoardSquare::Empty,
                  BoardSquare::ManWhite,
                  BoardSquare::Empty]];

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
