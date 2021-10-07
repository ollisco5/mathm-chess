
use std::{io::{ErrorKind, Read, Write, BufReader, BufRead}, net::{SocketAddr, TcpListener, TcpStream}, sync::{Arc, Mutex, mpsc::{self, Receiver, Sender, channel, SyncSender, sync_channel}}};
use std::thread;
use std::time::Duration;

use chess_engine;
use chess_engine::Board;
use chess_engine::Color::{self, White, Black};
use chess_engine::GameState;
use chess_engine::util::Move;
use chess_engine::piece::Kind::{Queen, Bishop, Rook, Knight};


#[derive(Debug, PartialEq)]
enum SendTo {
    Itself,
    Everyone,
    Others,
}


pub struct Server {
    listener: TcpListener,
    clients: Vec<TcpStream>,
    tx: SyncSender<(String, SocketAddr)>,
    rx: Arc<Mutex<Receiver<(String, SocketAddr)>>>,
}

fn match_color_to_char(color: Color) -> char {
    match color {
        White => 'w',
        Black => 'b',
    }
}


impl Server {
    pub fn new() -> Self {
        let address = "localhost:8080";
        println!("Binding Listener to address:  {}", address);
        let listener = TcpListener::bind(address)
            .expect("Error binding listener");
        
        /* 
        listener
            .set_nonblocking(true)
            .expect("Error initializing non block");
        */
        
        let clients = Vec::new();

        let (tx, rx) = sync_channel::<(String, SocketAddr)>(2);

        Self {
            listener,
            clients,
            tx,
            rx: Arc::new(Mutex::new(rx)),
        }
    }

    pub fn run(&mut self, game: &mut chess_engine::Game) -> Result<(), std::io::Error> {
        let (socket, address) = match self.listener.accept() {
            Ok((s, a)) => (s, a),
            Err(e) => {
                println!("Error accepting client: {}", e); 
                return Err(e);
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
                
        
        
        println!("Creating Thread for read line");
        thread::spawn(move || loop {
            println!("Looping readline for: {}", address);
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
            thread::sleep(Duration::from_millis(100));
        });
        
        println!("Handling chess");
        let mut clients: Vec<TcpStream> = self.clients
            .iter()    
            .map(|tcp_stream| tcp_stream.try_clone().expect("cant clone tcp stream"))
            .collect();

        
        let rx = self.rx.clone();
        let mut game = game.clone();

        thread::spawn(move || loop {
            let (mut incomming_message, sending_address) =  match rx.lock().unwrap().recv().ok() {
                Some((msg, addr)) => (msg, addr),
                None => {
                    println!("Error reciveing message");
                    return;
                }, // Return result
            };
    
    

            if incomming_message.ends_with("\n") {
                incomming_message.pop();
                if incomming_message.ends_with(";") {
                    incomming_message.pop();
                }
            }
            
            
            println!("Incoming message: {}", incomming_message);
            let split: Vec<String> = incomming_message.trim().split(":").map(|s| s.to_string()).collect();  // Maybe map each slice to String 

            if split.len() != 2 {
                panic!("Payload error while splitting at \":\"");
            }

            
            let payload_type = split.get(0).unwrap();
            
            
                    
            let mut sending_message = String::new();
            let mut send_to = SendTo::Itself; // false implies send to self only
            match &payload_type[..] {
                "move" => {
                    send_to = SendTo::Everyone;
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
                            

                        },
                        Ok(GameState::Draw) => {
                            println!("Draw");
                            let new_board_fen = game.board().to_fen();
                            sending_message = format!("end:-{}", new_board_fen);
                            
                        
                        }
                        Ok(GameState::Checkmate { winner }) => {
                            println!("Checkmate! {:?} wins", winner);
                            let new_board_fen = game.board().to_fen();
                            let winner_color = match_color_to_char(winner);
                            sending_message = format!("end:{}{}", winner_color, new_board_fen);
                            
                        }
                        Err(err) => {
                            println!("Error: {}", err);
                            sending_message = format!("err:{};", err);
                            send_to = SendTo::Itself;
                        }
                    }
                },
                "init" => {
                    let new_board_fen = game.board().to_fen();
                    sending_message = format!("board:{}", new_board_fen);
                    send_to = SendTo::Itself;
                },
                _ => sending_message = format!("err:Invalid payload type"),
            }

            if send_to == SendTo::Everyone {
                for client in &mut clients {
                    println!("Sending {} to {:?}", sending_message, client);
                    let _error = client.write_all(sending_message.as_bytes()); 
                }
            }
            else if send_to == SendTo::Itself {
                if address == sending_address {
                    for client in &mut clients {
                        if client.peer_addr().unwrap() == address {
                            println!("Sending {} to {:?}", sending_message, client);
                            let _error = client.write_all(sending_message.as_bytes());
                        }
                    } 
                }
            }

            else {
                for client in &mut clients {
                    let client_address = client.peer_addr();
                    
                    let client_address = match client_address {
                        Ok(client_addr) => client_addr,
                        Err(e) => {
                            println!("{}", e);
                            continue
                        },
                    };
                    let _error = client.write_all(sending_message.as_bytes());
                }
            }
            
        });
        
        println!("Returning from run");
        Ok(())
    }      
}

