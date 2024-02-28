#[derive(Debug)]
pub struct Flags {}

#[allow(dead_code)]
impl Flags {
    pub const RESPONSE: u16 = 1 << 15;

    pub const OPCODE_INVERSE_QUERY: u16 = 1 << 11;
    pub const OPCODE_SERVER_STATUS_REQUEST: u16 = 2 << 11;
    pub const OPCODE_RESERVED: u16 = 0b11 << 13;

    pub const AUTHORATIVE_ANSWER: u16 = 1 << 10;

    pub const TRUNCATED: u16 = 1 << 9;

    pub const RECURSION_DESIRED: u16 = 1 << 8;

    pub const RECURSION_AVAILABLE: u16 = 1 << 7;

    pub const RCODE_FORMAT_ERROR: u16 = 1;
    pub const RCODE_SERVER_FAILURE: u16 = 2;
    pub const RCODE_NAME_ERROR: u16 = 3;
    pub const RCODE_NOT_IMPLEMENTED: u16 = 4;
    pub const RCODE_REFUSED: u16 = 5;
    pub const RCODE_RESERVED: u16 = 15;
}
