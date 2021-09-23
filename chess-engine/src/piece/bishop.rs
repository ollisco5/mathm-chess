use crate::{Board, Color, Position};

use super::util::{floating_checks, floating_moves};

const DELTAS: &[(i8, i8)] = &[(1, 1), (1, -1), (-1, -1), (-1, 1)];

pub fn checks(at: Position, color: Color, board: &Board) -> bool {
    floating_checks(DELTAS, at, color, board)
}

pub fn append_moves(board: &Board, from: Position, dst: &mut Vec<Position>) {
    floating_moves(DELTAS, board, from, dst)
}
