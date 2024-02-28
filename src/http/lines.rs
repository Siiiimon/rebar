use super::message::HttpMethod;
use std::error::Error;

pub enum StartLine {
    Request(RequestLine),
    Response(ResponseLine),
}

// lots of uri stuff (5.1.2, 5.2), but we can ignore that for now
pub struct RequestLine {
    pub method: HttpMethod,
    pub uri: String,
}

impl RequestLine {
    const HTTP_VERSION: &str = "HTTP/1.1";
    pub fn new(method: HttpMethod, uri: String) -> RequestLine {
        RequestLine { method, uri }
    }

    pub fn to_string(&self) -> String {
        let method_string = match self.method {
            HttpMethod::Get => "GET",
        };
        format!(
            "{} {} {}\n",
            method_string,
            self.uri,
            RequestLine::HTTP_VERSION
        )
    }
}

pub struct ResponseLine {
    pub version: String,
    pub status_code: u16,
    pub reason_phrase: String,
}

impl ResponseLine {
    pub fn parse(resp: String) -> Result<ResponseLine, Box<dyn Error>> {
        let mut parts = resp.split_whitespace();
        let version = parts
            .next()
            .ok_or("No version in Response Line")?
            .to_string();
        let status_code = parts
            .next()
            .ok_or("No status code in Response Line")?
            .parse::<u16>()?;
        let reason_phrase = parts.collect::<Vec<&str>>().join(" ");
        Ok(ResponseLine {
            version,
            status_code,
            reason_phrase,
        })
    }
}
