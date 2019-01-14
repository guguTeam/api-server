use crate::stream::Stream;

pub mod authorization_packet;

pub trait Packet {
    fn get_stream(&self) -> &Stream;
    fn encode(&mut self);
    fn decode(&self) -> Vec<u8>;
}
