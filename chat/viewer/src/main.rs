use std::io::prelude::*;
use std::io::BufReader;
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:9090")
        .expect("An error occurred while binding '0.0.0.0:9090' port for visualization...");

    for stream in listener.incoming() {
        match stream {
            Ok(stm) => {
                thread::spawn(move || {
                    handle_stream(&mut BufReader::new(stm));
                });
                ()
            }
            Err(_) => println!("An error occurred to connect a client..."),
        };
    }
}

fn handle_stream(stream: &mut BufReader<TcpStream>) {
    let mut msg = String::from("");
    loop {
        stream
            .read_line(&mut msg)
            .expect("An error occurred while reading client data...");
        if msg.len() == 0 {
            return;
        };
        println!("{}", msg);
        msg = String::from("");
    }
}
