use std::net::{TcpStream, IpAddr};
use std::sync::mpsc::Sender;

const MAX_PORT: u16 = 65535;

pub fn scan(tx: Sender<u16>, start_port: u16, addr: IpAddr, num_threads: u16) {
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
