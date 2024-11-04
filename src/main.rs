use redis::{db::MemoryDatabase, server::Redis};
use std::{io::Error, sync::Arc};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt, BufReader},
    net::TcpStream,
    sync::Mutex,
};

mod mem_db;
mod redis;
mod respv2;

#[tokio::main]
async fn main() {
    let listener = tokio::net::TcpListener::bind("127.0.0.1:6379")
        .await
        .unwrap();
    let thread_pool = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(4)
        .enable_all()
        .build()
        .unwrap();
    let db = mem_db::MemDB::new();

    let db = Arc::new(Mutex::new(db));

    loop {
        let (mut stream, ip) = listener.accept().await.unwrap();

        println!("Connection with: {}", ip);
        let db = Arc::clone(&db);

        thread_pool.spawn(async move {
            let result = handler(&mut stream, db).await;

            if let Err(ref e) = result {
                if e.kind() != std::io::ErrorKind::BrokenPipe {
                    println!("Connection closed with: {}", ip);
                } else {
                    stream
                        .write_all(format!("-ERR {}\r\n", e.to_string()).as_bytes())
                        .await
                        .unwrap();
                }
            }
        });
    }
}

async fn handler<'a>(
    stream: &mut TcpStream,
    db: Arc<Mutex<impl MemoryDatabase>>,
) -> Result<(), Error> {
    loop {
        let db = Arc::clone(&db);
        let (read, mut write) = stream.split();
        let string = read_to_string(read).await?;

        let response = Redis::handle(string, db).await?;

        write.write_all(response.as_bytes()).await?;
    }
}

async fn read_to_string(read: tokio::net::tcp::ReadHalf<'_>) -> Result<String, Error> {
    let mut reader = BufReader::new(read);
    let mut buffer = [0; 1024];

    let length = reader.read(&mut buffer).await?;

    let string = String::from_utf8(buffer[..length].to_vec())
        .unwrap()
        .trim()
        .to_string();

    Ok(string)
}
