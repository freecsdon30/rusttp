use crate::enums::{http_version, method};
use httparse::Header;

#[derive(Debug)]
pub struct Request<'a> {
    pub method: method::RequestMethod,
    pub path: &'a str,
    pub version: http_version::HttpVersion,
    pub headers: Vec<Header<'a>>,
    pub body: &'a [u8],
}
