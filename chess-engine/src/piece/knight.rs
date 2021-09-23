use crate::{Board, Color, Position};

use super::util::threatened_at;

pub fn checks(at: Position, color: Color, board: &Board) -> bool {
    let king_pos = board.get_king_position(color.other());
    let delta_file = (king_pos.file() as i8 - at.file() as i8).abs();
    let delta_rank = (king_pos.rank() as i8 - at.rank() as i8).abs();

    delta_file == 1 && delta_rank == 2 || delta_file == 2 && delta_rank == 1
}

pub fn append_moves(board: &Board, from: Position, dst: &mut Vec<Position>) {
    let color = board[from].unwrap().color;
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
            board[pos].map(|piece| piece.color) != Some(color)
                && !threatened_at(
                    board.get_king_position(color),
                    &[from],
                    &[pos],
                    color,
                    board,
                )
        }),
    )
}
