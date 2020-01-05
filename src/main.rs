#![windows_subsystem = "windows"]

use web_view::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let mut url = "https://gopher.commons.host/".to_string();
    if args.len() > 1 {
        let mut target = args[1].to_string();
        if !target.starts_with("gopher://") {
            target = format!("gopher://{}", target);
        }
        url.push_str(&target);
    }

    web_view::builder()
        .title("Gopher - Commons Host")
        .content(Content::Url(url))
        .size(800, 600)
        .resizable(true)
        .debug(true)
        .user_data(())
        .invoke_handler(|_webview, _arg| Ok(()))
        .run()
        .unwrap();
}