use crate::Result;
use std::{
    io::{prelude::*, BufReader, Read, Write},
    net::{TcpListener, TcpStream},
};
use threadpool::ThreadPool;

/// This is only for running locally, so only allow a low number of
/// simultaneous connections.
const MAX_WORKERS: usize = 10;

/// Starts a web server locally.
pub fn start(listener: TcpListener) -> Result<()> {
    let pool = ThreadPool::new(MAX_WORKERS);
    let addr = listener.local_addr()?;

    println!("┌ Listening at {}", addr);
    for stream in listener.incoming() {
        let stream = stream?;
        println!("┌ Connection from {}", stream.peer_addr()?);
        pool.execute(move || {
            if let Err(e) = handle_request(stream) {
                eprintln!("└ {}", e);
            }
        });
    }
    Ok(())
}

/// Reads from the client and responds.
fn handle_request(mut stream: TcpStream) -> Result<()> {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();
    let reader = BufReader::new(buffer.as_ref());
    println!("│ {}", reader.lines().nth(0).unwrap().unwrap());
    write_response(&mut stream)?;
    Ok(())
}

/// Writes a response to a client based on a Request.
fn write_response<'a, W>(mut w: &'a W) -> Result<()>
where
    &'a W: Write,
{
    let contents = std::fs::read_to_string("./html/layout.html").unwrap();
    let contents = contents
        .replace("{{content}}", "<h1>Hiya 🦀</h1>")
        .replace("{{title}}", "Hi from Rust");
    let response = format!("HTTP/1.1 200 OK\r\n\r\n{}", contents);

    w.write(response.as_bytes()).unwrap();
    w.flush().unwrap();
    Ok(())
}
