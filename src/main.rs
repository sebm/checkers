use std::fmt;
// fn draw_board(board) {

// }

fn main() {
    enum BoardSquare {
        Empty,
        ManWhite,
        ManRed,
        // KingWhite,
        // KingRed
    };

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

    let board = [
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


    // for _ in 0..25 {
    //     print!("_");
    // }
    // print!("\n");
    for _ in 0..8 {
        print!("+___");
    }
    print!("+\n");
    for row in board.iter() {
        print!("|");
        for square in row.iter() {
            print!(" {} |", square);
        }
        print!("\n");
        for _ in 0..8 {
            print!("+___");
        }
        print!("+\n");

    }

}
