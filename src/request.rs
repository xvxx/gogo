pub struct Request {
    pub path: String,
}

impl Request {
    /// Creates a new Request from a raw HTTP request.
    pub fn from(line: &str) -> Request {
        Request {
            path: path_from_line(line),
        }
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
