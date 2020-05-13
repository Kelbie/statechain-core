use std::thread;
use std::sync::mpsc;
use std::env;
use std::collections::HashMap;
use std::future;

use futures;

extern crate crossbeam;
#[macro_use]
extern crate crossbeam_channel;

#[macro_use]
extern crate ureq;

extern crate reqwest;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

use reqwest::Client;

use ini::Ini;

use crossbeam_channel::{bounded, Sender};

mod api;
mod network;

use multi_party_schnorr;

fn main() {
    let (s, r) = bounded(0);
    
    // thread for rest api
    let api_handle = thread::spawn(move || {
        api::main(s);
    });
    
    // thread for peer-to-peer networking
    // let network_handle = thread::spawn(move || {
    //     network::main();
    // });

    api_handle.join();
    // network_handle.join();
}