use crate::*;
use crate::byte_packet::BytePacketBuffer;

#[derive(Debug)]
pub struct DnsHeader {
    id: u16,
    flags: Flags,
    pub questions: u16,
    pub answers: u16,
    pub authority_records: u16,
    pub additional_records: u16,
}

#[derive(Debug)]
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
impl Flags {
    fn new() -> Flags {
        Flags {
            qr: false,
            opcode: 0,
            aa: false,
            tc: false,
            rd: false,
            ra: false,
            Zero: 0,
            ResCode: ResCode::NOERROR,
        }
    }
}

#[derive(Debug)]
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
    pub fn new() -> DnsHeader {
        DnsHeader {
            id: 0,
            flags: Flags::new(),
            questions: 0,
            answers: 0,
            authority_records: 0,
            additional_records: 0,
        }
    }
    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), Error> {
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