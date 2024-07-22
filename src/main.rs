// Uncomment this block to pass the first stage
use std::{
    io::{Error, Read, Write},
    net::{TcpListener, TcpStream},
};

fn handle_client(mut stream: TcpStream, data: &str) -> Result<(), Error> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;
    let message = String::from_utf8_lossy(&buffer[..bytes_read]);
    let lines: Vec<&str> = message.split("\r\n").collect();
    if let Some(first_line) = lines.first() {
        let parts: Vec<&str> = first_line.split_whitespace().collect();

        if parts[1] == "/" {
            stream.write(data.as_bytes())?;
            stream.flush()?;
        } else {
            stream.write("HTTP/1.1 404 Not Found\r\n\r\n".as_bytes())?;
            stream.flush()?;
        }
    }

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
