use crate::{Board, Color, Position};

use super::Kind;

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

/// Indicates if the piece at `pos` is being threatened from `dir` by a piece
/// with color `color.other()`
pub fn threatened_from_dir(
    board: &Board,
    color: Color,
    (dir, mut pos): ((i8, i8), Position),
) -> bool {
    for _ in 1..8 {
        pos = match Position::new_i8(pos.file() as i8 + dir.0, pos.rank() as i8 + dir.1) {
            Some(p) => p,
            None => break,
        };
        if let Some(piece) = board[pos] {
            if (piece.kind == Kind::Queen
                || piece.kind == Kind::Rook && (dir.0 == 0 || dir.1 == 0)
                || piece.kind == Kind::Bishop && dir.0 != 0 && dir.1 != 0)
                && piece.color != color
            {
                return true;
            }
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
