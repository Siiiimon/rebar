use std::fmt::{self, Debug, Formatter};

pub struct DnsHeader {
    pub id: u16,
    pub header: u16,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl DnsHeader {
    pub fn new_question() -> DnsHeader {
        DnsHeader {
            id: 1,
            header: 0b0_0000_0_1_0_0,
            qdcount: 1,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    // TODO: very ugly, just turn the struct into a byte array
    pub fn to_u8_array(&self) -> [u8; 12] {
        [
            self.id.to_be_bytes()[0],
            self.id.to_be_bytes()[1],
            self.header.to_be_bytes()[0],
            self.header.to_be_bytes()[1],
            self.qdcount.to_be_bytes()[0],
            self.qdcount.to_be_bytes()[1],
            self.ancount.to_be_bytes()[0],
            self.ancount.to_be_bytes()[1],
            self.nscount.to_be_bytes()[0],
            self.nscount.to_be_bytes()[1],
            self.arcount.to_be_bytes()[0],
            self.arcount.to_be_bytes()[1],
        ]
    }

    pub fn from_u8_array(h: Vec<u8>) -> DnsHeader {
        DnsHeader {
            id: ((h[0] as u16) << 8) | h[1] as u16,
            header: ((h[2] as u16) << 8) | h[3] as u16,
            qdcount: ((h[4] as u16) << 8) | h[5] as u16,
            ancount: ((h[6] as u16) << 8) | h[7] as u16,
            nscount: ((h[8] as u16) << 8) | h[9] as u16,
            arcount: ((h[10] as u16) << 8) | h[11] as u16,
        }
    }
}

impl Debug for DnsHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let qr = match self.header & 0b1000_0000_0000_0000 {
            0 => "query",
            _ => "response",
        };
        let opcode = match self.header & 0b0111_1000_0000_0000 {
            0 => "QUERY",
            1 => "IQUERY",
            2 => "STATUS",
            _ => "reserved",
        };
        let aa = match self.header & 0b0000_0100_0000_0000 {
            0 => "non-authoritative",
            _ => "authoritative",
        };
        let tc = match self.header & 0b0000_0010_0000_0000 {
            0 => "not truncated",
            _ => "truncated",
        };
        let rd = match self.header & 0b0000_0001_0000_0000 {
            0 => "recursion not desired",
            _ => "recursion desired",
        };
        let ra = match self.header & 0b0000_0000_1000_0000 {
            0 => "recursion not available",
            _ => "recursion available",
        };
        let rcode = match self.header & 0b0000_0000_0000_1111 {
            0 => "no error",
            1 => "format error",
            2 => "server failure",
            3 => "name error",
            4 => "not implemented",
            5 => "refused",
            _ => "reserved",
        };

        let header_formatted = format!(
            "\n\tqr: {},\n\topcode: {},\n\taa: {},\n\ttc: {},\n\trd: {},\n\tra: {},\n\trcode: {}",
            qr, opcode, aa, tc, rd, ra, rcode
        );
        write!(
            f,
            "id: {:x},\nheader: {},\nqdcount: {:x},\nancount: {:x},\nnscount: {:x},\narcount: {:x}",
            self.id, header_formatted, self.qdcount, self.ancount, self.nscount, self.arcount
        )
    }
}
