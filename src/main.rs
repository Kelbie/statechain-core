use std::thread;
use std::sync::mpsc;
use std::net::{TcpListener, TcpStream, Shutdown};
use std::io::{Read, Write};
use ini::Ini;

extern crate crossbeam;
#[macro_use]
extern crate crossbeam_channel;

use crossbeam_channel::{bounded, Sender};

mod api;
mod network;


fn main() {
    // database::main();

    // let (s, r) = bounded(0);
    
    // let apiHandle = thread::spawn(move || {
    //     api::main(s);
    // });

    let networkHandle = thread::spawn(move || {
        network::main();
    });
    
    networkHandle.join();
    // apiHandle.join();

}