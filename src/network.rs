use std::thread;
use std::env;
use std::sync::mpsc;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use ini::Ini;
use sha2::{Sha256, Sha512, Digest};
use rocksdb::{DB, Options};
use bitcoin_hashes::{sha256, Hash};
use bitcoin_hashes::hex::{FromHex, ToHex};


use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferMessage {
    blinded_message: String,
    signature: String,
    receiver: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitMessage {
    receiver: String
}

fn handle_client(mut stream: TcpStream) {
    let mut data = [0 as u8; 500]; // using 50 byte buffer
    
    println!("{:?}", stream.read(&mut data));

    let digest = sha256::Hash::hash(&data[..]);

    // save encoded message with digest as key
    let args: Vec<String> = env::args().collect();
    let path = String::from("/Users/kevinkelbie/Documents/GitHub/statechain-core/src/") + args.get(2).unwrap();
    let db = DB::open_default(path).unwrap();
    db.put(String::from(digest.to_hex()), &data[..]).unwrap();
    
}

pub fn main() {
    let args: Vec<String> = env::args().collect();

    let conf = Ini::load_from_file(args.get(1).unwrap()).unwrap();

    let section = conf.section(Some("network".to_owned())).unwrap();
    let port = section.get("network.port").unwrap();

    let listener = TcpListener::bind(format!("127.0.0.1:{}", port)).unwrap();
    
    println!("Server listening on port {}", port);
    for stream in listener.incoming() {
        handle_client(stream.unwrap());
    }
    drop(listener);
}