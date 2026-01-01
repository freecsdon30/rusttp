use std::{
    io::{BufRead, BufReader},
    net::TcpStream,
    str::FromStr,
};

use crate::enums::{http_version::HttpVersion, method::RequestMethod};
use crate::request::request;

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
    if status.is_complete() {
        if let (Some(v), Some(m), Some(p)) = (req.version, req.method, req.path) {
            let headers_vec: Vec<httparse::Header> = req
                .headers
                .iter()
                .filter(|h| !h.name.is_empty())
                .map(|h| *h)
                .collect();

            let request = request::Request {
                version: HttpVersion::try_from(v).expect("invalid or unsupported http version..!!"),
                method: RequestMethod::from_str(m).expect("invalid or unsupported method..!!"),
                path: p,
                headers: headers_vec,
                body: &[],
            };
            println!("REQUEST: {:#?}", request);
        }
    }
}
