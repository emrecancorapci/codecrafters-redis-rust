#![allow(unused_imports)]
use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut _stream) => {
                handler(&_stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handler(mut stream: &TcpStream) {
    let buffer: &mut [u8] = &mut [0; 1024];
    let length = stream.read(buffer);

    if length.is_err() {
        println!("Error reading from stream: {:?}", length.err());
        return;
    }

    let recieved = String::from_utf8_lossy(&buffer[..length.unwrap()]).to_string();

    let answer: &str = match recieved.to_uppercase().as_str() {
        "*1\r\n$4\r\nPING\r\n" => ping(),
        _ => "-ERR unknown command\r\n",
    };

    let _ = stream.write_all(answer.as_bytes());
}

fn ping<'a>() -> &'a str {
    return "+PONG\r\n";
}
