//! HTTP/1.1 on a std `TcpStream`: one request per connection, answered and closed.

use std::io::{self, BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::time::Duration;

/// The largest request body read: a patch is text.
pub const MAX_BODY: usize = 1 << 20;
/// The longest a request may take to arrive.
pub const READ_TIMEOUT: Duration = Duration::from_secs(5);
pub const WRITE_TIMEOUT: Duration = Duration::from_secs(10);

pub const JSON: &str = "application/json";
pub const HTML: &str = "text/html; charset=utf-8";
pub const TEXT: &str = "text/plain; charset=utf-8";

/// The header that names the caller a request is sent as, `User` or `Agent`: the `kind` of
/// `studio.session.User` and `studio.session.Agent`. The agent channel sends `Agent`.
pub const CALLER_HEADER: &str = "X-Studio-Caller";

#[derive(Debug, Clone, Default)]
pub struct Request {
    pub method: String,
    /// The path without its query.
    pub path: String,
    pub query: String,
    pub body: Vec<u8>,
    /// The caller the request is sent as (its [`CALLER_HEADER`]); `None` when it names none,
    /// which is the user in the browser.
    pub caller: Option<String>,
}

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub content_type: &'static str,
    pub body: Vec<u8>,
}

impl Response {
    pub fn new(status: u16, content_type: &'static str, body: impl Into<Vec<u8>>) -> Self {
        Response {
            status,
            content_type,
            body: body.into(),
        }
    }

    pub fn json(status: u16, value: &serde_json::Value) -> Self {
        Response::new(status, JSON, value.to_string())
    }
}

pub fn reason(status: u16) -> &'static str {
    match status {
        200 => "OK",
        201 => "Created",
        400 => "Bad Request",
        403 => "Forbidden",
        404 => "Not Found",
        405 => "Method Not Allowed",
        409 => "Conflict",
        413 => "Payload Too Large",
        422 => "Unprocessable Content",
        _ => "Internal Server Error",
    }
}

fn bad(message: &str) -> io::Error {
    io::Error::new(io::ErrorKind::InvalidData, message.to_string())
}

/// Reads one request: its line, its headers and a body of `Content-Length` bytes.
pub fn read(stream: &TcpStream) -> io::Result<Request> {
    let mut reader = BufReader::new(stream);
    let mut line = String::new();
    reader.read_line(&mut line)?;
    let mut words = line.split_whitespace();
    let (Some(method), Some(target)) = (words.next(), words.next()) else {
        return Err(bad("no request line"));
    };
    let (path, query) = target.split_once('?').unwrap_or((target, ""));
    let mut length = 0usize;
    let mut caller = None;
    loop {
        let mut header = String::new();
        if reader.read_line(&mut header)? == 0 {
            break;
        }
        let header = header.trim_end();
        if header.is_empty() {
            break;
        }
        if let Some((k, v)) = header.split_once(':') {
            let k = k.trim();
            if k.eq_ignore_ascii_case("content-length") {
                length = v.trim().parse().map_err(|_| bad("bad content-length"))?;
            } else if k.eq_ignore_ascii_case(CALLER_HEADER) {
                caller = Some(v.trim().to_string());
            }
        }
    }
    if length > MAX_BODY {
        return Err(bad("body too large"));
    }
    let mut body = vec![0; length];
    reader.read_exact(&mut body)?;
    Ok(Request {
        method: method.to_string(),
        path: path.to_string(),
        query: query.to_string(),
        body,
        caller,
    })
}

pub fn write(stream: &mut TcpStream, response: &Response) -> io::Result<()> {
    let head = format!(
        "HTTP/1.1 {} {}\r\nContent-Type: {}\r\nContent-Length: {}\r\nCache-Control: no-store\r\nConnection: close\r\n\r\n",
        response.status,
        reason(response.status),
        response.content_type,
        response.body.len()
    );
    stream.write_all(head.as_bytes())?;
    stream.write_all(&response.body)?;
    stream.flush()
}
