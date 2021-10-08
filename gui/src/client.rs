use std::{fmt::format, io::{self, BufRead, BufReader, ErrorKind, Read, Write}, net::{SocketAddr, TcpStream}, sync::{Arc, Mutex, mpsc::{self, Receiver, SyncSender, TryRecvError, sync_channel}}, thread, time::Duration};

use chess_engine;
// use crate::Game;

struct Client {
    stream: TcpStream,
    tx: SyncSender<(String, SocketAddr)>,
    rx: Arc<Mutex<Receiver<(String, SocketAddr)>>>,
    reader: BufReader<TcpStream>,
}

impl Client {
    pub fn new() -> Self {
        let mut ip: String = String::new();
        println!("Input host ip");
        let line = std::io::stdin()
            .read_line(&mut ip)
            .expect("Could not read line");
        
        ip.pop();
        println!("Trying to connect to |{}|", ip);

        let mut stream = TcpStream::connect("127.0.0.1:8080")
            .expect("Client could not connect");
        // client.set_nonblocking(true).expect("failed to initiate non-blocking");

        let mut reader = BufReader::new(
            stream
                .try_clone()
                .expect("Error: could not clone client into buffreader")
        );

        let (tx, rx) = sync_channel::<(String, SocketAddr)>(1);
        Self {
            stream,
            tx,
            rx: Arc::new(Mutex::new(rx)),
            reader,
        }
    }

    pub fn run(&mut self, game: &Arc<Mutex<chess_engine::Game>>) -> Result<(), std::io::Error> {
        
        let rx = self.rx.clone();
        let mut game = game.clone();
        thread::spawn(move || loop {
            let (mut incomming_message, sending_address) = match rx.lock().unwrap().recv().ok() {
                Some((msg, addr)) => (msg, addr),
                None => {
                    println!("Server: Error reciveing message");
                    return;
                } // Return result
            };
            println!("Client: Recieved message: {}", incomming_message);
            
            let mut split: Vec<String> = incomming_message
                .trim()
                .split(":")
                .map(|s| s.to_string())
                .collect();
            
            if split.len() != 2 {
                panic!("Payload error while splitting at \":\"");
            }

            let payload_type = &split.get(0).unwrap()[..];

            match payload_type {
                "board" => {
                    let mut data = split.get(1).unwrap().to_string();
                    data.retain(|s| s != ';');
                    let new_board = chess_engine::Board::from_fen(&data[..]);
                    let board = match new_board {
                        Ok(board) => board,
                        _ => panic!("Error getting board from fen"),
                    };

                    let new_chess_engine_game = chess_engine::Game::new(board);
                    game = Arc::new(Mutex::new(new_chess_engine_game));

                }
                "end" => panic!("Game over!"),
                "err" => panic!("Error"),
                _ => {
                    ()
                }

            }

        });

        Ok(())
    }

    pub fn send(&mut self, message: String) -> Result<(), String> {
        let result = self.tx.send((message.clone(), self.stream.peer_addr().unwrap()));
        match result {
            Err(e) => Err(format!("Error sending: {}", e)),
            _ => Ok(()),
        }
    }
}


fn main() {
    let mut g = Arc::new(
        Mutex::new(
                chess_engine::Game::new(
                        chess_engine::Board::default()
                )
        )
    );
    let mut client = Client::new();
    client.run(&g.clone());

}