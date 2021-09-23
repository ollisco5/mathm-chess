use crate::{Board, Color, Position};

use super::util::threatened_at;

pub fn checks(_at: Position, _color: Color, _board: &Board) -> bool {
    false
}

pub fn append_moves(board: &Board, from: Position, dst: &mut Vec<Position>) {
    let color = board[from].unwrap().color;
    for (x, y) in [
        (1, 0),
        (1, -1),
        (0, -1),
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ] {
        let pos = match Position::new_i8(from.file() as i8 + x, from.rank() as i8 + y) {
            Some(pos) => pos,
            None => continue,
        };

        if board[pos].map(|p| p.color) == Some(color) {
            continue;
        }

        if threatened_at(pos, &[from], &[], color, board) {
            continue;
        }

        dst.push(pos);
    }

    if board.can_castle_queenside(color) {
        assert_eq!(from.file(), 4);
    }

    if board.can_castle_queenside(color)
        && board[Position::new_unchecked(from.file() - 3, from.rank())].is_none()
        && !threatened_at(from, &[], &[], color, board)
        && [1, 2]
            .iter()
            .map(|x| Position::new_unchecked(from.file() - x, from.rank()))
            .all(|pos| board[pos].is_none() && !threatened_at(pos, &[], &[], color, board))
    {
        dst.push(Position::new_unchecked(from.file() - 2, from.rank()))
    }

    if board.can_castle_kingside(color)
        && !threatened_at(from, &[], &[], color, board)
        && [1, 2]
            .iter()
            .map(|x| Position::new_unchecked(from.file() + x, from.rank()))
            .all(|pos| board[pos].is_none() && !threatened_at(pos, &[], &[], color, board))
    {
        dst.push(Position::new_unchecked(from.file() + 2, from.rank()))
    }
}
