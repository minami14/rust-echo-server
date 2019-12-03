use std::io::{Error, Read, Write};
use std::net::{TcpListener, TcpStream};
use std::thread;

fn main() {
    let listener = TcpListener::bind("0.0.0.0:1234").expect("failed to bind");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                thread::spawn(|| echo(stream).unwrap_or_else(|err| eprintln!("{}", err)));
            }
            Err(err) => eprintln!("{}", err),
        }
    }
}

fn echo(mut stream: TcpStream) -> Result<(), Error> {
    let mut buf = [0u8; 1024];
    match stream.read(&mut buf) {
        Ok(n) => match stream.write(&buf[..n]) {
            Ok(_) => stream.flush(),
            Err(err) => Err(err),
        },
        Err(err) => Err(err),
    }
}
