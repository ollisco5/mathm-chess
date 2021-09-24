//! For example uses, see `Decider`, and `Game`

#![deny(warnings)]

mod board;
mod decider;
mod error;
mod game;
pub mod piece;
mod util;

pub use board::Board;
pub use decider::Decider;
pub use error::Error;
pub use game::{Game, GameState};
pub use piece::Piece;
pub use util::{Color, Move, Position};

#[cfg(test)]
mod tests;
