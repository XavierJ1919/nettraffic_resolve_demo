use crate::*;
use crate::byte_packet::BytePacketBuffer;

#[derive(Debug)]
pub struct DnsQuestion {
    qname: String,
    qtype: QueryType,
    qclass: u16,
}
impl DnsQuestion {
    pub fn new(qname: String, qtype: QueryType) -> DnsQuestion {
        DnsQuestion {
            qname,
            qtype,
            qclass: 0x01,
        }
    }
    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), Error> {
        buffer.read_qname(&mut self.qname)?;
        self.qtype = QueryType::from_num(buffer.read_u16()?);
        self.qclass = buffer.read_u16()?;
        Ok(())
    }
}

// todo: txt;
#[derive(Debug)]
pub enum QueryType {
    UNKNOWN(u16),
    A,
    NS,
    MX,
    CNAME,
    AAAA,
    TXT,
}
impl QueryType {
    pub fn from_num(num: u16) -> QueryType {
        match num {
            1 => QueryType::A,
            2 => QueryType::NS,
            5 => QueryType::CNAME,
            15 => QueryType::MX,
            16 => QueryType::TXT,
            28 => QueryType::AAAA,
            _ => QueryType::UNKNOWN(num),
        }
    }
}
