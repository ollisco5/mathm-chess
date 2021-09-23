use crate::{piece, Board, Error, Game, GameState, Move};

/// # Example use:
/// struct D;
///
/// impl Decider for D {
///     fn get_move(&mut self, board: &Board) -> Move {
///         /* ... */
///     }
///     fn get_pawn_promotion(&mut self) -> piece::Kind {
///         /* ... */
///     }
/// }
///
/// fn main() {
///     D.run(Board::default()).unwrap();
/// }

pub trait Decider {
    fn get_move(&mut self, board: &Board) -> Move;
    fn get_pawn_promotion(&mut self) -> piece::Kind;

    /// Runs a game of chess with on `board`. Do not overwrite
    fn run(&mut self, board: Board) -> Result<GameState, Error> {
        let mut game = Game::new(board);
        loop {
            let game_state =
                game.make_move(self.get_move(game.board()), || self.get_pawn_promotion())?;
            if game_state != GameState::Ongoing {
                break Ok(game_state);
            }
        }
    }
}

pub fn run_with_decider(board: Board, mut decider: impl Decider) -> Result<GameState, Error> {
    let mut game = Game::new(board);
    loop {
        let game_state = game.make_move(decider.get_move(game.board()), || {
            decider.get_pawn_promotion()
        })?;
        if game_state != GameState::Ongoing {
            break Ok(game_state);
        }
    }
}
