use std::{fmt, ops};

use crate::{Color, Piece, Position};

mod fen;

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
}

impl Board {
    pub fn tiles(&self) -> &[[Option<Piece>; 8]; 8] {
        &self.tiles
    }
    pub fn next_to_move(&self) -> Color {
        self.next_to_move
    }
    pub fn switch_next_to_move(&mut self) {
        self.next_to_move = self.next_to_move.other();
    }
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
    pub fn cannot_castle_kingside(&mut self, color: Color) {
        match color {
            Color::White => self.can_castle_white_kingside = false,
            Color::Black => self.can_castle_black_kingside = false,
        }
    }
    pub fn cannot_castle_queenside(&mut self, color: Color) {
        match color {
            Color::White => self.can_castle_white_queenside = false,
            Color::Black => self.can_castle_black_queenside = false,
        }
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
