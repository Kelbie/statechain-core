use std::thread;
use std::sync::mpsc;
use std::env;

extern crate crossbeam;
#[macro_use]
extern crate crossbeam_channel;

use crossbeam_channel::{bounded, Sender};

mod api;
mod network;

fn main() {
    let (s, r) = bounded(0);
    
    // thread for rest api
    let api_handle = thread::spawn(move || {
        api::main(s);
    });
    
    // thread for peer-to-peer networking
    let network_handle = thread::spawn(move || {
        network::main();
    });

    api_handle.join();
    network_handle.join();
}