use std::{fmt, ops};

use crate::{Color, Piece, Position};

mod fen;

#[derive(Debug, PartialEq, Eq)]
pub struct Board {
    tiles: [[Option<Piece>; 8]; 8],
    next_to_move: Color,
    can_castle_white_kingside: bool,
    can_castle_white_queenside: bool,
    can_castle_black_kingside: bool,
    can_castle_black_queenside: bool,
    en_passant_square: Option<Position>,
    halfmove_counter: u16,
    move_number: u16,
}

impl Board {
    pub fn new(
        tiles: [[Option<Piece>; 8]; 8],
        next_to_move: Color,
        can_castle_white_kingside: bool,
        can_castle_white_queenside: bool,
        can_castle_black_kingside: bool,
        can_castle_black_queenside: bool,
        en_passant_square: Option<Position>,
        halfmove_counter: u16,
        move_number: u16,
    ) -> Self {
        Self {
            tiles,
            next_to_move,
            can_castle_white_kingside,
            can_castle_white_queenside,
            can_castle_black_kingside,
            can_castle_black_queenside,
            en_passant_square,
            halfmove_counter,
            move_number,
        }
    }
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
    pub fn can_castle_white_kingside(&self) -> bool {
        self.can_castle_white_kingside
    }
    pub fn can_castle_white_queenside(&self) -> bool {
        self.can_castle_white_queenside
    }
    pub fn can_castle_black_kingside(&self) -> bool {
        self.can_castle_black_kingside
    }
    pub fn can_castle_black_queenside(&self) -> bool {
        self.can_castle_black_queenside
    }
    pub fn cannot_castle_white_kingside(&mut self) {
        self.can_castle_white_kingside = false;
    }
    pub fn cannot_castle_white_queenside(&mut self) {
        self.can_castle_white_queenside = false;
    }
    pub fn cannot_castle_black_kingside(&mut self) {
        self.can_castle_black_kingside = false;
    }
    pub fn cannot_castle_black_queenside(&mut self) {
        self.can_castle_black_queenside = false;
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
        write!(
            f,
            "{}  A B C D E F G H\n",
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
