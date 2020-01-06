#![windows_subsystem = "windows"]

use gogo::{server, Result};
use std::{net::TcpListener, thread};
use web_view::*;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut port = 0;

    let mut server_only = false;
    let mut iter = args.iter();
    while let Some(arg) = iter.next() {
        match arg.as_ref() {
            "-v" | "--version" | "-version" => {
                print_version();
                return Ok(());
            }
            "-h" | "--help" | "-help" => {
                print_help();
                return Ok(());
            }
            "-p" | "--port" | "-port" => {
                if let Some(p) = iter.next() {
                    port = p.parse().unwrap_or(0);
                }
            }
            "-s" | "--server" | "-server" => {
                server_only = true;
            }
            arg => {
                if !arg.is_empty() {
                    if let Some('-') = arg.chars().nth(0) {
                        eprintln!("Unknown option: {}", arg);
                        std::process::exit(1);
                    }
                }
            }
        }
    }
    if server_only {
        return run_server(port);
    }

    let listener = TcpListener::bind("0.0.0.0:0")?;
    let mut url = format!("http://{}/", listener.local_addr()?);

    if let Some(target) = iter.next() {
        if !target.starts_with("gopher://") {
            url.push_str("gopher://");
        }
        url.push_str(&target);
    }

    thread::spawn(move || {
        if let Err(e) = server::start(listener) {
            eprintln!("{}", e);
        }
    });

    web_view::builder()
        .title("gogo")
        .content(Content::Url(url))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();

    Ok(())
}

fn run_server(port: usize) -> Result<()> {
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;

    if let Err(e) = server::start(listener) {
        eprintln!("{}", e);
    }

    Ok(())
}

fn print_help() {
    println!(
        "Usage:

    gogo [options] <gopher-url>

Options:

    -s, --server      Just start as HTTP server, no UI.
    -p, --port [NUM]  Set the server's port. Only works with -s.

Other flags:

    -h, --help        Print this screen.
    -v, --version     Print gogo version."
    );
}

fn print_version() {
    println!("gogo v{}", env!("CARGO_PKG_VERSION"));
}
