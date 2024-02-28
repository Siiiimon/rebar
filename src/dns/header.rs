use std::fmt::{self, Debug, Formatter};

use super::flags::Flags;

pub struct DnsHeader {
    pub id: u16,
    pub flags: u16,
    pub qdcount: u16,
    pub ancount: u16,
    pub nscount: u16,
    pub arcount: u16,
}

impl DnsHeader {
    pub fn new_question(flags: u16) -> DnsHeader {
        DnsHeader {
            id: 1,
            flags: 0 as u16 | flags,
            qdcount: 1,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }

    // TODO: very ugly
    pub fn to_u8_array(&self) -> [u8; 12] {
        [
            self.id.to_be_bytes()[0],
            self.id.to_be_bytes()[1],
            self.flags.to_be_bytes()[0],
            self.flags.to_be_bytes()[1],
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
            flags: ((h[2] as u16) << 8) | h[3] as u16,
            qdcount: ((h[4] as u16) << 8) | h[5] as u16,
            ancount: ((h[6] as u16) << 8) | h[7] as u16,
            nscount: ((h[8] as u16) << 8) | h[9] as u16,
            arcount: ((h[10] as u16) << 8) | h[11] as u16,
        }
    }

    pub fn has_errors(&self) -> bool {
        self.flags & Flags::RCODE_FORMAT_ERROR != 0
            || self.flags & Flags::RCODE_SERVER_FAILURE != 0
            || self.flags & Flags::RCODE_NAME_ERROR != 0
            || self.flags & Flags::RCODE_NOT_IMPLEMENTED != 0
            || self.flags & Flags::RCODE_REFUSED != 0
    }
}

impl Debug for DnsHeader {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let header_formatted = format!(
            "\n\tqr: {:?},\n\topcode: {:?},\n\taa: {:?},\n\ttc: {:?},\n\trd: {:?},\n\tra: {:?},\n\trcode: {:?}",
            if self.flags & Flags::RESPONSE != 0 { "response" } else { "query" },

            if self.flags & Flags::OPCODE_INVERSE_QUERY != 0 { "inverse query" }
            else if self.flags & Flags::OPCODE_SERVER_STATUS_REQUEST != 0 { "server status request" }
            else if self.flags & Flags::OPCODE_RESERVED != 0 { "reserved" }
            else { "standard query" },

            if self.flags & Flags::AUTHORATIVE_ANSWER != 0 { "authoritative" } else { "non-authoritative" },

            if self.flags & Flags::TRUNCATED != 0 { "truncated" } else { "not truncated" },

            if self.flags & Flags::RECURSION_DESIRED != 0 { "recursion desired" } else { "recursion not desired" },

            if self.flags & Flags::RECURSION_AVAILABLE != 0 { "recursion available" } else { "recursion not available" },

            if self.flags & Flags::RCODE_FORMAT_ERROR != 0 { "format error" }
            else if self.flags & Flags::RCODE_SERVER_FAILURE != 0 { "server failure" }
            else if self.flags & Flags::RCODE_NAME_ERROR != 0 { "name error" }
            else if self.flags & Flags::RCODE_NOT_IMPLEMENTED != 0 { "not implemented" }
            else if self.flags & Flags::RCODE_REFUSED != 0 { "refused" }
            else if self.flags & Flags::OPCODE_RESERVED != 0 { "reserved" }
            else { "no error" }
        );
        write!(
            f,
            "id: {:x},\nheader: {},\nqdcount: {:x},\nancount: {:x},\nnscount: {:x},\narcount: {:x}",
            self.id, header_formatted, self.qdcount, self.ancount, self.nscount, self.arcount
        )
    }
}
