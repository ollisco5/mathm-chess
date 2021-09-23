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

/// Indicates whether a piece at `pos` is hiding a check for the king with
/// `color`, i.e. in one direction there is a king with `color`, and in the other
/// there is a piece with `color.other()` that can capture in that direction. If
/// this is not the case, `None` is returned. If it is the case, the direction
/// from the king to `pos` and the position of the king is returned.
///
/// If this returns `Some(...)`, and the color if the piece at `pos` is the same
/// as `color`, the piece may only move if `threatened_from_dir` called with
/// parameters from this function returns false, or this is checked in the other
/// way.
pub fn position_hides_check(
    board: &Board,
    pos: Position,
    color: Color,
) -> Option<((i8, i8), Position)> {
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
        let mut king_pos = None;
        for i in 1..8 {
            let p = match Position::new_i8(pos.file() as i8 + i * x, pos.rank() as i8 + i * y) {
                Some(pos) => pos,
                None => break,
            };
            if let Some(piece) = board[p] {
                if piece.kind == Kind::King && piece.color == color {
                    king_pos = Some(p);
                }
                break;
            }
        }
        let king_pos = match king_pos {
            Some(king_pos) => king_pos,
            None => continue,
        };
        for i in 1..8 {
            let p = match Position::new_i8(pos.file() as i8 - i * x, pos.rank() as i8 - i * y) {
                Some(pos) => pos,
                None => break,
            };
            if let Some(piece) = board[p] {
                if (piece.kind == Kind::Queen
                    || piece.kind == Kind::Rook && (x == 0 || y == 0)
                    || piece.kind == Kind::Bishop && x != 0 && y != 0)
                    && piece.color != color
                {
                    return Some(((-x, -y), king_pos));
                }
            }
        }
    }

    None
}

// /// Indicates if the piece at `pos` is being threatened from `dir` by a piece
// /// (either rook, bishop, or queen) with color `color.other()`
// pub fn threatened_from_dir(
//     board: &Board,
//     color: Color,
//     (dir, mut pos): ((i8, i8), Position),
// ) -> bool {
//     for _ in 1..8 {
//         pos = match Position::new_i8(pos.file() as i8 + dir.0, pos.rank() as i8 + dir.1) {
//             Some(p) => p,
//             None => break,
//         };
//         if let Some(piece) = board[pos] {
//             if (piece.kind == Kind::Queen
//                 || piece.kind == Kind::Rook && (dir.0 == 0 || dir.1 == 0)
//                 || piece.kind == Kind::Bishop && dir.0 != 0 && dir.1 != 0)
//                 && piece.color != color
//             {
//                 return true;
//             }
//         }
//     }
//
//     false
// }

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
    let this_color = board[from].unwrap().color;
    let was_hiding_check = position_hides_check(board, from, this_color);
    for (x, y) in deltas {
        for i in 1..8 {
            if let Some((d, _)) = was_hiding_check {
                if !(d == (*x, *y) || d == (-x, -y)) {
                    break;
                }
            }

            let pos = match Position::new_i8(from.file() as i8 + i * x, from.rank() as i8 + i * y) {
                Some(pos) => pos,
                None => break,
            };

            let target_color = board[pos].map(|p| p.color);

            if target_color == Some(this_color) {
                break;
            }

            dst.push(pos);

            if target_color.is_some() {
                break;
            }
        }
    }
}
