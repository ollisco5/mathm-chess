use crate::{Board, Position};

pub fn floating_moves(deltas: &[(i8, i8)], board: &Board, from: Position, dst: &mut Vec<Position>) {
    let this_color = board[from].unwrap().color;
    for (x, y) in deltas {
        for i in 1..8 {
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
