use crate::{Board, Color, Piece, Position};

pub fn append_moves(board: &Board, from: Position, dst: &mut Vec<Position>) {
    dst.extend(
        [
            (2, -1),
            (1, -2),
            (-1, -2),
            (-2, -1),
            (-2, 1),
            (-1, 2),
            (1, 2),
            (2, 1),
        ]
        .iter()
        .map(|(file, rank)| Position::new_i8(from.file() as i8 + file, from.rank() as i8 + rank))
        .flatten()
        .filter(|&pos| {
            !matches!(
                board[pos],
                Some(Piece {
                    color: Color::White,
                    ..
                })
            )
        }),
    )
}
