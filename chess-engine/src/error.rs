use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    IllegalMove(IllegalMove),
    UnknwonPiece(char),
    InvalidNotation { pos: usize, expected: String },
}

#[derive(Debug, PartialEq, Eq)]
pub enum IllegalMove {
    OtherPlayersTurn,
    NoPieceToMove,
    WouldPutSelfInCheck,
    DisallowedMovement,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::IllegalMove(i) => write!(f, "Illegal move, {}", i),
            Self::UnknwonPiece(c) => write!(f, "Unknown piece {}", c),
            Self::InvalidNotation { pos, expected } => write!(
                f,
                "Invalid notation at position {}, expected {}",
                pos, expected
            ),
        }
    }
}

impl fmt::Display for IllegalMove {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OtherPlayersTurn => write!(f, "other players turn"),
            Self::NoPieceToMove => write!(f, "no piece to move"),
            Self::WouldPutSelfInCheck => write!(f, "would put own king in check"),
            Self::DisallowedMovement => write!(f, "disallowed movement"),
        }
    }
}
