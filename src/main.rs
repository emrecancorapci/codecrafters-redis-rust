use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

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
    let reader = BufReader::new(read);
    let mut lines = reader.lines();

    while let Some(line) = lines.next_line().await? {
        let answer = match line.to_uppercase().as_str() {
            "PING" => "+PONG\r\n",
            l => {
                if l.starts_with(&['*', '$']) {
                    ""
                } else {
                    "-ERR unknown command\r\n"
                }
            }
        };

        if answer.is_empty() {
            continue;
        }

        println!("Answer: {}", answer);

        let _ = write.write_all(answer.as_bytes()).await;
    }
    Ok(())
}
