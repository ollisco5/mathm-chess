use std::fmt;
use std::str::FromStr;

use crate::{error::IllegalMove, Color, Error, Move, Piece, Position};

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
    pub fn make_move<M>(&mut self, _move: M) -> Result<bool, Error>
    where
        M: Into<Move>,
    {
        Err(Error::IllegalMove(IllegalMove::OtherPlayersTurn))
    }
    pub fn tiles(&self) -> &[[Option<Piece>; 8]; 8] {
        &self.tiles
    }
    pub fn next_to_move(&self) -> Color {
        self.next_to_move
    }
    pub fn en_passant_square(&self) -> Option<Position> {
        self.en_passant_square
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
    pub fn from_fen(fen: &str) -> Result<Self, Error> {
        let mut fen = fen.split_ascii_whitespace();

        let mut tiles = [[None; 8]; 8];

        let mut file = 0;
        let mut rank = 0;

        let tiles_part = fen.next().ok_or(Error::ParsingError)?;
        for c in tiles_part.chars() {
            match c {
                '/' => {
                    rank += 1;
                    file = 0;
                }
                '1'..='8' => {
                    file += c as usize - '0' as usize;
                }
                _ => {
                    tiles[rank][file] = Some(Piece::from_name(c)?);
                    file += 1;
                }
            }
        }

        let mut board = Board {
            tiles,
            next_to_move: Color::White,
            can_castle_white_kingside: true,
            can_castle_white_queenside: true,
            can_castle_black_kingside: true,
            can_castle_black_queenside: true,
            en_passant_square: None,
            halfmove_counter: 0,
            move_number: 0,
        };

        let next_to_move_part = fen.next().ok_or(Error::ParsingError)?;
        board.next_to_move = match next_to_move_part {
            "w" => Color::White,
            "b" => Color::Black,
            _ => return Err(Error::ParsingError),
        };

        let castling_part = fen.next().ok_or(Error::ParsingError)?;
        for c in castling_part.chars() {
            match c {
                'K' => board.can_castle_white_kingside = true,
                'Q' => board.can_castle_white_queenside = true,
                'k' => board.can_castle_black_kingside = true,
                'q' => board.can_castle_black_queenside = true,
                _ => return Err(Error::ParsingError),
            }
        }

        let en_passant_square_part = fen.next().ok_or(Error::ParsingError)?;
        board.en_passant_square = match en_passant_square_part {
            "-" => None,
            ep => match Position::from_str(ep) {
                Ok(p) => Some(p),
                Err(err) => return Err(err),
            },
        };

        let halfmove_counter_part = fen.next().ok_or(Error::ParsingError)?;
        board.halfmove_counter = halfmove_counter_part
            .parse()
            .map_err(|_| Error::ParsingError)?;

        let move_number_part = fen.next().ok_or(Error::ParsingError)?;
        board.move_number = move_number_part.parse().map_err(|_| Error::ParsingError)?;

        Ok(board)
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            Board::default()
                .tiles()
                .iter()
                .map(|row| format!(
                    "{}\n",
                    row.iter()
                        .map(|p| p.as_ref().map(Piece::emoji).unwrap_or(' '))
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
