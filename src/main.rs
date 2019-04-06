extern crate clap;

use std::net::{ToSocketAddrs, IpAddr};
use std::io::{Error};
use std::process;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Port Scanner")
        .arg(Arg::with_name("url")
                .help("The URL to scan")
                .required(true))
        .get_matches();

    println!("{:?}", matches);

    let url = matches.value_of("url").unwrap();
    println!("Url: {}", url);
    
    // Convert url to socket address
    let address: Result<Vec<IpAddr>, Error> = (url, 0).to_socket_addrs().map(|iter| {
        iter.map(|addr| addr.ip()).collect()
    });
    
    // Exit program if address is an error
    if address.is_err() {
        println!("{:?}", address);
        process::exit(1);
    }

    println!("{:?}", address);
}
