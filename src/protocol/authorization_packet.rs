use crate::stream::{Stream};
use crate::protocol::Packet;

const LEN: usize = 16usize;

pub struct AuthorizationPacket {
    pub password: String,
    stream: Stream
}
/*
len: 16
data:
0-15 string 密码
*/

impl Packet for AuthorizationPacket {
    fn get_stream(&self) -> &Stream {
        &self.stream
    }

    fn encode(&mut self) {
        self.stream.check_len(LEN);
        //println!("{:?}", s.read_u8_array(16));
        self.password = self.stream.read_str(16);
    }

    fn decode(&self) -> Vec<u8> {
        Vec::new()
    }
}

pub fn new(b: Vec<u8>) -> AuthorizationPacket {
    AuthorizationPacket{password: String::new(), stream: Stream::new(b)}
}
