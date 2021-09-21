use crate::{error, piece, Board, Error, Move, Piece, Position};

pub struct Game {
    board: Board,
}

impl Game {
    pub fn new(board: Board) -> Self {
        Self { board }
    }
    pub fn board(&self) -> &Board {
        &self.board
    }
    pub fn make_move<M, P>(&mut self, move_: M, pawn_promotion: P) -> Result<(), Error>
    where
        M: Into<Move>,
        P: FnOnce() -> piece::Kind,
    {
        let move_ = move_.into();
        if let Some(piece) = self.board[move_.from] {
            if piece.color != self.board.next_to_move() {
                return Err(Error::IllegalMove(error::IllegalMove::OtherPlayersTurn));
            }
            if !piece.can_move(move_, &self.board) {
                return Err(Error::IllegalMove(error::IllegalMove::DisallowedMovement));
                // TODO: what about `error::IllegalMove::WouldPutSelfInCheck`?
            }
        } else {
            return Err(Error::IllegalMove(error::IllegalMove::NoPieceToMove));
        }
        self.make_move_unchecked(move_, pawn_promotion)
    }
    fn make_move_unchecked<M, P>(&mut self, move_: M, pawn_promotion: P) -> Result<(), Error>
    where
        M: Into<Move>,
        P: FnOnce() -> piece::Kind,
    {
        let move_: Move = move_.into();
        let piece = self.board[move_.from].unwrap();
        let current_color = self.board.next_to_move();

        self.board[move_.to] = self.board[move_.from].take();

        // Handle promotion
        if piece.kind == piece::Kind::Pawn && (move_.to.rank() == 7 || move_.to.rank() == 0) {
            self.board[move_.to] = Some(Piece::new(current_color, pawn_promotion()))
        }

        // Handle castling
        let delta_file = move_.to.file() as i8 - move_.from.file() as i8;
        if piece.kind == piece::Kind::King && delta_file.abs() == 2 {
            let rook_pos =
                Position::new_unchecked(if delta_file > 0 { 7 } else { 0 }, move_.to.rank());
            let rook_dst_file = move_.to.file() as i8 + -delta_file / 2;
            let rook_dst = Position::new_unchecked(rook_dst_file as u8, move_.to.rank());
            self.board[rook_dst] = self.board[rook_pos].take();
        }

        // Handle castling marking
        match (piece.kind, move_.from.file()) {
            (piece::Kind::King, _) => {
                self.board.cannot_castle_kingside(current_color);
                self.board.cannot_castle_queenside(current_color);
            }
            (piece::Kind::Rook, 0) => self.board.cannot_castle_queenside(current_color),
            (piece::Kind::Rook, 7) => self.board.cannot_castle_kingside(current_color),
            _ => {}
        }

        // Handle en passant capture
        if piece.kind == piece::Kind::Pawn && Some(move_.to) == self.board.en_passant_square() {
            let target_rank = move_.to.rank() as i8 + current_color.backwards();
            let target = Position::new_unchecked(move_.to.file(), target_rank as u8);
            self.board[target] = None;
        }

        // Handle en passant marking
        let delta_rank = move_.to.rank() as i8 - move_.from.rank() as i8;
        if piece.kind == piece::Kind::Pawn && delta_rank.abs() == 2 {
            let eps_rank = move_.to.rank() as i8 + current_color.backwards();
            self.board
                .set_en_passant_square(Some(Position::new_unchecked(
                    move_.to.file(),
                    eps_rank as u8,
                )));
        } else {
            self.board.set_en_passant_square(None);
        }

        self.board.switch_next_to_move();

        Ok(())
    }
}
