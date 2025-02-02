use std::fs::File;
use std::io::{self, Read, Write, BufReader};
use std::net::{TcpListener, TcpStream};

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let mut buffer = [0; 1024];
    let bytes_read = stream.read(&mut buffer)?;

    let request = String::from_utf8_lossy(&buffer[..bytes_read]);

    println!("Received request: {}", request);

    // Default to index.html if the request is for "/"
    let file_path = if request.starts_with("GET / ") || request.starts_with("GET / HTTP") {
        "index.html"
    } else {
        "404.html" // Optional: Handle unknown paths with a 404 response
    };

    let file = File::open(file_path).unwrap_or_else(|_| File::open("404.html").unwrap());
    let mut reader = BufReader::new(file);

    // Send HTTP response headers
    writeln!(stream, "HTTP/1.1 200 OK\r\n\r")?;

    // Stream file contents to client
    io::copy(&mut reader, &mut stream)?;

    Ok(())
}

fn main() -> io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:8080")?;
    println!("Listening on 8080");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection!");
                if let Err(e) = handle_client(stream) {
                    eprintln!("Error handling client: {}", e);
                }
            }
            Err(e) => eprintln!("Connection failed: {}", e),
        }
    }

    Ok(())
}
