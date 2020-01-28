# 🌎 gogo 

WebKit-based Gopher desktop client. 

Inspired by https://github.com/Lartu/OpenNapkin and
https://gopher.commons.host, gogo uses [phroxy][phroxy] and your
operating system's native [WebView][webview] to browse the world of
Gopher in style.

## installation

Standalone binaries are coming soon. For now you have to build from
source. 

On Linux, you must first install webkit2gtk:

    sudo apt install webkit2gtk-4.0

Then, assuming you have Rust/Cargo and Git installed:

    git clone https://github.com/xvxx/gogo
    cd gogo
    cargo install --path .
    gogo <gopher-url>

## screenies

|![Screenshot](./img/rpod.png)|![Screenshot](./img/hn.png)|
|:-:|:-:|
| RPoD | hngopher.com |

## todo

- [ ] keyboard shortcuts
- [ ] new window
- [ ] ...tabs?
- [ ] bookmarks
- [ ] history
- [ ] share history and bookmarks format with phetch

[phroxy]: https://github.com/xvxx/phroxy
https://github.com/boscop/web-view