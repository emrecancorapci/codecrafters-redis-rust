use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};

#[tokio::main]
async fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    while let Ok((ref mut stream, _)) = listener.accept().await {
        handler(stream).await;
    }
}

async fn handler(stream: &mut TcpStream) {
    let buffer: &mut [u8] = &mut [0; 1024];
    let length = stream.read(buffer).await;

    if length.is_err() {
        println!("Error reading from stream: {:?}", length.err());
        return;
    }

    let received = String::from_utf8_lossy(&buffer[..length.unwrap()]).to_string();

    let mut lines = received.lines();

    while let Some(line) = lines.next() {
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

        let _ = stream.write_all(answer.as_bytes()).await;
    }
}

fn ping<'a>() -> &'a str {
    return "+PONG\r\n";
}
