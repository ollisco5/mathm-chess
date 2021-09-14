use chess_engine::{Board, Game, Move};
use std::io::BufRead;

fn main() {
    let mut game = Game::new(Board::default());
    print!("{}", game.board().to_string());
    for line in std::io::stdin().lock().lines().map(|line| line.unwrap()) {
        let m = match Move::arabic(line.trim()) {
            Ok(m) => m,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        match game.make_move(m, || unimplemented!()) {
            Ok(()) => (),
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        print!("{}", game.board().to_string());
    }
}
