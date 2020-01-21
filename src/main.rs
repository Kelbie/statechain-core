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
mod database;
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

mod tests {
    use std::io::prelude::*;
    use std::net::TcpStream;

    #[test]
    fn connect_to_port() -> std::io::Result<()> {
        let mut stream = TcpStream::connect("127.0.0.1:9939")?;

        // stream.write(&[1])?;
        // stream.read(&mut [0; 128])?;
        Ok(())
    }
}