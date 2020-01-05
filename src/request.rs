use std::net::SocketAddr;

pub struct Request {
    pub addr: SocketAddr,
    pub path: String,
}

impl Request {
    /// Creates a new Request from a socket address..
    pub fn from(addr: SocketAddr) -> Request {
        Request {
            addr,
            path: String::new(),
        }
    }

    /// Parse HTTP request line to fill out this Request.
    pub fn parse(&mut self, line: &str) {
        self.path = path_from_line(line);
    }
}

/// Given an HTTP request line, returns just the path requested.
fn path_from_line(line: &str) -> String {
    let mut out = String::new();
    if line.starts_with("GET ") {
        if let Some(end) = line.find(" HTTP/1.1") {
            out.push_str(&line[5..end]);
        }
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_from_line() {
        assert_eq!("", path_from_line("GET / HTTP/1.1"));
        assert_eq!("dawg", path_from_line("GET /dawg HTTP/1.1"));
        assert_eq!("users/414", path_from_line("GET /users/414 HTTP/1.1"));
        assert_eq!("", path_from_line("GET /users/414 HTTP/1.0"));
        assert_eq!("", path_from_line("  get /users/414 http/1.1"));
        assert_eq!("", path_from_line("POST /users HTTP/1.1"));
        assert_eq!(
            "()#)%# #%) *# )#",
            path_from_line("GET /()#)%# #%) *# )# HTTP/1.1")
        );
    }
}
