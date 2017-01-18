use std::fmt;

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum BoardSquare {
    Empty,
    ManWhite,
    ManRed, 
    KingWhite,
    KingRed
}

impl fmt::Display for BoardSquare {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let x = match *self {
            BoardSquare::ManWhite => "w",
            BoardSquare::ManRed => "r",
            BoardSquare::KingWhite => "W",
            BoardSquare::KingRed => "R",
            BoardSquare::Empty => " ",
        };

        write!(f, "{}", x)
    }
}

pub fn initial_board() -> [[BoardSquare; 8]; 8] {
    return [[BoardSquare::Empty,
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
            [BoardSquare::Empty; 8],
            [BoardSquare::Empty; 8],
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
}

pub fn one_piece_board() -> [[BoardSquare; 8]; 8] {
    let mut board = [[BoardSquare::Empty; 8]; 8];

    board[6][6] = BoardSquare::ManRed;

    return board;
}

pub fn draw_board(board: &[[BoardSquare; 8]; 8]) {
    print!("    ");
    for i in 0..8 {
        print!("{}   ", i);
    }

    print!("\n  ╔");
    for _ in 0..7 {
        print!("═══╦");
    }
    print!("═══╗\n");
    for (i, row) in board.iter().enumerate() {
        print!("{} ", i);
        for square in row.iter() {
            print!("║ {} ", square);
        }
        print!("║{}\n", i);
    }
    print!("  ╚");
    for _ in 0..7 {
        print!("═══╩");
    }
    print!("═══╝\n");
}
