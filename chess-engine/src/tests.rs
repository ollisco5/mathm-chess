use crate::*;

#[test]
fn it_works() {
    let mut board = Board::default();
    assert!(board.next_to_move() == Color::White);
    assert!(matches!(
        board.make_move(Move {
            from: (0, 1).into(),
            to: (0, 2).into(),
        }),
        Err(Error::IllegalMove(error::IllegalMove::OtherPlayersTurn))
    ));
}

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
