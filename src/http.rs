use crate::dns;
use std::{
    error::Error,
    io::prelude::*,
    net::{Ipv4Addr, TcpStream},
};

use self::message::HttpMessage;

mod get;
mod lines;
mod message;

fn send_http_message(
    address: Ipv4Addr,
    port: u16,
    request_message: HttpMessage,
) -> Result<String, Box<dyn Error>> {
    let mut stream = TcpStream::connect((address, port))?;

    let request = request_message.to_string();
    println!("sending request:");
    println!("{}", request);
    let sent_len = stream.write(request.as_bytes())?;
    stream.flush()?;
    println!("Sent {} bytes", sent_len);

    let mut buf = String::new();
    let recv_len = stream.read_to_string(&mut buf)?;
    println!("Read {} bytes", recv_len);

    // apparently this causes issues?
    // stream.shutdown(Shutdown::Both)?;

    Ok(buf)
}

pub fn request_content(web_addr: &String) -> Result<HttpMessage, Box<dyn Error>> {
    let ip = dns::lookup(web_addr)?;
    println!("DNS resolved IP: {}", ip);

    let request = get::new_get_request(web_addr.to_string());
    let raw_response = send_http_message(ip, 80, request)?;
    let response = HttpMessage::parse_response(raw_response)?;

    Ok(response)
}
