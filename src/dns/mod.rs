use crate::*;
use std::fs::File;
use std::io::Read;
use crate::byte_packet::BytePacketBuffer;
use crate::dns::packet::DnsPacket;

mod header;
mod question;
mod resource_record;
mod packet;

pub fn resolve_dns() -> Result<(), Error> {
    println!("testing1");
    let mut f = File::open("response_packet.txt")?;
    let mut buffer = BytePacketBuffer::new();
    f.read(&mut buffer.buf)?;

    println!("testing1.1");
    let packet = DnsPacket::from_buffer(&mut buffer)?;
    println!("testing1.2");
    print_dns(packet);
    Ok(())
}

fn print_dns(packet: DnsPacket) -> () {
    println!("testing print dns");
    println!("{:#?}", packet.Header);

    for q in packet.Question {
        println!("{:#?}", q);
    }
    for answer in packet.Answer {
        println!("{:#?}", answer);
    }for rec in packet.Authority {
        println!("{:#?}", rec);
    }for rec in packet.Additional {
        println!("{:#?}", rec);
    }
}