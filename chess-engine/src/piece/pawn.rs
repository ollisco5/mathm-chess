use crate::{Board, Color, Position};

use super::util::threatened_at;

pub fn checks(at: Position, color: Color, board: &Board) -> bool {
    [
        Position::new_i8(at.file() as i8 - 1, at.rank() as i8 + color.forwards()),
        Position::new_i8(at.file() as i8 + 1, at.rank() as i8 + color.forwards()),
    ]
    .iter()
    .flatten()
    .any(|pos| *pos == board.get_king_position(color))
}

pub fn append_moves(board: &Board, from: Position, dst: &mut Vec<Position>) {
    let color = board[from].unwrap().color;
    let forwards = color.forwards();

    if let Some(pos) = Position::new(from.file(), (from.rank() as i8 + forwards) as u8) {
        if board[pos].is_none()
            && !threatened_at(
                board.get_king_position(color),
                &[from],
                &[pos],
                color,
                board,
            )
        {
            dst.push(pos);
        }
    }

    if from.rank() == 6 && color == Color::White || from.rank() == 1 && color == Color::Black {
        if let Some(pos) = Position::new(from.file(), (from.rank() as i8 + forwards * 2) as u8) {
            if board[Position::new_i8_unchecked(from.file() as i8, from.rank() as i8 + forwards)]
                .is_none()
                && board[pos].is_none()
                && !threatened_at(
                    board.get_king_position(color),
                    &[from],
                    &[pos],
                    color,
                    board,
                )
            {
                dst.push(pos);
            }
        }
    }

    for x in [-1, 1] {
        if let Some(pos) = Position::new_i8(from.file() as i8 + x, from.rank() as i8 + forwards) {
            if board[pos].map(|p| p.color) == Some(color.other())
                && !threatened_at(
                    board.get_king_position(color),
                    &[from],
                    &[pos],
                    color,
                    board,
                )
            {
                dst.push(pos);
            }
            if Some(pos) == board.en_passant_square() {
                let pawn_pos =
                    Position::new_i8_unchecked(pos.file() as i8, pos.rank() as i8 - forwards);
                if !threatened_at(
                    board.get_king_position(color),
                    &[from, pawn_pos],
                    &[pos],
                    color,
                    board,
                ) {
                    dst.push(pos);
                }
            }
        }
    }
}
