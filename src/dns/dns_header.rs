use crate::*;
use crate::byte_packet::BytePacketBuffer;

// type Error = Box<dyn std::error::Error>;
// type Result<T> = std::result::Result<T, Error>;

struct DnsPacket {
    DnsHeader: DnsHeader,
    Question: DnsQuestion,
    Answer: DnsAnswer,
    Authority: ,
    Additional,
}
struct DnsHeader {
    id: u16,
    flags: Flags,
    questions: u16,
    answers: u16,
    authority_records: u16,
    additional_records: u16,
}

struct DnsQuestion {
    qname: String,
    qtype: QueryType,
    qclass: u16,
}
impl DnsQuestion {
    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), Err> {
        self.qname = buffer.read_qname()?;
        self.qtype = QueryType::from_num(buffer.read_u16()?);
        self.qclass = 0x01;
        Ok(())
    }
}

enum QueryType {
    UNKNOWN(u16),
    A,
    NS,
    MX,
    CNAME,
    AAAA,
}
impl QueryType {
    pub fn from_num(num: u16) -> QueryType {
        match num {
            1 => QueryType::A,
            2 => QueryType::NS,
            5 => QueryType::CNAME,
            15 => QueryType::MX,
            28 => QueryType::AAAA,
            _ => QueryType::UNKNOWN(num),
        }
    }
}

struct Flags {
    qr: bool,
    opcode: u8,
    aa: bool,
    tc: bool,
    rd: bool,
    ra: bool,
    Zero: u8,
    ResCode: ResCode,
}

enum ResCode {
    NOERROR = 0,
    FORMAT_ERROR = 1,
    SERVER_FAILURE = 2,
    NAME_ERROR = 3,
    NOT_IMPLEMENTED = 4,
    REFUSED = 5,
}
impl ResCode {
    pub fn from_num(num: u16) -> ResCode {
        match num {
            1 => ResCode::FORMAT_ERROR,
            2 => ResCode::SERVER_FAILURE,
            3 => ResCode::NAME_ERROR,
            4 => ResCode::NOT_IMPLEMENTED,
            5 => ResCode::REFUSED,
            _ => ResCode::NOERROR,
        }
    }
}

impl DnsHeader {
    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), Err> {
        self.id = buffer.read_u16()?;
        let flags = buffer.read_u16()?;
        self.flags.ResCode = ResCode::from_num(flags & 0x000F);
        self.flags.Zero = ((flags & (0b111 << 4)) >> 4) as u8;
        self.flags.ra = (flags & (0b1 << 7)) > 0;
        self.flags.rd = (flags & (0b1 << 8)) > 0;
        self.flags.tc = (flags & (0b1 << 9)) > 0;
        self.flags.aa = (flags & (0b1 << 10)) > 0;
        self.flags.opcode = ((flags & (0b1111 << 11)) >> 11) as u8;
        self.flags.qr = (flags & (0b1 << 15)) > 0;

        self.questions = buffer.read_u16()?;
        self.answers = buffer.read_u16()?;
        self.authority_records = buffer.read_u16()?;
        self.additional_records = buffer.read_u16()?;
        Ok(())
    }
}