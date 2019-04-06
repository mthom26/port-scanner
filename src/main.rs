extern crate clap;

use std::net::{ToSocketAddrs, IpAddr, TcpStream};
use std::io::{Error};
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use clap::{App, Arg};

const MAX_PORT: u16 = 65535;

fn main() {
    let matches = App::new("Port Scanner")
        .arg(Arg::with_name("url")
                .help("The URL to scan")
                .required(true))
        .arg(Arg::with_name("threads")
                .help("Number of threads to use")
                .short("t")
                .long("threads")
                .takes_value(true)
                .value_name("threads")
                .default_value("100"))
        .get_matches();

    println!("{:?}", matches);

    let url = matches.value_of("url").unwrap();
    let num_threads = matches.value_of("threads").unwrap();
    println!("Url: {}", url);
    println!("Num Threads: {}", num_threads);
    
    // Convert url to socket address
    let address: Result<Vec<IpAddr>, Error> = (url, 0).to_socket_addrs().map(|iter| {
        iter.map(|addr| addr.ip()).collect()
    });
    
    // Exit program if address is an error
    if address.is_err() {
        println!("{:?}", address);
        process::exit(1);
    }

    let address = address.unwrap();
    println!("{:?}", address);

    let num_threads: u16 = num_threads.parse().unwrap();
    let (tx, rx) = channel();
    for index in 0..num_threads {
        let tx = Sender::clone(&tx);
        // TODO - maybe send reference of address to 'scan()' instead? 
        let addr = address.clone()[0];
        thread::spawn(move || {
            scan(tx, index, addr, num_threads);
        });
    }

    let mut output = Vec::new();
    drop(tx);

    for port in rx {
        output.push(port);
    }

    println!("Output: {:?}", output);
}

fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
    let mut current_port = start_port + 1;
    loop {
        match TcpStream::connect((addr, current_port)) {
            Ok(_) => {
                // Connection succeeded, send port number
                tx.send(current_port).unwrap();
            },
            Err(_) => {
                // connection failed, nothing to do
            }
        }

        if (MAX_PORT - current_port) <= num_threads {
            break;
        }
        current_port += num_threads;
    }
}
