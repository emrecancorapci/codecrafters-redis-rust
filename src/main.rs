use std::io::{Error, ErrorKind};

use bytes::BytesMut;
use respv2::{Parser, RESPv2Types, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

pub mod respv2;

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    let thread_pool = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();

    loop {
        let (mut stream, ip) = listener.accept().await.unwrap();

        println!("Connection from: {}", ip);

        thread_pool.spawn(async move {
            let result = handler(&mut stream).await;

            if let Err(e) = result {
                if e.kind() != std::io::ErrorKind::BrokenPipe {
                    eprintln!("Error: {}", e);
                    println!("Connection closed from: {}", ip);
                } else {
                    eprintln!("Error: {}", e);
                }
            }
        });
    }
}

async fn handler(stream: &mut TcpStream) -> Result<(), std::io::Error> {
    let (read, mut write) = stream.split();
    let mut reader = BufReader::new(read);
    let mut buffer = BytesMut::with_capacity(1000);
    let _ = reader.read_buf(&mut buffer).await;

    let buffer: bytes::Bytes = buffer.freeze();
    let string = String::from_utf8(buffer.to_vec()).unwrap();

    let parse_result = string.try_parse_to_respv2();

    if parse_result.is_err() {
        return Err(Error::new(
            ErrorKind::InvalidData,
            parse_result.unwrap_err().to_string(),
        ));
    }

    match parse_result.unwrap() {
        RESPv2Types::Array(vec) => {
            let mut itr = vec.iter().peekable();

            while let Some(data) = itr.next() {
                match data.as_ref() {
                    RESPv2Types::Array(_) => todo!(),
                    RESPv2Types::Number(_) => todo!(),
                    RESPv2Types::String(str) => match str.to_lowercase().as_str() {
                        "ping" => {
                            let _ = write
                                .write_all("PONG".serialize_to_respv2().as_bytes())
                                .await;
                            return Ok(());
                        }
                        "echo" => match itr.peek() {
                            Some(echo) => match echo.as_ref() {
                                RESPv2Types::String(echo) => {
                                    let _ = write
                                        .write_all(echo.serialize_to_respv2().as_bytes())
                                        .await;
                                    return Ok(());
                                }
                                _ => {
                                    return Err(Error::new(
                                        ErrorKind::InvalidData,
                                        "Wrong use of ECHO command.",
                                    ))
                                }
                            },
                            None => {
                                return Err(Error::new(
                                    ErrorKind::InvalidData,
                                    "ECHO command needs another argument: ECHO [message]",
                                ))
                            }
                        },
                        _ => {}
                    },
                    RESPv2Types::Bulk(_) => todo!(),
                    RESPv2Types::Error(_) => todo!(),
                    RESPv2Types::Null => todo!(),
                }
            }
        }
        RESPv2Types::Number(_) => todo!(),
        RESPv2Types::String(_) => todo!(),
        RESPv2Types::Bulk(_) => todo!(),
        RESPv2Types::Error(_) => todo!(),
        RESPv2Types::Null => todo!(),
    }
    Ok(())
}
