use bytes::BytesMut;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

pub mod respv2;
pub mod server;

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
                    stream
                        .write_all(format!("-ERR {}\r\n", e.to_string()).as_bytes())
                        .await
                        .unwrap();
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
    let string = read_to_string(read).await?;

    let response = server::Redis::handle(string).await?;

    write.write_all(response.as_bytes()).await?;

    Ok(())
}

async fn read_to_string(read: tokio::net::tcp::ReadHalf<'_>) -> Result<String, std::io::Error> {
    let mut reader = BufReader::new(read);
    let mut buffer = BytesMut::with_capacity(1000);

    reader.read_buf(&mut buffer).await?;

    let buffer: bytes::Bytes = buffer.freeze();
    let string = String::from_utf8(buffer.to_vec()).unwrap();

    Ok(string)
}
