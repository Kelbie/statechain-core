use std::thread;
use std::env;
use std::sync::mpsc;
use actix_web::{middleware, guard, web, App, HttpResponse, HttpServer, HttpRequest, Responder, Result};
use actix_web::web::Json;
use ini::Ini;
use rocksdb::{DB, Options, WriteBatch};
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
pub struct InitResponse {
    public_key: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferResponse {
    signed_blinded_message: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct StatechainMessage {
    public_key: String,
    transfers: Vec<TransferMessage>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferMessage {
    from: String,
    blinded_message: String,
    signature: String,
    to: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitMessage {
    to: String
}

fn send_to_peers(encoded_message: &Vec<u8>) -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let conf = Ini::load_from_file(args.get(1).unwrap()).unwrap();
    let section = conf.section(Some("network").to_owned()).unwrap();

    let peers = section.get_vec("network.peers[]").unwrap();
    let speak = section.get("network.speak").unwrap();

    if speak == "1" {
        for peer in peers {
            let mut stream = TcpStream::connect(peer).unwrap();
            // println!("{:?}", &encoded_message);
            stream.write(&encoded_message).unwrap();
        }
    }

    Ok(())
}

pub fn getTransferById(transfer_id: web::Path<String>, req: HttpRequest, item: web::Json<TransferMessage>) -> Result<String> {
    
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

// fn update_statechain(encoded_message: &Vec<u8>) -> std::io::Result<()> {
//     let args: Vec<String> = env::args().collect();
//     let path = String::from("/Users/kevinkelbie/Documents/GitHub/statechain-core/src/") + args.get(2).unwrap();
//     let db = DB::open_default(path).unwrap();
//     let mut batch = WriteBatch::default();
//     let decoded_statechain = match db.get(String::from("statechain_public_key")) {
//         Ok(Some(value)) => bincode::deserialize(&value[..]).unwrap(),
//         Ok(None) => println!("value not found"),
//         Err(e) => println!("operational problem encountered: {}", e)
//     };

//     batch.put(b"statechain_public_key", decoded_statechain);
//     db.write(batch); // Atomically commits the batch

//     Ok(())
// }


pub fn init(req: HttpRequest, item: web::Json<InitMessage>) -> Result<web::Json<InitResponse>> {
    println!("HTTP: {:?}", item);

    let initMessage = InitMessage {
        to: String::from(&item.to),
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

    let init_response = InitResponse {
        public_key: String::from("public_key")
    };

    // return digest
    Ok(web::Json(init_response))
}

pub fn transfer(req: HttpRequest, item: web::Json<TransferMessage>) -> Result<web::Json<TransferResponse>> {
    let transfer_message = TransferMessage {
        from: String::from(&item.from),
        blinded_message: String::from(&item.blinded_message),
        signature: String::from(&item.signature),
        to: String::from(&item.to)
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
    let transfer_response = TransferResponse {
        signed_blinded_message: String::from("signed_blinded_message")
    };

    Ok(web::Json(transfer_response))
}

pub fn main(s: Sender<u64>) {
    let handle = thread::spawn(move || {
        let args: Vec<String> = env::args().collect();

        let conf = Ini::load_from_file(args.get(1).unwrap()).unwrap();
    
        let section = conf.section(Some("api").to_owned()).unwrap();
    
        let port = section.get("api.port").unwrap();

        HttpServer::new(move || {
            App::new()
                .register_data(web::Data::new(s.clone())) // Pass the sender to the service
                // enable logger
                .wrap(middleware::Logger::default())
                // register simple handler, handle all methods
                .service(web::resource("/transfer/{transfer_id}")
                    .data(web::JsonConfig::default().limit(1024))
                    .route(web::post().to(getTransferById)))
                // .service(web::resource("/statechain/{server_public_key}/transfers")
                //     .data(web::JsonConfig::default().limit(1024))
                //     .route(web::post().to(())))
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
