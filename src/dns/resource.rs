#[derive(Debug)]
pub enum RecordType {
    A = 1,
    AAAA = 28,
}

#[derive(Debug)]
pub enum ClassCode {
    IN = 1,
    CS = 2,
    CH = 3,
    HS = 4,
}

#[derive(Debug)]
pub struct ResourceRecord {
    pub name: String,
    pub r#type: RecordType,
    pub class: ClassCode,
    pub ttl: u32,
    pub rdlength: u16,
    pub rdata: Vec<u8>,
}

impl ResourceRecord {
    pub fn from_u8_array(buf: Vec<u8>) -> Result<ResourceRecord, Box<dyn std::error::Error>> {
        let name = String::from("TODO"); // not dealing with dns message compression yet
        let r#type = match (buf[2] as u16) << 8 | buf[3] as u16 {
            1 => RecordType::A,
            28 => RecordType::AAAA,
            _ => panic!(
                "Failed to parse record type: {:?}",
                (buf[2] as u16) << 8 | buf[3] as u16
            ),
        }; // will support more RRs as needed
        let class = match (buf[4] as u16) << 8 | buf[5] as u16 {
            1 => ClassCode::IN,
            2 => ClassCode::CS,
            3 => ClassCode::CH,
            4 => ClassCode::HS,
            _ => panic!("Unknown class code"),
        };
        let ttl = u32::from_be_bytes([buf[6], buf[7], buf[8], buf[9]]);
        let rdlength = u16::from_be_bytes([buf[10], buf[11]]);
        let rdata = buf[12..rdlength as usize + 12].to_vec();

        Ok(ResourceRecord {
            name,
            r#type,
            class,
            ttl,
            rdlength,
            rdata,
        })
    }
}
