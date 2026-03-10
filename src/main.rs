use std::io::{Read, Write};
use std::net::TcpListener;
use std::time::{Instant, SystemTime, UNIX_EPOCH};

/// Format a `SystemTime` as `YYYY-MM-DD HH:MM:SS.mmmZ` (UTC) using only std.
fn format_time(t: SystemTime) -> String {
    // Falls back to epoch (1970-01-01) if the system clock is set before the Unix epoch.
    let dur = t.duration_since(UNIX_EPOCH).unwrap_or_default();
    let secs = dur.as_secs();
    let millis = dur.subsec_millis();

    let sec = (secs % 60) as u32;
    let min = ((secs / 60) % 60) as u32;
    let hour = ((secs / 3600) % 24) as u32;
    let mut days = (secs / 86400) as u32;

    let mut year = 1970u32;
    loop {
        let days_in_year = if is_leap(year) { 366 } else { 365 };
        if days < days_in_year {
            break;
        }
        days -= days_in_year;
        year += 1;
    }

    let month_lengths = [
        31u32,
        if is_leap(year) { 29 } else { 28 },
        31,
        30,
        31,
        30,
        31,
        31,
        30,
        31,
        30,
        31,
    ];
    let mut month = 1u32;
    for &ml in &month_lengths {
        if days < ml {
            break;
        }
        days -= ml;
        month += 1;
    }
    let day = days + 1;

    format!("{year:04}-{month:02}-{day:02} {hour:02}:{min:02}:{sec:02}.{millis:03}Z")
}

fn is_leap(year: u32) -> bool {
    (year.is_multiple_of(4) && !year.is_multiple_of(100)) || year.is_multiple_of(400)
}

fn main() {
    let addr = "[::]:8080";
    let listener = TcpListener::bind(addr).expect("Failed to bind to [::]:8080");
    println!("Listening on http://{}/ (IPv4 + IPv6)", addr);
    eprintln!(
        "[{}] Server started — listening on http://{}/ (IPv4 + IPv6)",
        format_time(SystemTime::now()),
        addr
    );

    // Single-threaded: each connection is handled sequentially.
    // This keeps the implementation simple; scale out via multiple container replicas.
    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                let peer = stream
                    .peer_addr()
                    .map(|a| a.to_string())
                    .unwrap_or_else(|_| "unknown".to_string());
                let request_start = Instant::now();
                eprintln!(
                    "[{}] Request from {} — processing",
                    format_time(SystemTime::now()),
                    peer
                );

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

                eprintln!(
                    "[{}] Request from {} — done in {:.3}ms",
                    format_time(SystemTime::now()),
                    peer,
                    request_start.elapsed().as_micros() as f64 / 1000.0
                );
            }
            Err(e) => eprintln!("Connection error: {e}"),
        }
    }
}
