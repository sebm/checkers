use std::fmt;

enum BoardSquare {
    Empty,
    ManWhite,
    ManRed,
    // KingWhite,
    // KingRed
}

impl fmt::Display for BoardSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match *self {
            BoardSquare::ManWhite => "w",
            BoardSquare::ManRed => "r",
            BoardSquare::Empty => " "
        };

        write!(f, "{}", x)
    }
}

fn draw_board(board: [[BoardSquare; 8]; 8]) {
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

    let board: [[BoardSquare; 8]; 8] = [
        [ BoardSquare::Empty, BoardSquare::ManRed, BoardSquare::Empty, BoardSquare::ManRed, 
            BoardSquare::Empty, BoardSquare::ManRed, BoardSquare::Empty, BoardSquare::ManRed ],
        [ BoardSquare::ManRed, BoardSquare::Empty, BoardSquare::ManRed, BoardSquare::Empty,
            BoardSquare::ManRed, BoardSquare::Empty,BoardSquare::ManRed, BoardSquare::Empty ],
        [ BoardSquare::Empty, BoardSquare::ManRed, BoardSquare::Empty, BoardSquare::ManRed, 
            BoardSquare::Empty, BoardSquare::ManRed, BoardSquare::Empty, BoardSquare::ManRed ],
        [ BoardSquare::Empty, BoardSquare::Empty, BoardSquare::Empty, BoardSquare::Empty, 
            BoardSquare::Empty, BoardSquare::Empty, BoardSquare::Empty, BoardSquare::Empty ],
        [ BoardSquare::Empty, BoardSquare::Empty, BoardSquare::Empty, BoardSquare::Empty, 
            BoardSquare::Empty, BoardSquare::Empty, BoardSquare::Empty, BoardSquare::Empty ],
        [ BoardSquare::ManWhite, BoardSquare::Empty, BoardSquare::ManWhite, BoardSquare::Empty,
            BoardSquare::ManWhite, BoardSquare::Empty,BoardSquare::ManWhite, BoardSquare::Empty ],
        [ BoardSquare::Empty, BoardSquare::ManWhite, BoardSquare::Empty, BoardSquare::ManWhite, 
            BoardSquare::Empty, BoardSquare::ManWhite, BoardSquare::Empty, BoardSquare::ManWhite ],
        [ BoardSquare::ManWhite, BoardSquare::Empty, BoardSquare::ManWhite, BoardSquare::Empty,
            BoardSquare::ManWhite, BoardSquare::Empty,BoardSquare::ManWhite, BoardSquare::Empty ],
    ];

    draw_board(board);

}
