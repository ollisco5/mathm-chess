use chess_engine::{piece, Board, Game, Move};
use std::io::BufRead;

fn main() {
    let mut game = Game::new(Board::default());
    print!("{}", game.board().to_string());
    let stdin = std::io::stdin();
    let mut lines = stdin.lock().lines().map(|line| line.unwrap());
    while let Some(line) = lines.next() {
        let m = match Move::arabic(line.trim()) {
            Ok(m) => m,
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };

        match game.make_move(m, || loop {
            break match piece::Kind::from_name(lines.next().unwrap().chars().next().unwrap()) {
                Ok(kind) => kind,
                Err(err) => {
                    println!("{}", err);
                    continue;
                }
            };
        }) {
            Ok(()) => (),
            Err(err) => {
                println!("{}", err);
                continue;
            }
        };
        print!("{}", game.board().to_string());
    }
}
