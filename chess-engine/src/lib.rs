#![deny(warnings)]

mod board;
mod error;
pub mod piece;

pub use board::Board;
pub use error::Error;
pub use piece::Piece;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Color {
    White,
    Black,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Coordinate(u8, u8);

impl From<(u8, u8)> for Coordinate {
    fn from((x, y): (u8, u8)) -> Self {
        Coordinate(x, y)
    }
}

pub struct Move {
    pub from: Coordinate,
    pub to: Coordinate,
}

impl From<(Coordinate, Coordinate)> for Move {
    fn from((from, to): (Coordinate, Coordinate)) -> Self {
        Self { from, to }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut board = Board::default();
        assert!(*board.next_move() == Color::White);
        assert!(matches!(
            board.make_move(Move {
                from: Coordinate(0, 1),
                to: Coordinate(0, 2),
            }),
            Err(Error::IllegalMove(error::IllegalMove::OtherPlayersTurn))
        ));
    }
}
