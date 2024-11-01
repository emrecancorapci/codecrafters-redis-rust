use tokio::{
    io::{AsyncBufReadExt, AsyncWriteExt, BufReader},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    while let Ok((ref mut stream, ip)) = listener.accept().await {
        println!("Connection from: {}", ip);

        let _ = handler(stream).await;

        println!("Connection closed from: {}", ip);
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
