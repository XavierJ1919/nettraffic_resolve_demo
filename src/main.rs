type Error = Box<dyn std::error::Error>;

mod byte_packet;
mod dns;

fn main() {
    dns::resolve_dns();
}
