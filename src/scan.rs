use std::net::{TcpStream, IpAddr};
use std::sync::mpsc::Sender;

pub fn scan(
    tx: Sender<u16>,
    index: u16,
    addr: IpAddr,
    num_threads: u16,
    start_port: u16,
    end_port: u16
) {
    let mut current_port = start_port + index + 1;
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

        if (end_port - current_port) <= num_threads {
            break;
        }
        current_port += num_threads;
    }
}
