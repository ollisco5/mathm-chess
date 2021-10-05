
use std::{
    io::{ErrorKind, Read, Write, BufReader, BufRead},
    net::{SocketAddr, TcpListener, TcpStream},
    sync::mpsc::{self, Receiver, Sender, channel},
};
use std::thread;

use chess_engine;
use chess_engine::Board;
use chess_engine::Color::{self, White, Black};
use chess_engine::GameState;
use chess_engine::util::Move;
use chess_engine::piece::Kind::{Queen, Bishop, Rook, Knight};



pub struct Server {
    listener: TcpListener,
    clients: Vec<TcpStream>,
    tx: Sender<(String, SocketAddr)>,
    rx: Receiver<(String, SocketAddr)>,
}

fn match_color_to_char(color: Color) -> char {
    match color {
        White => 'w',
        Black => 'b',
    }
}


impl Server {
    pub fn new() -> Self {
        let listener = TcpListener::bind("127.0.0.1:8080")
            .expect("Error binding listener");
            
        listener
            .set_nonblocking(true)
            .expect("Error initializing non block");
    
        let clients = Vec::new();

        let (tx, rx) = channel::<(String, SocketAddr)>();

        Self {
            listener,
            clients,
            tx,
            rx,
        }
    }

    pub fn run(&mut self, game: &mut chess_engine::Game) -> Result<(), &'static str> {
        self.accept_client();
        loop {
            self.chess_handling(game);
        }


        Ok(())
    }

    fn accept_client(&mut self) {
        let (socket, address) = match self.listener.accept() {
            Ok((s, a)) => (s, a),
            Err(e) => {
                println!("{}", e); 
                return
            },
        };
        println!("{} conected", address);

        let tx = self.tx.clone(); // Sender

        self.clients
            .push(socket.try_clone()
            .expect("Error cloning socket while pushing to server.clients"));

        let mut reader = BufReader::new(
            socket
                .try_clone()
                .expect("Error cloning socket into BufReader")
                
        );

        
        
        thread::spawn(move || loop {
            let mut message = String::new();

            match reader.read_line(&mut message) {
                Err(e) => {
                    println!("Closing connection due to: '{}'", e);
                    break;
                }, // Maybe change to return Result
                Ok(_) => {
                    println!("{} sent message: {}", address, message);
                    
                    tx.send((message.clone(), address)) // Do i need to clone it?
                        .expect(&format!("Tx ({}) failed to send message ({})", address, message)[..]);

                },
            
            };


            // Should i sleep her?

        });

    }

    fn chess_handling(&mut self, game: &mut chess_engine::Game) {
        let (incomming_message, from_address) =  match self.rx.recv() {
            Ok((msg, addr)) => (msg, addr),
            Err(e) => panic!("Error reciveing message: {}", e), // Return result
        };

        let split: Vec<String> = incomming_message.split(":").map(|s| s.to_string()).collect();  // Maybe map each slice to String 

        if split.len() != 2 {
            panic!("Payload error while splitting at \":\"");
        }

        
        let payload_type = split.get(0).unwrap();
        

                
        let mut sending_message = String::new();
        let mut send_to_everyone = false; // false implies send to self only
        match &payload_type[..] {
            "move" => {
                let data = split.get(1).unwrap(); // Warning this gets the ";"
                if (*data).len() != 5 {
                    panic!("Invalid move length");
                }
                let move_ = &data[0..4];
                let pawn_promotion = data.chars().nth(4).unwrap();
                let move_ = match Move::arabic(move_) {
                    Ok(m) => m,
                    Err(e) => panic!("Error making move: {}", e),
                };

                let pawn_promotion = match pawn_promotion {
                    'q' => Queen,
                    'r' => Rook,
                    'b' => Bishop,
                    'n' => Knight,
                    '-' => Queen,
                    _ => panic!("Invalid promotion char"),
                };

                match game.make_move(move_, || pawn_promotion) {
                    Ok(GameState::Ongoing) => {
                        println!("Ongoing");
                        let new_board_fen = game.board().to_fen();
                        sending_message = format!("board:{}", new_board_fen);
                        send_to_everyone = true;

                    },
                    Ok(GameState::Draw) => {
                        println!("Draw");
                        let new_board_fen = game.board().to_fen();
                        sending_message = format!("end:-{}", new_board_fen);
                        send_to_everyone = true;
                      
                    }
                    Ok(GameState::Checkmate { winner }) => {
                        println!("Checkmate! {:?} wins", winner);
                        let new_board_fen = game.board().to_fen();
                        let winner_color = match_color_to_char(winner);
                        sending_message = format!("end:{}{}", winner_color, new_board_fen);
                        send_to_everyone = true;
                    }
                    Err(err) => {
                        println!("Error: {}", err);
                        sending_message = format!("err:{};", err);
                    }
                }
            },
            "init" => {
                println!("Init");
                let new_board_fen = game.board().to_fen();
                sending_message = format!("board:{}", new_board_fen);
            },
            _ => sending_message = format!("err:Invalid payload type"),
        }

        let tx = self.tx.clone();
        if send_to_everyone {
            for client in &mut self.clients {
                let _error = client.write_all(sending_message.as_bytes()); 
            }
        }
        else {
            for client in &mut self.clients {
                let client_address = client.peer_addr();
                
                let client_address = match client_address {
                    Ok(client_addr) => client_addr,
                    Err(e) => {
                        println!("{}", e);
                        continue
                    },
                };
                let _error = client.write_all(sending_message.as_bytes());
                break;
            }
        }
    }
}


fn main() {
    println!("Hello world");
}