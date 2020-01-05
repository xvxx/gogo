#![windows_subsystem = "windows"]

use gogo::{server, Result};
use std::{net::TcpListener, thread};
use web_view::*;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let mut url = "https://gopher.commons.host/".to_string();
    if args.len() > 1 {
        let mut target = args[1].to_string();
        if !target.starts_with("gopher://") {
            target = format!("gopher://{}", target);
        }
        url.push_str(&target);
    }

    let listener = TcpListener::bind("0.0.0.0:0")?;
    let addr = format!("http://{}", listener.local_addr()?);

    thread::spawn(move || {
        if let Err(e) = server::start(listener) {
            eprintln!("{}", e);
        }
    });

    web_view::builder()
        .title("gogo")
        .content(Content::Url(addr))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();

    Ok(())
}
