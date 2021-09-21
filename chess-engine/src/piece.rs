use crate::{Board, Color, Error, Move, Position};

mod bishop;
mod king;
mod knight;
mod pawn;
mod queen;
mod rook;
mod util;

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

impl Piece {
    pub fn new(color: Color, kind: Kind) -> Self {
        Self { color, kind }
    }
    pub fn emoji(&self) -> char {
        use Color::*;
        use Kind::*;
        match (self.color, self.kind) {
            (Black, Pawn) => '♟',
            (White, Pawn) => '♙',
            (Black, Rook) => '♜',
            (White, Rook) => '♖',
            (Black, Knight) => '♞',
            (White, Knight) => '♘',
            (Black, Bishop) => '♝',
            (White, Bishop) => '♗',
            (Black, Queen) => '♛',
            (White, Queen) => '♕',
            (Black, King) => '♚',
            (White, King) => '♔',
        }
    }
    pub fn from_name(name: char) -> Result<Self, Error> {
        Ok(Piece {
            color: if name.is_ascii_uppercase() {
                Color::White
            } else {
                Color::Black
            },
            kind: Kind::from_name(name)?,
        })
    }
    pub fn can_move(&self, move_: Move, board: &Board) -> bool {
        self.get_moves(board, move_.from).contains(&move_.to)
    }
    pub fn get_moves(&self, board: &Board, from: Position) -> Vec<Position> {
        let mut ret = vec![];
        self.append_moves(board, from, &mut ret);
        ret
    }
    pub fn append_moves(&self, board: &Board, from: Position, dst: &mut Vec<Position>) {
        match self.kind {
            Kind::Pawn => pawn::append_moves(board, from, dst),
            Kind::Rook => rook::append_moves(board, from, dst),
            Kind::Knight => knight::append_moves(board, from, dst),
            Kind::Bishop => bishop::append_moves(board, from, dst),
            Kind::Queen => queen::append_moves(board, from, dst),
            Kind::King => king::append_moves(board, from, dst),
        }
    }
}

impl Kind {
    pub fn from_name(name: char) -> Result<Self, Error> {
        match name {
            'p' | 'P' => Ok(Self::Pawn),
            'r' | 'R' => Ok(Self::Rook),
            'n' | 'N' => Ok(Self::Knight),
            'b' | 'B' => Ok(Self::Bishop),
            'q' | 'Q' => Ok(Self::Queen),
            'k' | 'K' => Ok(Self::King),
            _ => Err(Error::ParsingError),
        }
    }
}
