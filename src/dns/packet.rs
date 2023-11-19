use crate::byte_packet::BytePacketBuffer;
use crate::dns::header::*;
use crate::Error;
use super::question::*;
use super::resource_record::*;

#[derive(Debug)]
pub struct DnsPacket {
    pub Header: DnsHeader,
    pub Question: Vec<DnsQuestion>,
    pub Answer: Vec<RRecord>,
    pub Authority: Vec<RRecord>,
    pub Additional: Vec<RRecord>,
}

impl DnsPacket {
    fn new() -> DnsPacket {
        DnsPacket {
            Header: DnsHeader::new(),
            Question: Vec::new(),
            Answer: Vec::new(),
            Authority: Vec::new(),
            Additional: Vec::new(),
        }
    }
    pub fn from_buffer(buffer: &mut BytePacketBuffer) -> Result<DnsPacket, Error> {
        let mut res = DnsPacket::new();
        res.Header.read(buffer)?;

        for _ in 0..res.Header.questions {
            let mut question = DnsQuestion::new("".to_string(), QueryType::UNKNOWN(0));
            question.read(buffer)?;
            res.Question.push(question);
        }
        for _ in 0..res.Header.answers {
            let record = RRecord::read(buffer)?;
            res.Answer.push(record);
        }
        for _ in 0..res.Header.authority_records {
            let record = RRecord::read(buffer)?;
            res.Authority.push(record);
        }
        for _ in 0..res.Header.additional_records {
            let record = RRecord::read(buffer)?;
            res.Additional.push(record);
        }
        Ok(res)
    }
}