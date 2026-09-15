// src/main.rs

// 1. **What do I have?** State it literally, as a fact. Data, a variable, a file, a socket, nothing. It can't be wrong.
// 2. **What's the one next thing that has to be true?** Not the goal. One thing, closer to it.
// 3. **Can I do that by hand on a tiny input?** Yes → do it, then translate. No → too big; split it, back to 2.
// **Stuck means shrink, not close.** Blank at any question is a signal the step is wrong-sized, not that you can't.
// **Two kinds of stuck:** *I don't know what* → shrink. *I don't know how* → look it up (docs, examples first). Name which one before doing anything.
// **Run it, don't judge it.** Trace on paper, compiler, print, debugger — any cheap judge beats the one in your head.

use std::collections::HashMap;
use std::io::{Read, Write};
use std::net::TcpListener;

#[derive(Debug)]
#[allow(dead_code)]
struct Request {
    method: String,
    path: String,
    version: String,
    headers: HashMap<String, String>,
    body: String,
}

#[derive(Debug)]
#[allow(dead_code)]
struct Response {
    status_line: (String, u16, String),
    headers: HashMap<String, String>,
    body: String,
}

impl std::fmt::Display for Response {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} {} {}\r\nContent-Length: {}\r\n\r\n{}",
            self.status_line.0,
            self.status_line.1,
            self.status_line.2,
            self.body.len(),
            self.body,
        )
    }
}

// function for the index route
fn index(_req: &Request) -> Response {
    Response {
        status_line: ("HTTP/1.1".to_string(), 200, "OK".to_string()),
        headers: HashMap::new(),
        body: std::fs::read_to_string("index.html").unwrap(),
    }
}

// function for the not found route
fn not_found(_req: &Request) -> Response {
    Response {
        status_line: ("HTTP/1.1".to_string(), 404, "NOT FOUND".to_string()),
        headers: HashMap::new(),
        body: "Nothing here by that name.".to_string(),
    }
}

// function which accepts a path and returns a response (status code and body)
fn route(router: &HashMap<String, fn(&Request) -> Response>, req: Request) -> Response {
    match router.get(&req.path) {
        Some(handler) => handler(&req),
        None => not_found(&req),
    }
}

// main function
fn main() -> std::io::Result<()> {
    // initialize the router
    let mut router: HashMap<String, fn(&Request) -> Response> = HashMap::new();
    router.insert("/".to_string(), index as fn(&Request) -> Response);

    // make a listener
    let listener = TcpListener::bind("127.0.0.1:8000")?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // accumulate the incoming bytes
                let mut buffer: [u8; 4096] = [0; 4096];
                let mut chunks: Vec<u8> = Vec::new();
                let header_end = loop {
                    let n = stream.read(&mut buffer)?;
                    if n == 0 {
                        break chunks.len();
                    }
                    chunks.extend_from_slice(&buffer[..n]);

                    match chunks.windows(4).position(|w| w == b"\r\n\r\n") {
                        Some(index) => break index,
                        None => continue,
                    }
                };

                // convert the raw incoming bytes into a string
                let raw_request_headers = String::from_utf8_lossy(&chunks[..header_end]);

                // get the method, path, and version
                let request_method_path_version = raw_request_headers.lines().next().unwrap();

                // get the headers
                let mut request_headers: HashMap<String, String> = HashMap::new();
                for line in raw_request_headers.lines().skip(1) {
                    let parts = line.split_once(":");
                    match parts {
                        Some((name, value)) => {
                            request_headers.insert(name.to_string(), value.trim().to_string());
                        }
                        None => break,
                    }
                }

                // split the first line into method, path, version
                let request_method_path_version_parts: Vec<&str> =
                    request_method_path_version.split(" ").collect();

                // build the request by assembling the parts
                let request = Request {
                    method: request_method_path_version_parts[0].to_string(),
                    path: request_method_path_version_parts[1].to_string(),
                    version: request_method_path_version_parts[2].to_string(),
                    headers: request_headers,
                    body: String::from_utf8_lossy(&chunks[header_end + 4..]).to_string(),
                };
                println!("{:#?}", request);

                // match on the path, which is held by `first_line_parts[1], send a `200 OK` and a
                // message for the `/` route
                // send a `404 NOT FOUND` and a message for anything else
                let response = route(&router, request);
                stream.write_all(response.to_string().as_bytes())?;
            }
            Err(e) => eprintln!("{}", e),
        }
    }
    Ok(())
}
