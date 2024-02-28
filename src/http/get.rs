use crate::http::lines;
use crate::http::message;
use std::collections::HashMap;

pub fn new_get_request(uri: String) -> message::HttpMessage {
    let start_line = lines::RequestLine::new(message::HttpMethod::Get, "/".to_string());
    // check request header fields in 5.3
    // TODO: add functionality to pass in headers
    let mut headers = HashMap::new();
    headers.insert("Host".to_string(), uri); // FIXME: will need to properly parse uri to get host, also add port
    headers.insert("User-Agent".to_string(), "rebar/WIP".to_string());

    message::HttpMessage::new(start_line, headers, None)
}
