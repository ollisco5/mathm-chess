use std::{fmt, ops};

use crate::{piece, Color, Piece, Position};

mod fen;

/// Represents the state of a chess board.
///
/// Note: the `Board` must always represent a valid state. Some methods might
/// panic if the is not the case.
#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    pub(crate) tiles: [[Option<Piece>; 8]; 8],
    pub(crate) next_to_move: Color,
    pub(crate) can_castle_white_kingside: bool,
    pub(crate) can_castle_white_queenside: bool,
    pub(crate) can_castle_black_kingside: bool,
    pub(crate) can_castle_black_queenside: bool,
    pub(crate) en_passant_square: Option<Position>,
    pub(crate) halfmove_counter: u16,
    pub(crate) move_number: u16,
    pub(crate) checking_pieces: u8,
}

impl Board {
    pub fn tiles(&self) -> &[[Option<Piece>; 8]; 8] {
        &self.tiles
    }
    /// Signifies wich color in next up to make a move. Starts as `Color::White`
    /// on a `Default` board
    pub fn next_to_move(&self) -> Color {
        self.next_to_move
    }
    /// Sets `next_to_move` to the other color and increments `move_number` and
    /// `halfmove_counter` if `next_to_move` was black before call
    pub fn switch_next_to_move(&mut self) {
        if self.next_to_move() == Color::Black {
            self.move_number += 1;
        }
        self.next_to_move = self.next_to_move.other();
    }
    /// Retrieves the tile where a pawn can move to capture another pawn that
    /// just moved two ranks
    pub fn en_passant_square(&self) -> Option<Position> {
        self.en_passant_square
    }
    pub fn set_en_passant_square(&mut self, eps: Option<Position>) {
        self.en_passant_square = eps;
    }
    pub fn can_castle_kingside(&self, color: Color) -> bool {
        match color {
            Color::White => self.can_castle_white_kingside,
            Color::Black => self.can_castle_black_kingside,
        }
    }
    pub fn can_castle_queenside(&self, color: Color) -> bool {
        match color {
            Color::White => self.can_castle_white_queenside,
            Color::Black => self.can_castle_black_queenside,
        }
    }
    /// Marks that `color` can no longer castle on the kingside. Can be called
    /// even if it was not possible before calling (but will have no effect)
    pub fn cannot_castle_kingside(&mut self, color: Color) {
        match color {
            Color::White => self.can_castle_white_kingside = false,
            Color::Black => self.can_castle_black_kingside = false,
        }
    }
    /// Marks that `color` can no longer castle on the queenside. Can be called
    /// even if it was not possible before calling (but will have no effect)
    pub fn cannot_castle_queenside(&mut self, color: Color) {
        match color {
            Color::White => self.can_castle_white_queenside = false,
            Color::Black => self.can_castle_black_queenside = false,
        }
    }
    /// Sets the halvmove counter to zero
    pub fn reset_halvmove_counter(&mut self) {
        self.halfmove_counter = 0;
    }
    pub fn in_check(&self) -> bool {
        self.checking_pieces > 0
    }
    pub fn in_double_check(&self) -> bool {
        self.checking_pieces > 1
    }
    /// Returns the position of the king with the color `color`.
    pub fn get_king_position(&self, color: Color) -> Position {
        let mut pos = Position::new_unchecked(0, 0);
        while self[pos]
            != Some(Piece {
                color,
                kind: piece::Kind::King,
            })
        {
            if pos.file() == 7 {
                pos = Position::new_unchecked(0, pos.rank() + 1)
            } else {
                pos = Position::new_unchecked(pos.file() + 1, pos.rank())
            }
        }
        pos
    }
}

impl ops::Index<Position> for Board {
    type Output = Option<Piece>;
    fn index(&self, p: Position) -> &Self::Output {
        &self.tiles[p.rank() as usize][p.file() as usize]
    }
}

impl ops::IndexMut<Position> for Board {
    fn index_mut(&mut self, p: Position) -> &mut Self::Output {
        &mut self.tiles[p.rank() as usize][p.file() as usize]
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "{}  A B C D E F G H",
            self.tiles()
                .iter()
                .enumerate()
                .map(|(i, row)| format!(
                    "{}{}\n",
                    i + 1,
                    row.iter()
                        .map(|p| format!(" {}", p.as_ref().map(Piece::emoji).unwrap_or('.')))
                        .collect::<String>()
                ))
                .collect::<String>()
        )
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::from_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1").unwrap()
    }
}
