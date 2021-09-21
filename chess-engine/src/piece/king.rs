use crate::{Board, Position};

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
        let pos = match Position::new_i8(from.file() as i8 + x, from.rank() as i8 * y) {
            Some(pos) => pos,
            None => continue,
        };

        let target_color = board[pos].map(|p| p.color);

        if target_color == Some(color) {
            continue;
        }

        dst.push(pos);
    }

    if board.can_castle_queenside(color) {
        dst.push(Position::new_unchecked(from.rank() - 2, from.rank()))
    }

    if board.can_castle_kingside(color) {
        dst.push(Position::new_unchecked(from.rank() + 2, from.rank()))
    }
}
