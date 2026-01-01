use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
    net::TcpStream,
};

pub fn handle_connections(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);

    let http_request: String = buf_reader
        .lines()
        .map(|result| result.expect("Failed to read line"))
        .take_while(|line| !line.is_empty())
        .collect::<Vec<String>>()
        .join("\r\n")
        + "\r\n\r\n";

    let mut headers = [httparse::EMPTY_HEADER; 32];
    let mut req = httparse::Request::new(&mut headers);

    let status = req.parse(http_request.as_bytes()).unwrap_or_else(|e| {
        eprintln!("error : {}", e);
        panic!("request parsing error");
    });

    let mut headers_map = HashMap::new();

    for header in req.headers.iter().filter(|h| !h.name.is_empty()) {
        let name = header.name.to_string();
        let value = String::from_utf8_lossy(header.value).to_string();
        headers_map.insert(name, value);
    }

    if status.is_complete() {
        if let (Some(v), Some(m), Some(p)) = (req.version, req.method, req.path) {
            println!("VERSION: {:?}", v);
            println!("METHOD: {:?}", m);
            println!("PATH: {:?}", p);
            println!("HEADERS: {:#?}", headers_map);
        }
    }
}
