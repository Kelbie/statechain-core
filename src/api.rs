use std::thread;
use std::env;
use std::sync::mpsc;
use actix_web::{guard, web, App, HttpResponse, HttpServer, HttpRequest, Responder, Result};
use ini::Ini;
use rocksdb::{DB, Options};
use sha2::{Sha256, Sha512, Digest};
use std::str;
use bitcoin_hashes::{sha256, Hash};
use bitcoin_hashes::hex::{FromHex, ToHex};
use std::net::TcpStream;

use crossbeam_channel::{bounded, Sender};
use json::JsonValue;
use serde::{Serialize, Deserialize};
use std::io::prelude::*;

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

fn send_to_peers(encoded_message: &Vec<u8>) -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    
    let conf = Ini::load_from_file(args.get(1).unwrap()).unwrap();
    
    let section = conf.section(Some("network").to_owned()).unwrap();

    let peers = section.get_vec("network.peers[]").unwrap();

    for peer in peers {
        let mut stream = TcpStream::connect(peer).unwrap();
        println!("{:?}", &encoded_message);
        stream.write(&encoded_message).unwrap();
    }

    Ok(())
}

pub fn getTransferById(sender: web::Data<Sender<u64>>, transfer_id: web::Path<String>, req: HttpRequest, item: web::Json<TransferMessage>) -> Result<String> {
    
    let args: Vec<String> = env::args().collect();
    let path = String::from("/Users/kevinkelbie/Documents/GitHub/statechain-core/src/") + args.get(2).unwrap();
    let db = DB::open_default(path).unwrap();
    let decoded_transfer_message = match db.get(String::from(transfer_id.as_ref())) {
        Ok(Some(value)) => bincode::deserialize(&value[..]).unwrap(),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e)
    };

    println!("{:?}", decoded_transfer_message);

    Ok(format!("{}", transfer_id))
}


pub fn init(sender: web::Data<Sender<u64>>, req: HttpRequest, item: web::Json<InitMessage>) -> Result<String> {
    let initMessage = InitMessage {
        receiver: String::from(&item.receiver),
    };

    // encode struct as vector
    let encoded_init_message = bincode::serialize(&initMessage).unwrap();

    // get sha256 digest of vector
    let digest = sha256::Hash::hash(encoded_init_message.as_ref());

    // save encoded message with digest as key
    let args: Vec<String> = env::args().collect();
    let path = String::from("/Users/kevinkelbie/Documents/GitHub/statechain-core/src/") + args.get(2).unwrap();
    let db = DB::open_default(path).unwrap();
    db.put(String::from(digest.to_hex()), &encoded_init_message).unwrap();

    // tell peers about the transfer
    send_to_peers(&encoded_init_message);

    // return digest
    Ok(format!("{}", digest.to_hex()))
}

pub fn transfer(sender: web::Data<Sender<u64>>, req: HttpRequest, item: web::Json<TransferMessage>) -> Result<String> {
    let transfer_message = TransferMessage {
        blinded_message: String::from(&item.blinded_message),
        signature: String::from(&item.signature),
        receiver: String::from(&item.receiver)
    };

    // encode struct as vector
    let encoded_init_message = bincode::serialize(&transfer_message).unwrap();

    // get sha256 digest of vector
    let digest = sha256::Hash::hash(encoded_init_message.as_ref());

    // save encoded message with digest as key
    let args: Vec<String> = env::args().collect();
    let path = String::from("/Users/kevinkelbie/Documents/GitHub/statechain-core/src/") + args.get(2).unwrap();
    let db = DB::open_default(path).unwrap();
    db.put(String::from(digest.to_hex()), encoded_init_message).unwrap();

    // return digest
    Ok(format!("{}", digest.to_hex()))
}

pub fn main(sender: Sender<u64>) {
    let handle = thread::spawn(move || {
        let args: Vec<String> = env::args().collect();

        let conf = Ini::load_from_file(args.get(1).unwrap()).unwrap();
    
        let section = conf.section(Some("api").to_owned()).unwrap();
    
        let port = section.get("api.port").unwrap();

        HttpServer::new(move || {
            App::new()
                .register_data(web::Data::new(sender.clone())) // Pass the sender to the service
                // enable logger
                // .wrap(middleware::Logger::default())
                // register simple handler, handle all methods
                .service(web::resource("/transfer/{transfer_id}")
                    .data(web::JsonConfig::default().limit(1024))
                    .route(web::post().to(getTransferById)))
                .service(web::resource("/init").route(web::post().to(init)))
                .service(web::resource("/transfer").route(web::post().to(transfer)))
        })
        .bind(format!("127.0.0.1:{}", port))
        .expect(&format!("Can not bind to port {}", port)[..])
        .run()
        .unwrap();
    });

    handle.join().unwrap();
}
