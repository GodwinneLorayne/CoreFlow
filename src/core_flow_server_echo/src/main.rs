use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> () {
    // Log progress
    println!("Waiting for client to connect");

    let listener = TcpListener::bind("localhost:8080")
        .await
        .expect("Failed to bind to tcp socket");
    let (mut socket, _) = listener
        .accept()
        .await
        .expect("Failed to listen on tcp socket");

    // Log progress
    println!("Creating mutable buffer");

    let mut buf = [0; 1024];

    // Log progress
    println!("Entering message loop");

    // In a loop, read data from the socket and write the data back.
    loop {
        // Log progress
        println!("Reading from client");

        let n = match socket.read(&mut buf).await {
            // socket closed
            Ok(n) if n == 0 => return,
            Ok(n) => n,
            Err(e) => {
                println!("failed to read from socket; err = {:?}", e);
                return;
            }
        };

        println!("Received: {:?}", buf);

        // Log progress
        println!("Sending message back to client");

        // Write the data back
        if let Err(e) = socket.write_all(&buf[0..n]).await {
            println!("failed to write to socket; err = {:?}", e);
            return;
        }
    }
}
