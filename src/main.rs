#![windows_subsystem = "windows"]

use gogo::{server, Result};
use std::{net::TcpListener, thread};
use web_view::*;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();

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
            "-s" | "--server" | "-server" => {
                return run_server();
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

    let listener = TcpListener::bind("0.0.0.0:0")?;
    let mut url = format!("http://{}/", listener.local_addr()?);

    if args.len() > 1 {
        let mut target = args[1].to_string();
        if !target.starts_with("gopher://") {
            target = format!("gopher://{}", target);
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

fn run_server() -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:0")?;

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

    -s, --server    Just start as HTTP server, no UI.

Other flags:

    -h, --help      Print this screen.
    -v, --version   Print gogo version."
    );
}

fn print_version() {
    println!("gogo v{}", env!("CARGO_PKG_VERSION"));
}
