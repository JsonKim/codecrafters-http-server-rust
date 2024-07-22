// Uncomment this block to pass the first stage
use std::{
    io::Error,
    io::Write,
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream, message: &str) -> Result<(), Error> {
    stream.write(message.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("accepted new connection");
                if let Err(e) = handle_client(_stream, "HTTP/1.1 200 OK\r\n\r\n") {
                    eprintln!("Error handling client: {}", e);
                }
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
