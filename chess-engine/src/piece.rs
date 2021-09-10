use crate::Color;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct Piece {
    pub color: Color,
    pub kind: Kind,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Kind {
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}
