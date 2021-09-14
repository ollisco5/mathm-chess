use crate::{error, piece, Board, Error, Move, Piece};

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
    pub fn make_move<M, P>(&mut self, move_: M, _pawn_promotion: P) -> Result<(), Error>
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
        self.make_move_unchecked(move_, _pawn_promotion)
    }
    fn make_move_unchecked<M, P>(&mut self, move_: M, pawn_promotion: P) -> Result<(), Error>
    where
        M: Into<Move>,
        P: FnOnce() -> piece::Kind,
    {
        let move_: Move = move_.into();
        let piece = self.board[move_.from].unwrap();

        self.board[move_.to] = self.board[move_.from].take();

        if piece.kind == piece::Kind::Pawn && move_.to.rank() == 7 || move_.to.rank() == 0 {
            self.board[move_.to] = Some(Piece::new(self.board.next_to_move(), pawn_promotion()))
        }

        Ok(())
    }
}
