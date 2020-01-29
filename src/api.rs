use std::thread;
use std::sync::mpsc;
use actix_web::{guard, web, App, HttpResponse, HttpServer, HttpRequest, Responder, Result};
use ini::Ini;
use rocksdb::{DB, Options};
use sha2::{Sha256, Sha512, Digest};
use std::str;
use bitcoin_hashes::{sha256, Hash};
use bitcoin_hashes::hex::{FromHex, ToHex};

use crossbeam_channel::{bounded, Sender};
use json::JsonValue;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct TransferMessage {
    blindedMessage: String,
    signature: String,
    to: String
}

#[derive(Serialize, Deserialize, Debug)]
pub struct InitMessage {
    to: String
}

pub fn getTransferById(sender: web::Data<Sender<u64>>, transfer_id: web::Path<String>, req: HttpRequest, item: web::Json<TransferMessage>) -> Result<String> {
    
    let path = "/Users/kevinkelbie/Documents/GitHub/statechain-core/src/db";
    let db = DB::open_default(path).unwrap();
    let decodedTransferMessage = match db.get(String::from(transfer_id.as_ref())) {
        Ok(Some(value)) => bincode::deserialize(&value[..]).unwrap(),
        Ok(None) => println!("value not found"),
        Err(e) => println!("operational problem encountered: {}", e)
    };

    println!("{:?}", decodedTransferMessage);

    Ok(format!("{}", transfer_id))
}


pub fn init(sender: web::Data<Sender<u64>>, req: HttpRequest, item: web::Json<InitMessage>) -> Result<String> {
    let initMessage = InitMessage {
        to: String::from(&item.to),
    };

    // encode struct as vector
    let encodedInitMessage = bincode::serialize(&initMessage).unwrap();

    // get sha256 digest of vector
    let digest = sha256::Hash::hash(encodedInitMessage.as_ref());

    // save encoded message with digest as key
    let path = "/Users/kevinkelbie/Documents/GitHub/statechain-core/src/db";
    let db = DB::open_default(path).unwrap();
    db.put(String::from(digest.to_hex()), encodedInitMessage).unwrap();

    // return digest
    Ok(format!("{}", digest.to_hex()))
}

pub fn transfer(sender: web::Data<Sender<u64>>, req: HttpRequest, item: web::Json<TransferMessage>) -> Result<String> {
    let transferMessage = TransferMessage {
        blindedMessage: String::from(&item.blindedMessage),
        signature: String::from(&item.signature),
        to: String::from(&item.to)
    };

    // encode struct as vector
    let encodedInitMessage = bincode::serialize(&transferMessage).unwrap();

    // get sha256 digest of vector
    let digest = sha256::Hash::hash(encodedInitMessage.as_ref());

    // save encoded message with digest as key
    let path = "/Users/kevinkelbie/Documents/GitHub/statechain-core/src/db";
    let db = DB::open_default(path).unwrap();
    db.put(String::from(digest.to_hex()), encodedInitMessage).unwrap();

    // return digest
    Ok(format!("{}", digest.to_hex()))
}

pub fn main(sender: Sender<u64>) {
    let handle = thread::spawn(move || {
        let conf = Ini::load_from_file("statechain.conf").unwrap();
    
        let section = conf.section(None::<String>).unwrap();
    
        let port = section.get("port").unwrap();

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
