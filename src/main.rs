extern crate clap;

use std::net::{ToSocketAddrs, IpAddr};
use std::io::{Error};
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;

mod scan;
use scan::scan;

mod config;
use config::Config;

fn main() {
    let config = Config::new();

    // Convert url to socket address
    let address: Result<Vec<IpAddr>, Error> = (&config.url[..], 0).to_socket_addrs().map(|iter| {
        iter.map(|addr| addr.ip()).collect()
    });
    
    // Exit program if address is an error
    if address.is_err() {
        println!("{:?}", address);
        process::exit(1);
    }

    let address = address.unwrap();
    println!("{:?}", address);

    let (tx, rx) = channel();
    for index in 0..config.threads {
        let tx = Sender::clone(&tx);
        // TODO - maybe send reference of address to 'scan()' instead? 
        let addr = address.clone()[0];
        let threads = config.threads;
        let start_port = config.start_port;
        let end_port = config.end_port;
        thread::spawn(move || {
            scan(tx, index, addr, threads, start_port, end_port);
        });
    }

    let mut output = Vec::new();
    drop(tx);

    for port in rx {
        output.push(port);
    }

    println!("Output: {:?}", output);
}
