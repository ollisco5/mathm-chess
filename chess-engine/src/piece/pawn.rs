use crate::{Board, Color, Position};

use super::util::position_hides_check;

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
    let was_hiding_check = position_hides_check(board, from, color);

    let mut can_move_forwards = true;
    let mut can_capture_left = true;
    let mut can_capture_right = true;
    if let Some((d, _)) = was_hiding_check {
        if d.0 != 0 {
            can_move_forwards = false;
        }
        if d != (-1, forwards) {
            can_capture_left = false;
        }
        if d != (1, forwards) {
            can_capture_right = false;
        }
    }

    if can_move_forwards {
        if let Some(pos) = Position::new(from.file(), (from.rank() as i8 + forwards) as u8) {
            dst.push(pos);
        }

        // en passant
        if from.rank() == 6 && color == Color::White || from.rank() == 1 && color == Color::Black {
            if let Some(pos) = Position::new(from.file(), (from.rank() as i8 + forwards * 2) as u8)
            {
                dst.push(pos);
            }
        }
    }

    // capturing
    // TODO: [e]
    //   q    p P   K
    if let Some(position) = Position::new_i8(from.file() as i8 - 1, from.rank() as i8 + forwards) {
        if can_capture_left
            && (board[position].map(|p| p.color) == Some(color.other())
                || Some(position) == board.en_passant_square())
        {
            dst.push(position);
        }
    }
    if let Some(position) = Position::new_i8(from.file() as i8 + 1, from.rank() as i8 + forwards) {
        if can_capture_right
            && (board[position].map(|p| p.color) == Some(color.other())
                || Some(position) == board.en_passant_square())
        {
            dst.push(position);
        }
    }
}
