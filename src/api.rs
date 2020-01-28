use std::thread;
use std::sync::mpsc;
use actix_web::{guard, web, App, HttpResponse, HttpServer, HttpRequest, Responder, Result};
use ini::Ini;

use crossbeam_channel::{bounded, Sender};
use json::JsonValue;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Debug)]
pub struct TransferMessage {
    blindedMessage: String,
    signature: String,
    to: String
}

#[derive(Deserialize, Debug)]
pub struct InitMessage {
    to: String
}

pub fn init(req: HttpRequest, item: web::Json<InitMessage>) -> Result<String> {
    println!("model: {:?}", &item);
    
    Ok(format!("to: {}", item.to))
}

pub fn transfer(sender: web::Data<Sender<u64>>, req: HttpRequest, item: web::Json<TransferMessage>) -> Result<String> {
    println!("model: {:?}", &item);
    
    Ok(format!("blindedMessage {}, signature: {}, to: {}", item.blindedMessage, item.signature, item.to))
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
