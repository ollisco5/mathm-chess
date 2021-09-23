use crate::*;

#[test]
fn arabic_parsing() {
    for (input, output) in [
        ("a4a5", Move::from(((0, 3).into(), (0, 4).into()))),
        ("h5h1", ((7, 4).into(), (7, 0).into()).into()),
        ("a1h8", ((0, 0).into(), (7, 7).into()).into()),
        ("b5a7", ((1, 4).into(), (0, 6).into()).into()),
    ] {
        assert_eq!(Move::arabic(input), Ok(output));
    }
}

#[test]
fn arabic_parsing_fails() {
    assert!(matches!(Move::arabic("a4a"), Err(Error::ParsingError),));
    assert!(matches!(Move::arabic("i2a3"), Err(Error::ParsingError),));
    assert!(matches!(Move::arabic("a2u3"), Err(Error::ParsingError),));
    assert!(matches!(Move::arabic("a4a4 "), Err(Error::ParsingError),));
}

#[test]
fn piece_parsing() {
    for color in [Color::White, Color::Black] {
        use piece::Kind::*;
        for (c, kind) in [('p', Pawn), ('r', Rook), ('b', Bishop)] {
            assert_eq!(
                Piece::from_name(if color == Color::Black {
                    c
                } else {
                    c.to_ascii_uppercase()
                }),
                Ok(Piece::new(color, kind)),
            );
        }
    }
}

#[test]
fn piece_parsing_fail() {
    for c in "acdefghijlmostuvwxyzACDEFGHIJLMOSTUVWXYZ".chars() {
        assert!(Piece::from_name(c).is_err())
    }
}

#[test]
fn default_board() {
    assert_eq!(
        Board::default(),
        Board {
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
            can_castle_white_kingside: true,
            can_castle_white_queenside: true,
            can_castle_black_kingside: true,
            can_castle_black_queenside: true,
            en_passant_square: None,
            halfmove_counter: 0,
            move_number: 1,
        }
    );
}
