use std::thread;
use std::sync::mpsc;
use actix_web::{guard, web, App, HttpResponse, HttpServer, HttpRequest, Responder};
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

pub fn init(req: HttpRequest) -> impl Responder {
    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
}

pub fn transfer(sender: web::Data<Sender<u64>>, req: HttpRequest, item: web::Json<TransferMessage>) -> impl Responder {
    println!("model: {:?}", &item);
    
    
    sender.send(1);

    let name = req.match_info().get("name").unwrap_or("World");
    format!("Hello {}!", &name)
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
                .service(web::resource("/transfer").route(web::post().to(transfer)))
        })
        .bind(format!("127.0.0.1:{}", port))
        .expect(&format!("Can not bind to port {}", port)[..])
        .run()
        .unwrap();
    });

    handle.join().unwrap();
}
