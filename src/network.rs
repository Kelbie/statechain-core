use std::thread;
use std::sync::mpsc;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use ini::Ini;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferMessage {
    blindedMessage: String,
    signature: String,
    receiver: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitMessage {
    owner: String
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 50]; // using 50 byte buffer
    while match stream.read(&mut data) {
        Ok(size) => {

            let transferMessage = TransferMessage {
                blindedMessage: String::from("blindedMessage"),
                signature: String::from("signature"),
                receiver: String::from("receiver"),
            };

            // encode struct to vec
            let encodedTransferMessage = bincode::serialize(&transferMessage).unwrap();

            // println!("{:?}", encodedTransferMessage.as_slice());

            // let decodedTransferMessage: TransferMessage = bincode::deserialize(&encodedTransferMessage[..]).unwrap();
            
            // println!("{:?}", decodedTransferMessage);

            stream.write(encodedTransferMessage.as_slice()).unwrap();
            true
        },
        Err(_) => {
            println!("An error occurred, terminating connection with {}", stream.peer_addr().unwrap());
            stream.shutdown(Shutdown::Both).unwrap();
            false
        }
    } {}
}

pub fn main() {
    // let conf = Ini::load_from_file("statechain.conf").unwrap();

    // let section = conf.section(Some("statechain".to_owned())).unwrap();
    // let port = section.get("port").unwrap();
    // let peers = section.get("peers").unwrap();

    // println!("peers: {:?}", peers);

    let port = "9939";

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    
    println!("Server listening on port {}", port);
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection: {}", stream.peer_addr().unwrap());
                thread::spawn(move || {
                    handle_client(stream)
                });
            }
            Err(e) => {
                println!("Error: {}", e);
            }
        }
    }
    drop(listener);
}