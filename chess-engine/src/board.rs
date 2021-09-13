use crate::{error::IllegalMove, piece, Color, Coordinate, Error, Move, Piece};

#[derive(derive_getters::Getters)]
pub struct Board {
    tiles: [[Option<Piece>; 8]; 8],
    next_move: Color,
    en_passant_square: Option<Coordinate>,
    can_castle_white_kingside: bool,
    can_castle_white_queenside: bool,
    can_castle_black_kingside: bool,
    can_castle_black_queenside: bool,
}

impl Board {
    pub fn make_move<M>(&mut self, _move: M) -> Result<Option<PromotionWanted>, Error>
    where
        M: Into<Move>,
    {
        Err(Error::IllegalMove(IllegalMove::OtherPlayersTurn))
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
            next_move: Color::White,
            en_passant_square: None,
            can_castle_white_kingside: true,
            can_castle_white_queenside: true,
            can_castle_black_kingside: true,
            can_castle_black_queenside: true,
        }
    }
}

#[must_use]
pub struct PromotionWanted<'b> {
    board: &'b mut Board,
    target: Coordinate,
}

impl PromotionWanted<'_> {
    pub fn choose(self, kind: piece::Kind) {
        self.board.tiles[self.target.1 as usize][self.target.0 as usize] = Some(Piece {
            kind,
            color: self.board.next_move,
        })
    }
}
