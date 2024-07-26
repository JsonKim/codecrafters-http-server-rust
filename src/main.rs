// Uncomment this block to pass the first stage
use std::{
    collections::HashMap, env, fs::File, io::{Error, Read, Write}, net::{TcpListener, TcpStream}, path::Path
};

enum HttpMethod {
    GET,
    POST,
}

enum RouteContent<'a> {
    Index,
    Echo(&'a str),
    GetFile(&'a str),
    PostFile(&'a str),
    UserAgent,
    NotFound,
}

fn parse_route<'a>(method: HttpMethod, input: &'a str) -> RouteContent {
    match (method, input) {
        (HttpMethod::GET, "/") => RouteContent::Index,
        (HttpMethod::GET, "/user-agent") => RouteContent::UserAgent,
        (HttpMethod::GET, _) if input.starts_with("/echo/") => RouteContent::Echo(&input[6..]),
        (HttpMethod::GET, _) if input.starts_with("/files/") => RouteContent::GetFile(&input[7..]),
        (HttpMethod::POST, _) if input.starts_with("/files/") => RouteContent::PostFile(&input[7..]),
        _ => RouteContent::NotFound,
    }
}

fn parse_text_to_map(text: &str) -> HashMap<String, String> {
    text
        .split("\r\n\r\n")
        .next()
        .unwrap_or("")
        .lines()
        .skip(1)
        .filter_map(|line| {
            let parts: Vec<&str> = line.splitn(2, ": ").collect();
            if parts.len() == 2 {
                Some((parts[0].trim().to_string(), parts[1].trim().to_string()))
            } else {
                None
            }
        })
        .collect()
}

fn parse_body(text: &str) -> &str {
    text.split("\r\n\r\n")
        .nth(1)
        .unwrap_or("")
}

fn get_path(request: &str) -> &str {
    let first_line = request.lines().next().unwrap();
    first_line.split_whitespace().nth(1).unwrap()
}

fn get_method(request: &str) -> Option<HttpMethod> {
    let first_line = request.lines().next().unwrap();
    let method = first_line.split_whitespace().nth(0)?;
    match method {
        "GET" => Some(HttpMethod::GET),
        "POST" => Some(HttpMethod::POST),
        _ => None,
    }
}

fn handle_client(mut stream: TcpStream, data: &str) -> Result<(), Error> {
    stream.write(data.as_bytes())?;
    stream.flush()?;
    Ok(())
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    let args: Vec<String> = env::args().collect();
    // Uncomment this block to pass the first stage

    let listener = TcpListener::bind("127.0.0.1:4221").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let directory = if args.len() == 3 { args[2].clone() } else { ".".to_string() };
                std::thread::spawn(move || {
                    println!("accepted new connection");
    
                    let mut buffer = [0; 1024];
                    let bytes_read = stream.read(&mut buffer).unwrap();
                    let request = String::from_utf8_lossy(&buffer[..bytes_read]);
                    let method = get_method(&request);
                    let path = get_path(&request);
                    let headers = parse_text_to_map(&request);
    
                    let message = method.map(|method| match parse_route(method, path) {
                        RouteContent::Index => "HTTP/1.1 200 OK\r\n\r\n".to_string(),
                        RouteContent::Echo(content) => 
                            format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", content.len(), content),
                        RouteContent::GetFile(file) => {
                            let file_path = Path::new(&directory).join(file); // Use the cloned directory variable
                            match std::fs::read_to_string(file_path) {
                                Ok(content) => format!("HTTP/1.1 200 OK\r\nContent-Type: application/octet-stream\r\nContent-Length: {}\r\n\r\n{}", content.len(), content),
                                Err(_) => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
                            }
                        },
                        RouteContent::PostFile(file) => {
                            let body = parse_body(&request);
                            let file_path = Path::new(&directory).join(file); // Use the cloned directory variable

                            File::create(file_path)
                                .unwrap()
                                .write_all(body.as_bytes())
                                .unwrap();

                            "HTTP/1.1 201 Created\r\n\r\n".to_string()
                        },
                        RouteContent::NotFound => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
                        RouteContent::UserAgent => match headers.get("User-Agent") {
                            Some(user_agent) => format!("HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\n\r\n{}", user_agent.len(), user_agent),
                            None => "HTTP/1.1 404 Not Found\r\n\r\n".to_string(),
                        }
                    })
                    .unwrap_or("HTTP/1.1 404 Not Found\r\n\r\n".to_string());
    
                    if let Err(e) = handle_client(stream, message.as_str()) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}
