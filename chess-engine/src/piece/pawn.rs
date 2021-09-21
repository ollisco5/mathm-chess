use crate::{Board, Color, Position};

pub fn append_moves(board: &Board, from: Position, dst: &mut Vec<Position>) {
    let color = board[from].unwrap().color;
    let forwards = color.forwards();

    if let Some(pos) = Position::new(from.file(), (from.rank() as i8 + forwards) as u8) {
        dst.push(pos);
    }

    if from.rank() == 6 && color == Color::White || from.rank() == 1 && color == Color::Black {
        if let Some(pos) = Position::new(from.file(), (from.rank() as i8 + forwards * 2) as u8) {
            dst.push(pos);
        }
    }

    if let Some(en_passant) = board.en_passant_square() {
        if (from.file() as i8 - en_passant.file() as i8).abs() == 1
            && en_passant.rank() == (from.rank() as i8 + forwards) as u8
        {
            dst.push(en_passant);
        }
    }
}
