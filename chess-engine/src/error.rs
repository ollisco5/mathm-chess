use std::error::Error as StdError;
use std::fmt;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    OtherPlayersTurn,
    NoPieceToMove,
    IllegalMove,
    UnknwonPiece(char),
    ParsingError,
    InvalidGameState,
}

impl StdError for Error {
    fn source(&self) -> Option<&(dyn StdError + 'static)> {
        None
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::OtherPlayersTurn => write!(f, "Other players turn"),
            Self::NoPieceToMove => write!(f, "No piece to move"),
            Self::IllegalMove => write!(f, "Illegal move"),
            Self::UnknwonPiece(c) => write!(f, "Unknown piece {}", c),
            Self::ParsingError => write!(f, "Parsing error"),
            Self::InvalidGameState => write!(f, "Invalid game state"),
        }
    }
}
