mod header;
mod question;
mod resource;

use header::DnsHeader;
use question::address_to_question;
use resource::ResourceRecord;
use std::net::{self, Ipv4Addr, SocketAddrV4};

use crate::dns::resource::RecordType;

fn send_dns_message(
    destination: net::SocketAddrV4,
    content: &[u8],
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let socket = net::UdpSocket::bind("0.0.0.0:34254")?;
    socket.connect(destination)?;

    print!("sending... ");
    let sent_bytes = socket.send(content)?;
    println!("sent {} bytes", sent_bytes);

    let mut buf: [u8; 512] = [0; 512];
    let recv_bytes = socket.recv(&mut buf)?;
    println!("recv {} bytes", recv_bytes);

    Ok(buf.to_vec())
}

fn decode_dns_message(message: Vec<u8>) -> Result<Ipv4Addr, Box<dyn std::error::Error>> {
    let header = DnsHeader::from_u8_array(message[0..12].to_vec());
    println!("header:");
    println!("{:?}", header);

    // skip question sections
    let mut index = 0;
    for _ in 0..header.qdcount {
        index += 12; // skip the next question sections 12-byte header
                     // skip qname
        while message[index] != 0 {
            index += message[index] as usize + 1;
        }
        index += 5; // skip the null byte and the qtype and qclass
    }

    let answer = ResourceRecord::from_u8_array(message[index..].to_vec())?;
    match answer.r#type {
        RecordType::A => Ok(Ipv4Addr::new(
            answer.rdata[0] as u8,
            answer.rdata[1] as u8,
            answer.rdata[2] as u8,
            answer.rdata[3] as u8,
        )),
        _ => panic!("Unknown record type"),
    }
}

pub fn lookup(address: &String) -> Result<Ipv4Addr, Box<dyn std::error::Error>> {
    let header = DnsHeader::new_question();
    let question_section = address_to_question(address)?;

    let mut message = Vec::new();
    message.extend(header.to_u8_array());
    message.extend(question_section);

    let response = send_dns_message(
        SocketAddrV4::new(Ipv4Addr::new(192, 168, 2, 1), 53),
        message.as_slice(),
    )?;

    decode_dns_message(response)
}
