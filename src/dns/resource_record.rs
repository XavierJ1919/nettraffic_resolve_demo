use std::net::Ipv4Addr;
use crate::byte_packet::BytePacketBuffer;
use crate::dns::question::{DnsQuestion, QueryType};

// resource record
pub struct RRecord {
    name: String,
    record_type: QueryType,
    class: u16,
    ttl: u32,
    rdlength: u16,
    rdata: RespData,
}
impl RRecord {
    fn read (buffer: &mut BytePacketBuffer) -> Result<RRecord, Err> {
        let mut domain = String::new();
        buffer.read_qname(&mut domain)?;

        let qtype_num = buffer.read_u16()?;
        let record_type = QueryType::from_num(qtype_num)?;
        let class = buffer.read_u16()?;
        let ttl = ((buffer.read_u16()? as u32) << 16) | (buffer.read_u16()? as u32);
        let rdlength = buffer.read_u16()?;
        let rdata = match record_type {
            QueryType::A => {
                let raw_addr = buffer.read_u32()?;
                RespData::A {
                    ipAddr: Ipv4Addr::new(
                    ((raw_addr >> 24) & 0xFF) as u8,
                    ((raw_addr >> 16) & 0xFF) as u8,
                    ((raw_addr >> 8) & 0xFF) as u8,
                    ((raw_addr >> 0) & 0xFF) as u8,
                    )
                }
            },
            QueryType::CNAME => {
                let mut name = String::new();
                buffer.read_qname(&mut name)?;
                RespData::CNAME {
                    name
                }
            },
            QueryType::NS => {
                let mut name = String::new();
                buffer.read_qname(&mut name)?;
                RespData::NS {
                    name
                }
            },
            QueryType::MX => {
                let preference = buffer.read_u16()?;
                let mut exchange = String::new();
                buffer.read_qname(&mut exchange)?;
                RespData::MX {
                    preference,
                    exchange
                }
            },
        };
        let res = RRecord {
            name: domain,
            record_type,
            class,
            ttl,
            rdlength,
            rdata
        };
        Ok(res)
    }
}

enum RespData {
    A {
        ipAddr: Ipv4Addr,
    },
    CNAME {
        name: String,
    },
    MX {
        preference: u16,
        exchange: String,
    },
    NS {
        name: String,
    },
}
