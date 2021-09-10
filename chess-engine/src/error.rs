use std::error::Error as StdError;
use std::fmt;

#[derive(Debug)]
pub enum Error {
    IllegalMove(IllegalMove),
}

#[derive(Debug)]
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
