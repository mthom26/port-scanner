extern crate clap;

use std::net::{ToSocketAddrs, IpAddr};
use std::io::{Error};
use std::process;
use std::sync::mpsc::{Sender, channel};
use std::thread;
use clap::{App, Arg};

mod scan;
use scan::scan;

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
        .arg(Arg::with_name("ports")
                .help("Port range to scan")
                .short("p")
                .long("ports")
                .takes_value(true)
                .number_of_values(2)
                .value_names(&["Start Port", "End Port"]))
        .get_matches();

    println!("{:?}", matches);

    let url = matches.value_of("url").unwrap();
    let num_threads = matches.value_of("threads").unwrap();
    let mut port_range = matches.values_of("ports").unwrap();
    let start_port = port_range.next().unwrap();
    let end_port = port_range.next().unwrap();
    println!("Url: {}", url);
    println!("Num Threads: {}", num_threads);
    println!("Ports: {} - {}", start_port, end_port);
    
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
