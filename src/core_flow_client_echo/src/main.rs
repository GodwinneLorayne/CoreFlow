use std::error::Error;
use std::io::{self, BufRead};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    // Log progress
    println!("Connecting to Server");

    // Connect to the server
    let mut stream = TcpStream::connect("localhost:8080")
        .await
        .expect("Failed to connect to server");

    // Log progress
    println!("Entering message loop");

    let stdin = io::stdin();

    // The loop where the client continuously sends requests to the server
    loop {
        // Log progress
        println!("Reading message from stdin");

        // Read message from stdin
        let mut stdin_buffer = String::new();
        let mut stdin_lock = stdin.lock();

        stdin_lock.read_line(&mut stdin_buffer)?;

        // Write the request
        stream.write_all(stdin_buffer.as_bytes()).await?;

        // Log progress
        println!("Creating mutable buf");

        // Read the server's response
        let mut buf = [0; 1024];

        // Log progress
        println!("Reading server response");

        let n = stream.read(&mut buf).await.expect("Failed to get response");

        let response = String::from_utf8(buf[0..n].to_vec()).unwrap();

        println!("Received: {:?}", response);

        // Sleep for one second
        sleep(Duration::from_secs(1)).await;
    }
}
