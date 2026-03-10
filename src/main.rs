use std::io::{Read, Write};
use std::net::TcpListener;

fn main() {
    let addr = "[::]:8080";
    let listener = TcpListener::bind(addr).expect("Failed to bind to [::]:8080");
    println!("Listening on http://{}/ (IPv4 + IPv6)", addr);

    // Single-threaded: each connection is handled sequentially.
    // This keeps the implementation simple; scale out via multiple container replicas.
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let mut buf = [0u8; 4096];
                if let Err(e) = stream.read(&mut buf) {
                    eprintln!("Read error: {e}");
                    continue;
                }

                let body = "Hello from simple_webserver_8080!\n";
                let response = format!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/plain\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}",
                    body.len(),
                    body
                );
                if let Err(e) = stream.write_all(response.as_bytes()) {
                    eprintln!("Write error: {e}");
                }
            }
            Err(e) => eprintln!("Connection error: {e}"),
        }
    }
}
