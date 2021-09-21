use crate::{Board, Position};

use super::util::floating_moves;

pub fn append_moves(board: &Board, from: Position, dst: &mut Vec<Position>) {
    floating_moves(&[(0, 1), (1, 0), (0, -1), (-1, 0)], board, from, dst)
}
