use crate::{Board, Color, Position};

use super::Kind;

/// Indicates if a piece at `position` with color `color` can be captured.
///
/// Note: Ignores en passant rules.
///
/// If `ignore_piece_at` is `Some`, that square will be treated as empty.
///
/// If `treat_pos_as_wall` is `Some`, a capture that require moving over that
/// square wont count.
///
/// Also, the piece in question does not have to be at `position` in `board`.
pub fn threatened_at(
    position: Position,
    treat_as_empty: &[Position],
    treat_as_occupied: &[Position],
    color: Color,
    board: &Board,
) -> bool {
    for (x, y) in [
        (2, -1),
        (1, -2),
        (-1, -2),
        (-2, -1),
        (-2, 1),
        (-1, 2),
        (1, 2),
        (2, 1),
    ] {
        let pos = match Position::new_i8(position.file() as i8 + x, position.rank() as i8 + y) {
            Some(pos) => pos,
            None => continue,
        };
        if treat_as_empty.contains(&pos) {
            continue;
        }
        if board[pos].map_or(false, |piece| {
            piece.kind == Kind::Knight && piece.color == color.other()
        }) {
            return true;
        }
    }
    for (x, y, k) in [
        (0, 1, Kind::Rook),
        (1, 0, Kind::Rook),
        (0, -1, Kind::Rook),
        (-1, 0, Kind::Rook),
        (1, 1, Kind::Bishop),
        (1, -1, Kind::Bishop),
        (-1, -1, Kind::Bishop),
        (-1, 1, Kind::Bishop),
    ] {
        for i in 1..8 {
            let pos = match Position::new_i8(
                position.file() as i8 + i * x,
                position.rank() as i8 + i * y,
            ) {
                Some(pos) => pos,
                None => break,
            };
            if treat_as_empty.contains(&pos) {
                continue;
            }
            if treat_as_occupied.contains(&pos) {
                break;
            }
            if let Some(piece) = board[pos] {
                if piece.color == color {
                    break;
                }
                if piece.kind == k || piece.kind == Kind::Queen {
                    return true;
                }
            }
        }
    }
    for (x, y) in [(-1, color.forwards()), (1, color.forwards())] {
        let pos = match Position::new_i8(position.file() as i8 + x, position.rank() as i8 + y) {
            Some(pos) => pos,
            None => continue,
        };
        if treat_as_empty.contains(&pos) {
            continue;
        }
        if board[pos].map_or(false, |piece| {
            piece.kind == Kind::Pawn && piece.color == color.other()
        }) {
            return true;
        }
    }
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
        let pos = match Position::new_i8(position.file() as i8 + x, position.rank() as i8 + y) {
            Some(pos) => pos,
            None => continue,
        };
        if treat_as_empty.contains(&pos) {
            continue;
        }
        if board[pos].map_or(false, |piece| {
            piece.kind == Kind::King && piece.color == color.other()
        }) {
            return true;
        }
    }
    false
}

pub fn floating_checks(deltas: &[(i8, i8)], at: Position, color: Color, board: &Board) -> bool {
    for (x, y) in deltas {
        for i in 1..8 {
            let pos = match Position::new_i8(at.file() as i8 + i * x, at.rank() as i8 + i * y) {
                Some(pos) => pos,
                None => break,
            };

            if let Some(piece) = board[pos] {
                if piece.color == color.other() && piece.kind == Kind::King {
                    return true;
                } else {
                    break;
                }
            }
        }
    }
    false
}

pub fn floating_moves(deltas: &[(i8, i8)], board: &Board, from: Position, dst: &mut Vec<Position>) {
    let color = board[from].unwrap().color;
    for (x, y) in deltas {
        for i in 1..8 {
            let pos = match Position::new_i8(from.file() as i8 + i * x, from.rank() as i8 + i * y) {
                Some(pos) => pos,
                None => break,
            };

            let target_color = board[pos].map(|p| p.color);

            if target_color == Some(color) {
                break;
            }

            if !threatened_at(
                board.get_king_position(color),
                &[from],
                &[pos],
                color,
                board,
            ) {
                dst.push(pos);
            }

            if target_color.is_some() {
                break;
            }
        }
    }
}
