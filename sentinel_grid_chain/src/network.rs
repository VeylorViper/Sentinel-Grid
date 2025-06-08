use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncReadExt, AsyncWriteExt};

pub async fn handle_connection(mut socket: TcpStream) {
    let mut buf = [0; 1024];

    loop {
        let n = match socket.read(&mut buf).await {
            Ok(0) => return, // connection closed
            Ok(n) => n,
            Err(_) => return,
        };

        // For demo: just echo back what we got
        if socket.write_all(&buf[0..n]).await.is_err() {
            return;
        }
    }
}

pub async fn run_server() -> tokio::io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080").await?;
    println!("Listening on 127.0.0.1:8080");

    loop {
        let (socket, _) = listener.accept().await?;
        tokio::spawn(handle_connection(socket));
    }
}

pub fn start_networking() {
    tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .block_on(run_server());
}
