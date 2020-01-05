# 🌎 gogo 

WebKit-based Gopher browser. Inspired by
https://github.com/Lartu/OpenNapkin.

Uses https://gopher.commons.host/ and your operating system's WebView
to browse Gopher via https://github.com/Boscop/web-view.

## installation

Standalone binaries are coming soon. For now you have to build from
source. Assumng you have Rust/Cargo and Git installed:

    git clone https://github.com/dvkt/gogo
    cd gogo
    cargo install --path .
    gogo <gopher-url>

## screenies

|![Screenshot](./img/rpod.png)|![Screenshot](./img/hn.png)|
|:-:|:-:|
| RPoD | hngopher.com |

## todo

- [ ] back/forward
- [ ] new window
- [ ] ...tabs?
- [ ] bookmarks
- [ ] history
- [ ] share history and bookmarks format with phetch
- [ ] use local server instead of gopher.commons.host

