use crate::http::lines::{RequestLine, ResponseLine, StartLine};
use std::collections::HashMap;
use std::error::Error;

pub enum HttpMethod {
    Get,
}

pub type HttpBody = String;

pub struct HttpMessage {
    pub start_line: StartLine,

    pub headers: HashMap<String, String>,

    pub body: Option<HttpBody>,
}

impl HttpMessage {
    pub fn new(
        start_line: RequestLine,
        headers: HashMap<String, String>,
        body: Option<HttpBody>,
    ) -> HttpMessage {
        // check general header fields again in 4.5
        if let Some(body) = body {
            if !HttpMessage::is_body_allowed(&start_line.method, &headers) {
                panic!("Body not allowed for this request");
            }

            HttpMessage {
                start_line: StartLine::Request(start_line),
                headers,
                body: Some(body),
            }
        } else {
            // TODO: is_body_required
            HttpMessage {
                start_line: StartLine::Request(start_line),
                headers,
                body: None,
            }
        }
    }

    pub fn parse_response(response: String) -> Result<HttpMessage, Box<dyn Error>> {
        // seperate by CRLF
        let mut lines = response.split("\r\n");
        let response_line =
            ResponseLine::parse(lines.next().ok_or("malformed response")?.to_string())?;
        println!(
            "response line: {} - {}",
            response_line.status_code, response_line.reason_phrase
        );

        let mut headers = HashMap::new();
        for line in &mut lines {
            if line.is_empty() | line.starts_with("\r\n") {
                break;
            }
            let mut key_value: Vec<&str> = line.split(": ").collect();
            if key_value.len() != 2 {
                return Err("malformed header".into());
            }
            headers.insert(
                key_value.remove(0).to_string(),
                key_value.remove(0).to_string(),
            );
        }

        // collect remaining lines into body as string
        let body = lines.collect::<Vec<&str>>().join("\r\n\r\n");

        Ok(HttpMessage {
            start_line: StartLine::Response(response_line),
            headers,
            body: Some(body),
        })
    }

    fn is_body_allowed(method: &HttpMethod, _headers: &HashMap<String, String>) -> bool {
        match method {
            HttpMethod::Get => false,
        }
        // - check for content-length or transfer-encoding to signal body presence
        // - if receiving a message:
        //   - 1xx, 204 and 304 responses must not include a body
    }

    pub fn to_string(&self) -> String {
        let mut headers_string = String::new();
        for (key, value) in &self.headers {
            headers_string.push_str(&format!("{}: {}\n", key, value));
        }

        if self.body.is_none() {
            if let StartLine::Request(request_line) = &self.start_line {
                format!("{}{}\n", request_line.to_string(), headers_string)
            } else {
                panic!("Invalid Request line");
            }
        } else {
            "WIP".to_string()
        }
    }
}
