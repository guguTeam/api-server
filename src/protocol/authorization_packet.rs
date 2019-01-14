use stream::Stream;

pub struct AuthorizationPacket {
    pub password: String
}

impl AuthorizationPacket {}

pub fn encode(b: Vec<u8>) -> AuthorizationPacket {
let s = Stream::new(b);
println!("{}", s.get_index());
    AuthorizationPacket { password: String::new() }
}