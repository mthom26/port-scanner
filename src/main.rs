extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("Port Scanner")
        .arg(Arg::with_name("url")
                .help("The URL to scan")
                .required(true))
        .get_matches();

    println!("{:?}", matches);
}
