use std::fmt;
pub enum BoardSquare {
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
}