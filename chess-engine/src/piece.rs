use crate::{Board, Color, Error, Move};

mod pawn;

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
    pub fn can_move(&self, r#move: Move, board: &Board) -> bool {
        match self.kind {
            Kind::Pawn => pawn::can_move(r#move, board),
            Kind::Rook => pawn::can_move(r#move, board),
            Kind::Knight => pawn::can_move(r#move, board),
            Kind::Bishop => pawn::can_move(r#move, board),
            Kind::Queen => pawn::can_move(r#move, board),
            Kind::King => pawn::can_move(r#move, board),
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
