#![windows_subsystem = "windows"]

use {
    phroxy,
    std::{net::TcpListener, thread},
    web_view::*,
};

static DEFAULT_GOPHERHOLE: &str = "gopher://phroxy.net";

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn main() -> Result<()> {
    let args: Vec<_> = std::env::args().skip(1).collect();
    let mut port = 0;
    let mut url_arg = "";
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
                if let Some('-') = arg.chars().nth(0) {
                    eprintln!("Unknown option: {}", arg);
                    std::process::exit(1);
                } else {
                    url_arg = arg;
                }
            }
        }
    }

    if port > 0 && !server_only {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::Other,
            "--port can only be used with --server",
        )));
    }

    let port = if server_only { port } else { 0 };
    let listener = TcpListener::bind(format!("0.0.0.0:{}", port))?;
    let mut url = format!("http://{}/", listener.local_addr()?);
    if !url_arg.is_empty() {
        if !url_arg.starts_with("gopher://") {
            url.push_str("gopher://");
        }
        url.push_str(&url_arg);
    }

    if server_only {
        run_server(listener)?;
    } else {
        thread::spawn(move || run_server(listener));
    }

    web_view::builder()
        .title("gogo")
        .content(Content::Url(url))
        .size(1024, 768)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();

    Ok(())
}

fn run_server(listener: TcpListener) -> Result<()> {
    if let Err(e) = phroxy::server::start(listener, DEFAULT_GOPHERHOLE) {
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
