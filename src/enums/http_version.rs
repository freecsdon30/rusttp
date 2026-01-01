use std::convert::TryFrom;

#[derive(Debug)]
pub enum HttpVersion {
    Http1,
    Http2,
    Http3,
}

impl TryFrom<u8> for HttpVersion {
    type Error = &'static str;

    fn try_from(value: u8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(Self::Http1),
            2 => Ok(Self::Http2),
            3 => Ok(Self::Http3),
            _ => Err("Invalid Http Version"),
        }
    }
}
