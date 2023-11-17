use crate::byte_packet::BytePacketBuffer;

pub struct DnsQuestion {
    qname: String,
    qtype: QueryType,
    qclass: u16,
}
impl DnsQuestion {
    pub fn read(&mut self, buffer: &mut BytePacketBuffer) -> Result<(), Err> {
        buffer.read_qname(&mut self.qname)?;
        self.qtype = QueryType::from_num(buffer.read_u16()?);
        self.qclass = 0x01;
        Ok(())
    }
}

pub enum QueryType {
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
