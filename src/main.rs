use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    // Create a tcp server with localhost:8989
    let listener = TcpListener::bind("127.0.0.1:8989").unwrap();
    println!("Start listening...");
    // Iterate through each connection's stream
    for stream in listener.incoming() {
        // Check stream result
        match stream {
            // Hanldle successfully connected
            Ok(stream) => handle_connection(stream),
            // Handle error 
            Err(e) => panic!("Connection with error: {}", e),
        }
    }
}

fn handle_connection(mut stream: TcpStream) {
    // Init a buffer
    let mut buffer = [0; 1024];
    // Receive multiple round messages
    loop {
        // Check client's message
        match stream.read(&mut buffer) {
            Ok(size) => {
                // Print the data from client for debugging
                println!("Received data: {}", String::from_utf8_lossy(&buffer[0..size]));
                // Send back to the client
                stream.write(&buffer[0..size]).unwrap();
            },
            Err(e) => {
                // Quit app when read error
                panic!("Read data with error: {}", e);
            },
        }
    }
}