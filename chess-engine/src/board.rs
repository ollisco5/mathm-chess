use crate::{error::IllegalMove, Color, Error, Move, Piece, Position};

pub struct Board {
    tiles: [[Option<Piece>; 8]; 8],
    next_to_move: Color,
    en_passant_square: Option<Position>,
    can_castle_white_kingside: bool,
    can_castle_white_queenside: bool,
    can_castle_black_kingside: bool,
    can_castle_black_queenside: bool,
}

impl Board {
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
}

impl Default for Board {
    fn default() -> Self {
        Self {
            tiles: [
                [
                    Some(Piece {
                        color: Color::Black,
                        kind: crate::piece::Kind::Rook,
                    }),
                    Some(Piece {
                        color: Color::Black,
                        kind: crate::piece::Kind::Knight,
                    }),
                    Some(Piece {
                        color: Color::Black,
                        kind: crate::piece::Kind::Bishop,
                    }),
                    Some(Piece {
                        color: Color::Black,
                        kind: crate::piece::Kind::Queen,
                    }),
                    Some(Piece {
                        color: Color::Black,
                        kind: crate::piece::Kind::King,
                    }),
                    Some(Piece {
                        color: Color::Black,
                        kind: crate::piece::Kind::Bishop,
                    }),
                    Some(Piece {
                        color: Color::Black,
                        kind: crate::piece::Kind::Knight,
                    }),
                    Some(Piece {
                        color: Color::Black,
                        kind: crate::piece::Kind::Rook,
                    }),
                ],
                [Some(Piece {
                    color: Color::Black,
                    kind: crate::piece::Kind::Pawn,
                }); 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [None; 8],
                [Some(Piece {
                    color: Color::White,
                    kind: crate::piece::Kind::Pawn,
                }); 8],
                [
                    Some(Piece {
                        color: Color::White,
                        kind: crate::piece::Kind::Rook,
                    }),
                    Some(Piece {
                        color: Color::White,
                        kind: crate::piece::Kind::Knight,
                    }),
                    Some(Piece {
                        color: Color::White,
                        kind: crate::piece::Kind::Bishop,
                    }),
                    Some(Piece {
                        color: Color::White,
                        kind: crate::piece::Kind::Queen,
                    }),
                    Some(Piece {
                        color: Color::White,
                        kind: crate::piece::Kind::King,
                    }),
                    Some(Piece {
                        color: Color::White,
                        kind: crate::piece::Kind::Bishop,
                    }),
                    Some(Piece {
                        color: Color::White,
                        kind: crate::piece::Kind::Knight,
                    }),
                    Some(Piece {
                        color: Color::White,
                        kind: crate::piece::Kind::Rook,
                    }),
                ],
            ],
            next_to_move: Color::White,
            en_passant_square: None,
            can_castle_white_kingside: true,
            can_castle_white_queenside: true,
            can_castle_black_kingside: true,
            can_castle_black_queenside: true,
        }
    }
}
